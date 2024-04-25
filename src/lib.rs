#![no_std]

use asr::{
    future::{next_tick, retry},
    signature::Signature,
    time::Duration,
    timer::{self},
    watcher::Watcher,
    Process,
};

const SAR_TIMER_SIGNATURE: Signature<42> = Signature::new("53 41 52 5F 54 49 4D 45 52 5F 53 54 41 52 54 00 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 53 41 52 5F 54 49 4D 45 52 5F 45 4E 44 00");
const SAR_TIMER_OFFSET: u64 = 16;

const _TIMER_ACTION_NONE: i32 = 0;
const TIMER_ACTION_START: i32 = 1;
const TIMER_ACTION_RESTART: i32 = 2;
const TIMER_ACTION_SPLIT: i32 = 3;
const TIMER_ACTION_END: i32 = 4;
const TIMER_ACTION_RESET: i32 = 5;

asr::panic_handler!();

#[cfg(not(feature = "nightly"))]
asr::async_main!(stable);
#[cfg(feature = "nightly")]
asr::async_main!(nightly);

async fn main() {
    loop {
        asr::set_tick_rate(1.0);

        timer::pause_game_time();

        let process =
            retry(|| Process::attach("portal2.exe").or_else(|| Process::attach("portal2_linux")))
                .await;

        process
            .until_closes(async {
                let mut interface_address: Option<asr::Address>;

                loop {
                    if process.get_module_address("sar.dll").is_ok()
                        || process.get_module_address("sar.so").is_ok()
                    {
                        interface_address = process
                            .memory_ranges()
                            .filter(|m| {
                                m.flags()
                                    .unwrap_or_default()
                                    .contains(asr::MemoryRangeFlags::WRITE)
                            })
                            .find_map(|m| {
                                m.range().map_or(None, |range| {
                                    SAR_TIMER_SIGNATURE.scan_process_range(&process, range)
                                })
                            })
                            .and_then(|x| Some(x.add(SAR_TIMER_OFFSET)));

                        if interface_address.is_some() {
                            break;
                        }
                    }

                    next_tick().await;
                }

                let total_address = interface_address.unwrap();
                let ipt_address = total_address.clone().add(4);
                let action_address = total_address.clone().add(8);

                asr::print_limited::<40>(&format_args!(
                    "Found SAR + pubInterface at 0x{:X}",
                    total_address.value()
                ));

                let mut total = Watcher::<i32>::new();
                let mut ipt = Watcher::<f32>::new();
                let mut action = Watcher::<i32>::new();

                asr::set_tick_rate(120.0);

                loop {
                    let Some(total) = total.update(process.read(total_address).ok()) else {
                        next_tick().await;
                        continue;
                    };
                    let Some(ipt) = ipt.update(process.read(ipt_address).ok()) else {
                        next_tick().await;
                        continue;
                    };
                    let Some(action) = action.update(process.read(action_address).ok()) else {
                        next_tick().await;
                        continue;
                    };

                    timer::set_game_time(Duration::saturating_seconds_f32(
                        total.current as f32 * ipt.current,
                    ));

                    if action.changed() {
                        match action.current {
                            TIMER_ACTION_START => {
                                timer::start();
                            }
                            TIMER_ACTION_RESTART => {
                                timer::reset();
                                timer::pause_game_time();
                                timer::start();
                            }
                            TIMER_ACTION_SPLIT | TIMER_ACTION_END => {
                                timer::split();
                            }
                            TIMER_ACTION_RESET => {
                                timer::reset();
                                timer::pause_game_time();
                            }
                            _ => (),
                        }
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}
