#![no_std]

mod auto_splitter;
mod auto_splitter_settings;
mod auto_splitter_structs;
mod debug;
pub mod state;

use crate::auto_splitter::*;
use crate::state::*;
use asr::future::retry;
use asr::{future::next_tick, print_limited, Process};
use asr::settings::Gui;

//use crate::debug::*;

asr::async_main!(stable);
asr::panic_handler!();

async fn main() {
    let mut settings = auto_splitter_settings::Settings::register();

    let mut custom_vars = auto_splitter_startup();

    loop {
        let process = retry(|| {
            ["sonic.exe", "Sonic Adventure DX.exe", "wine", "wine64"]
                .into_iter()
                .find_map(Process::attach)
        })
        .await;

        process
            .until_closes(async {
                auto_splitter_init(&settings);

                let mut game_state = GameState::default();

                loop {
                    settings.update();
                    
                    macro_rules! unwrap_or_next_tick_opt {
                        ( $e:expr, $s:expr ) => {
                            match $e {
                                Some(x) => x,
                                _ => {
                                    print_limited::<128>($s);
                                    next_tick().await;
                                    continue;
                                }
                            }
                        };
                    }

                    macro_rules! unwrap_or_next_tick_res {
                        ( $e:expr, $s:expr ) => {
                            match $e {
                                Ok(x) => x,
                                _ => {
                                    print_limited::<128>($s);
                                    next_tick().await;
                                    continue;
                                }
                            }
                        };
                    }

                    macro_rules! read_mem {
                        ( $name:ident, $addr:expr, $t:ty) => {
                            let $name = *unwrap_or_next_tick_opt!(
                                game_state.$name.update(Some(unwrap_or_next_tick_res!(
                                    process.read::<$t>($addr),
                                    &format_args!("Failed reading {}", stringify!($name))
                                ))),
                                &format_args!("Failed updating {}", stringify!($name))
                            );
                        };
                    }

                    macro_rules! read_mem_and_map {
                        ( $name:ident, $addr:expr, $t:ty, $mapper:expr) => {
                            let $name = *unwrap_or_next_tick_opt!(
                                game_state
                                    .$name
                                    .update(Some($mapper(unwrap_or_next_tick_res!(
                                        process.read::<$t>($addr),
                                        &format_args!("Failed reading {}", stringify!($name))
                                    )))),
                                &format_args!("Failed updating {}", stringify!($name))
                            );
                        };
                    }

                    read_mem!(global_frame_count, GLOBAL_FRAME_COUNT_ADDRESS, i32);
                    read_mem!(output_frame_count, OUTPUT_FRAME_COUNT_ADDRESS, i32);
                    read_mem_and_map!(in_cutscene, IN_CUTSCENE_ADDRESS, i32, |value| {
                        value != 0
                    });
                    read_mem!(game_status, GAME_STATUS_ADDRESS, u8);
                    read_mem!(game_mode, GAME_MODE_ADDRESS, u8);
                    read_mem_and_map!(
                        in_prerendered_movie,
                        IN_PRERENDERED_MOVIE_ADDRESS,
                        i32,
                        |value| { value != 0 }
                    );
                    read_mem!(current_character, CURRENT_CHARACTER_ADDRESS, u8);
                    read_mem!(last_story, LAST_STORY_ADDRESS, u8);
                    read_mem!(timer_start, TIMER_START_ADDRESS, u8);
                    read_mem!(demo_playing, DEMO_PLAYING_ADDRESS, u8);
                    read_mem!(boss_health, BOSS_HEALTH_ADDRESS, u8);
                    read_mem!(level, LEVEL_ADDRESS, u8);
                    read_mem!(level_copy, LEVEL_COPY_ADDRESS, u8);
                    read_mem!(act, ACT_ADDRESS, u8);
                    read_mem!(lives, LIVES_ADDRESS, u8);
                    read_mem!(e101_flag, E101_FLAG_ADDRESS, u8);
                    read_mem!(adventure_data, ADVENTURE_DATA_ADDRESS, u8);
                    read_mem!(rm_flag, RM_FLAG_ADDRESS, u8);
                    read_mem!(mk2_value, MK2_VALUE_ADDRESS, u8);
                    read_mem!(walker_value, WALKER_VALUE_ADDRESS, u8);

                    read_mem!(angel_cutscene, ANGEL_CUTSCENE_ADDRESS, u8);
                    read_mem!(selected_character, SELECTED_CHARACTER_ADDRESS, u8);
                    read_mem!(music, MUSIC_ADDRESS, u8);
                    read_mem!(lw_flag, LW_FLAG_ADDRESS, u8);

                    read_mem!(emblem_bytes, EMBLEM_BYTES_ADDRESS, [u8; 17]);
                    read_mem!(event_bytes, EVENT_BYTES_ADDRESS, [u8; 353]);

                    read_mem!(light_shoes, LIGHT_SHOES_ADDRESS, u8);
                    read_mem!(crystal_ring, CRYSTAL_RING_ADDRESS, u8);
                    read_mem!(ancient_light, ANCIENT_LIGHT_ADDRESS, u8);
                    read_mem!(jet_anklet, JET_ANKLET_ADDRESS, u8);
                    read_mem!(rhythm_badge, RHYTHM_BADGE_ADDRESS, u8);
                    read_mem!(shovel_claw, SHOVEL_CLAW_ADDRESS, u8);
                    read_mem!(fighting_gloves, FIGHTING_GLOVES_ADDRESS, u8);
                    read_mem!(warrior_feather, WARRIOR_FEATHER_ADDRESS, u8);
                    read_mem!(long_hammer, LONG_HAMMER_ADDRESS, u8);
                    read_mem!(life_belt, LIFE_BELT_ADDRESS, u8);
                    read_mem!(power_rod, POWER_ROD_ADDRESS, u8);
                    read_mem!(lure_ec, LURE_EC_ADDRESS, u8);
                    read_mem!(lure_ic, LURE_IC_ADDRESS, u8);
                    read_mem!(lure_ss, LURE_SS_ADDRESS, u8);
                    read_mem!(lure_mr, LURE_MR_ADDRESS, u8);
                    read_mem!(jet_booster, JET_BOOSTER_ADDRESS, u8);
                    read_mem!(laser_blaster, LASER_BLASTER_ADDRESS, u8);

                    let vars = GameStatePair {
                        global_frame_count, output_frame_count, in_cutscene, game_status,
                        game_mode, in_prerendered_movie, current_character, last_story,
                        timer_start, demo_playing, boss_health, level, level_copy, act, lives,
                        e101_flag, adventure_data, rm_flag, mk2_value, walker_value, angel_cutscene,
                        selected_character, music, lw_flag, emblem_bytes, event_bytes,
                        light_shoes, crystal_ring, ancient_light, jet_anklet, rhythm_badge,
                        shovel_claw, fighting_gloves, warrior_feather, long_hammer, life_belt,
                        power_rod, lure_ec, lure_ic, lure_ss, lure_mr, jet_booster, laser_blaster,
                    };

                    auto_splitter_loop(&vars, &mut custom_vars, &settings);
                    //print_game_state(&vars);

                    next_tick().await;
                }
            })
            .await;
    }
}
