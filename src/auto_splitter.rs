use super::state::GameStatePair;
use crate::auto_splitter_settings::Settings;
use crate::auto_splitter_structs::{BossSplits, IgnoredStages};
use asr::{print_message, time::Duration, time_util::Instant, timer, timer::TimerState};
use core::ops::Add;

pub struct CustomVars {
    total_frame_count: i32,
    total_cutscene_rta: Duration,
    previous_rta: Duration,
    credits_counter: i32,
    boss_started: bool,
    delay: i32,
    split_mask: [bool; 353],
    boss_splits: BossSplits,
    ignored_stages: IgnoredStages,
    run_start_time: Option<Instant>,
}

pub fn auto_splitter_startup() -> CustomVars {
    print_message("Sonic Adventure DX Auto Splitter - Loaded, waiting for game process");

    asr::set_tick_rate(63.0);

    CustomVars {
        total_frame_count: 0,
        total_cutscene_rta: Duration::new(0, 0),
        previous_rta: Duration::new(0, 0),
        credits_counter: 0,
        boss_started: false,
        delay: 0,
        split_mask: core::array::from_fn(|_| false),
        boss_splits: BossSplits::new(),
        ignored_stages: IgnoredStages::new(),
        run_start_time: None,
    }
}

pub fn auto_splitter_init(_settings: &Settings) {
    print_message(
        "Sonic Adventure DX Auto Splitter - Attached to process, beginning main auto splitter loop",
    )
}

pub fn auto_splitter_start(
    vars: &GameStatePair,
    custom_vars: &mut CustomVars,
    settings: &Settings,
) -> bool {
    custom_vars.total_frame_count = 0;
    custom_vars.total_cutscene_rta = Duration::new(0, 0);
    custom_vars.previous_rta = Duration::new(0, 0);
    custom_vars.credits_counter = -1;
    custom_vars.delay = 0;

    custom_vars.boss_started = false;
    custom_vars.split_mask = core::array::from_fn(|_| false);
    custom_vars.boss_splits.reset();
    custom_vars.ignored_stages.reset();

    if settings.boss_18 {
        custom_vars.boss_splits.set_boss_18();
    }
    if settings.boss_22 {
        custom_vars.boss_splits.set_boss_22();
    }

    if settings.stage_36 {
        custom_vars.ignored_stages.set_stage_36();
    }
    if settings.stage_37 {
        custom_vars.ignored_stages.set_stage_37();
    }

    macro_rules! set_split_mask_if_setting {
        ( $setting:ident, $number:expr ) => {
            if settings.$setting {
                custom_vars.split_mask[$number] = true
            }
        };
    }

    // Sonic bosses splits
    set_split_mask_if_setting!(gamma_sonic, 36);
    set_split_mask_if_setting!(knuckles_sonic, 37);
    set_split_mask_if_setting!(zero_sonic, 48);
    set_split_mask_if_setting!(four_sonic, 49);
    set_split_mask_if_setting!(six_sonic, 50);
    set_split_mask_if_setting!(egg_hornet_sonic, 51);

    // Tails bosses splits
    set_split_mask_if_setting!(gamma_tails, 95);
    set_split_mask_if_setting!(knuckles_tails, 96);
    set_split_mask_if_setting!(four_tails, 103);
    set_split_mask_if_setting!(egg_hornet_tails, 105);

    // Knuckles bosses splits
    set_split_mask_if_setting!(sonic_knuckles, 158);
    set_split_mask_if_setting!(two_knuckles, 163);
    set_split_mask_if_setting!(four_knuckles, 165);

    // Gamma bosses splits
    set_split_mask_if_setting!(sonic_gamma, 282);

    // Start on fade-to-black on story screen
    let story_enter_start = vars.demo_playing.current != 1
        && vars.game_status.old == 21
        && (vars.game_mode.current != 12 && vars.game_mode.current != 20)
        && (vars.game_mode.old == 12 || vars.game_mode.old == 20);

    if settings.char_start_ban_trial {
        return story_enter_start && vars.game_mode.current != 9;
    }

    return story_enter_start;
}

pub fn auto_splitter_split(
    vars: &GameStatePair,
    custom_vars: &mut CustomVars,
    settings: &Settings,
) -> bool {
    macro_rules! split_if_true {
        ( $e:expr ) => {
            if $e {
                return true;
            }
        };
    }

    // Split when entering a story
    if settings.char_split
        && vars.demo_playing.current != 1
        && vars.game_status.old == 21
        && (vars.game_mode.current != 12 && vars.game_mode.current != 20)
        && (vars.game_mode.old == 12 || vars.game_mode.old == 20)
    {
        if settings.char_split_ban_trial {
            split_if_true!(vars.game_mode.current != 9)
        } else {
            return true;
        }
    }

    // Ignored stages
    if custom_vars.ignored_stages[vars.level.current] {
        return false;
    }

    // Event split code
    for i in 0..custom_vars.split_mask.len() {
        if custom_vars.split_mask[i]
            && vars.game_status.current != 21
            && (vars.event_bytes.current[i] != vars.event_bytes.old[i])
        {
            print_message("Event Split");
            return true;
        }
    }

    // Big ending
    split_if_true!(
        vars.in_cutscene.current != false
            && vars.level.current == 29
            && vars.current_character.current == 7
    );

    // E-101 split
    split_if_true!(
        settings.e101_gamma && vars.current_character.current == 6 && vars.e101_flag.increased()
    );

    // MKII split
    split_if_true!(
        vars.level.current == 25
            && vars.mk2_value.current == 255
            && settings.boss_25
            && vars.boss_health.current == 0
            && vars.timer_start.changed_from_to(&1, &0)
    );

    // Perfect Chaos split
    split_if_true!(
        vars.level.current == 19 && settings.boss_19 && vars.boss_health.changed_from_to(&2, &0)
    );

    // Zero split
    split_if_true!(
        vars.level.current == 23 && settings.boss_23 && vars.boss_health.changed_from_to(&1, &0)
    );

    // Egg Walker split
    split_if_true!(
        vars.level.current == 21
            && vars.walker_value.current == 9
            && settings.boss_21
            && vars.boss_health.current == 0
            && vars.timer_start.changed_from_to(&1, &0)
    );

    // Boss Split timing
    if custom_vars.boss_splits[vars.level.current] {
        // Prevents split at beginning of boss
        if vars.boss_health.current == 0 && vars.timer_start.current == 0 {
            custom_vars.boss_started = true;
        }

        // Boss beaten checks
        if custom_vars.boss_started
            && vars.in_cutscene.current == false
            && vars.in_cutscene.unchanged()
            && vars.boss_health.current == 0
            && vars.game_status.current == 15
            && vars.timer_start.changed_from_to(&1, &0)
        {
            return !(vars.level.current == 18 && vars.current_character.current == 0);
        }
    }

    // Upgrades split
    if vars.level.current != 0 {
        split_if_true!(settings.lss_upgrade_sonic && vars.light_shoes.increased());
        split_if_true!(settings.cr_upgrade_sonic && vars.crystal_ring.increased());
        split_if_true!(settings.al_upgrade_sonic && vars.ancient_light.increased());
        split_if_true!(settings.ja_upgrade_tails && vars.jet_anklet.increased());
        split_if_true!(settings.rb_upgrade_tails && vars.rhythm_badge.increased());
        split_if_true!(settings.sc_upgrade_knuckles && vars.shovel_claw.increased());
        split_if_true!(settings.fg_upgrade_knuckles && vars.fighting_gloves.increased());
        split_if_true!(settings.wf_upgrade_amy && vars.warrior_feather.increased());
        split_if_true!(settings.lh_upgrade_amy && vars.long_hammer.increased());
        split_if_true!(settings.pr_upgrade_big && vars.power_rod.increased());
        split_if_true!(settings.lb_upgrade_big && vars.life_belt.increased());
        split_if_true!(
            settings.l_upgrade_big
                && (vars.lure_ss.increased()
                    || vars.lure_ic.changed()
                    || vars.lure_ec.changed()
                    || vars.lure_mr.changed())
        );
        split_if_true!(settings.jb_upgrade_gamma && vars.jet_booster.increased());
        split_if_true!(settings.lb_upgrade_gamma && vars.laser_blaster.increased());
    }

    // Enter bosses splits
    split_if_true!(settings.enter_egg_hornet && vars.level.changed_from_to(&33, &20));
    split_if_true!(settings.enter_chaos_2 && vars.level.changed_from_to(&26, &16));
    split_if_true!(settings.enter_chaos_4 && vars.level.changed_from_to(&33, &17));
    split_if_true!(settings.enter_chaos_6 && vars.level.changed_from_to(&29, &18));
    split_if_true!(settings.enter_beta_1 && vars.level.changed_from_to(&33, &24));
    split_if_true!(settings.enter_beta_2 && vars.level.changed_from_to(&29, &25));

    // Enter level splits
    split_if_true!(settings.enter_ec && vars.level.changed_from_to(&26, &1));
    split_if_true!(settings.enter_wv && vars.level.changed_from_to(&33, &2));
    split_if_true!(settings.enter_c && vars.level.changed_from_to(&26, &9));
    split_if_true!(settings.enter_ic && vars.level.changed_from_to(&33, &8));
    split_if_true!(settings.enter_tp && vars.level.changed_from_to(&26, &3));
    split_if_true!(settings.enter_sh && vars.level.changed_from_to(&26, &4));
    split_if_true!(settings.enter_rm && vars.level.changed_from_to(&33, &5));
    split_if_true!(settings.enter_sd && vars.level.changed_from_to(&29, &6));
    split_if_true!(settings.enter_lw && vars.level.changed_from_to(&33, &7));
    split_if_true!(settings.enter_fe && vars.level.changed_from_to(&33, &10));
    split_if_true!(settings.enter_hs && vars.level.changed_from_to(&32, &12));

    // Miscellaneous splits

    // Global miscellaneous
    if settings.act_split
        && vars.level.current != 0
        && vars.level.current != 15
        && vars.level.current != 22
        && vars.level.current != 26
        && vars.level.current != 29
        && vars.level.current != 32
        && vars.level.current != 33
        && vars.level.current != 37
    {
        split_if_true!(vars.act.changed())
    }

    let cur_b = &vars.emblem_bytes.current;
    let old_b = &vars.emblem_bytes.old;

    // Sonic miscellaneous
    if vars.level.current == 26 && vars.act.changed_from_to(&0, &2) {
        split_if_true!(settings.enter_sewer_sonic && vars.current_character.current == 0);
    }
    if vars.level.current == 26 && ((cur_b[8] & 0x8) >> 3) == 1 && ((cur_b[8] & 0x10) >> 4) == 0 {
        if vars.act.current == 1 && (vars.act.old == 3 || vars.act.old == 4) {
            split_if_true!(settings.enter_casino_hub_sonic && vars.current_character.current == 0);
        }
    }
    split_if_true!(
        settings.ship_transform && vars.level.current == 29 && vars.act.changed_from_to(&3, &6)
    );
    split_if_true!(
        settings.back_from_past
            && vars.level.changed_from_to(&34, &33)
            && vars.act.current == 2
            && vars.act.unchanged()
    );
    split_if_true!(
        settings.enter_ss
            && vars.level.changed_from_to(&33, &26)
            && vars.act.changed_from_to(&0, &1)
    );
    split_if_true!(
        settings.enter_mr
            && vars.level.changed_from_to(&26, &33)
            && vars.act.changed_from_to(&1, &0)
    );

    // Gamma miscellaneous
    if vars.level.current == 33 && vars.act.current == 0 {
        split_if_true!(
            settings.death_wv_gamma
                && vars.adventure_data.changed()
                && vars.current_character.current == 6
        );
    }
    if vars.level.current == 33 && vars.act.current == 1 {
        split_if_true!(
            settings.death_mr_gamma
                && vars.lives.decreased()
                && vars.current_character.current == 6
                && vars.adventure_data.current == 2
        );
    }
    if vars.level.changed_from_to(&33, &29) && vars.rm_flag.current == 1 {
        split_if_true!(settings.egg_carrier_mk2_gamma && vars.current_character.current == 6);
    }

    // Knuckles miscellaneous
    if vars.level.current == 33 && vars.act.current == 1 {
        split_if_true!(
            settings.death_mr_knuckles
                && vars.lw_flag.current == 1
                && vars.lives.decreased()
                && vars.current_character.current == 3
        );
    }
    if vars.level.changed_from_to(&33, &29) {
        split_if_true!(settings.enter_egg_carrier_knuckles && vars.current_character.current == 3);
    }

    // Amy miscellaneous
    if vars.level.changed_from_to(&33, &29) {
        split_if_true!(settings.enter_egg_carrier_amy && vars.current_character.current == 5);
    }
    if ((cur_b[0xA] & 0x10) >> 4) == 0
        && vars.level.current == 26
        && vars.act.changed_from_to(&3, &1)
    {
        split_if_true!(settings.enter_casino_hub_amy && vars.current_character.current == 5);
    }
    if vars.level.current == 33 && vars.act.changed_from_to(&0, &2) {
        split_if_true!(settings.enter_jungle_to_fe_amy && vars.current_character.current == 5);
    }
    if vars.level.current == 33 && vars.act.changed_from_to(&2, &0) {
        split_if_true!(settings.exit_jungle_to_ec_amy && vars.current_character.current == 5);
    }

    // Super Sonic miscellaneous
    if vars.selected_character.current == 6 {
        split_if_true!(
            settings.enter_cave_super_sonic
                && vars.level.current == 33
                && vars.act.changed_from_to(&0, &1)
        );
        split_if_true!(
            settings.angel_island_super_sonic
                && vars.level.current == 33
                && vars.act.current == 1
                && vars.music.changed_to(&40)
        );
        split_if_true!(settings.tikal_super_sonic && vars.level.changed_from_to(&34, &33));
        split_if_true!(
            settings.death_super_sonic
                && vars.level.current == 33
                && vars.act.current == 1
                && vars.lives.decreased()
        );
        split_if_true!(
            settings.jungle_super_sonic
                && vars.level.current == 33
                && vars.level.old != 0
                && vars.act.changed_from_to(&0, &2)
        );
    }

    for i in 0..cur_b.len() {
        // Do not check during menus
        if vars.game_mode.current != 12 && (cur_b[i] != old_b[i]) {
            if vars.level.current < 15 {
                // The action stages
                print_message("Going to split soon");
                custom_vars.delay = vars.global_frame_count.current + 190;
            } else {
                print_message("Field Emblem Split");
                return true;
            }
        }
    }

    if ((custom_vars.delay - vars.global_frame_count.current) as i64).abs() <= 2
        && custom_vars.delay > 0
    {
        custom_vars.delay = 0;
        print_message("Action Emblem Split");
        return true;
    }

    false
}

pub fn auto_splitter_update(
    vars: &GameStatePair,
    custom_vars: &mut CustomVars,
    _settings: &Settings,
) {
    let raw_rta = custom_vars
        .run_start_time
        .map(|value| Instant::duration_since(&Instant::now(), value));
    let mut current_rta = Duration::new(0, 0);

    if let Some(raw_rta) = raw_rta {
        current_rta = current_rta.add(raw_rta)
    }

    // Add RTA to the IGT when the frame counter does not increase
    if vars.in_prerendered_movie.current {
        custom_vars.total_cutscene_rta = custom_vars
            .total_cutscene_rta
            .add(current_rta - custom_vars.previous_rta);
    }

    if vars.game_mode.current == 1
        || vars.game_mode.current == 12
        || vars.game_mode.current == 18
        || vars.game_mode.current == 20
        || vars.game_status.current == 19
        || vars.game_status.current == 16
    {
        custom_vars.total_frame_count +=
            vars.output_frame_count.current - vars.output_frame_count.old;
    }

    if vars.game_mode.current == 22 {
        if vars.game_mode.old != 22 {
            // Arbitrary delay before frames are added during credits, so that categories that skip the credits don't get the frames added
            custom_vars.credits_counter = 3600;
        }

        custom_vars.credits_counter -= 1;

        if custom_vars.credits_counter == 0 {
            custom_vars.total_frame_count += match vars.current_character.current {
                0 => {
                    if vars.last_story.current == 0 {
                        16804
                    } else {
                        17360
                    }
                }
                2 => 14657,
                3 => 16804,
                5 => 18950,
                6 => 17545,
                7 => 15333,
                _ => 0,
            }
        }
    }

    let mut delta_frames = vars.global_frame_count.current - vars.global_frame_count.old;

    // Add twice the amount of time when in a cutscene
    if vars.in_cutscene.current {
        delta_frames *= 2;
    }

    custom_vars.total_frame_count += delta_frames;
    custom_vars.previous_rta = Duration::from(current_rta);
}

pub fn auto_splitter_game_time(
    _vars: &GameStatePair,
    custom_vars: &mut CustomVars,
    _settings: &Settings,
) -> Duration {
    Duration::seconds_f32(custom_vars.total_frame_count as f32 / 60.0)
        + custom_vars.total_cutscene_rta
}

pub fn auto_splitter_is_loading(
    _vars: &GameStatePair,
    _custom_vars: &mut CustomVars,
    _settings: &Settings,
) -> bool {
    true
}

pub fn auto_splitter_reset(
    vars: &GameStatePair,
    _custom_vars: &mut CustomVars,
    settings: &Settings,
) -> bool {
    if settings.reset {
        return vars.game_mode.current == 12
            && vars.current_character.old != 6
            && vars.level_copy.current != 32;
    }

    return false;
}

// Taken from the Sonic Colors Ultimate autosplitter by Jujstme
// https://github.com/SonicSpeedrunning/LiveSplit.SonicColorsUltimate/blob/831198961735fc204431c0365c4a7884456a108a/src/lib.rs#L649
pub fn auto_splitter_loop(vars: &GameStatePair, custom_vars: &mut CustomVars, settings: &Settings) {
    // Splitting logic. Adapted from OG LiveSplit:
    // Order of execution
    // 1. update() will always be run first. There are no conditions on the execution of this action.
    auto_splitter_update(&vars, custom_vars, &settings);

    // 2. If the timer is currently either running or paused,
    if timer::state() == TimerState::Running || timer::state() == TimerState::Paused {
        // 2.1 then the isLoading and reset actions will be run
        if auto_splitter_is_loading(&vars, custom_vars, &settings) {
            timer::pause_game_time()
        } else {
            timer::resume_game_time();
        }

        timer::set_game_time(auto_splitter_game_time(&vars, custom_vars, &settings));

        if auto_splitter_reset(&vars, custom_vars, &settings) {
            timer::reset();
            custom_vars.run_start_time = None
        }
        // 3. If reset does not return true, then the split action will be run.
        else if auto_splitter_split(&vars, custom_vars, &settings) {
            timer::split()
        }
    }

    // 4. If the timer is currently not running (and not paused), then the start action will be run.
    if timer::state() == TimerState::NotRunning {
        if auto_splitter_start(&vars, custom_vars, &settings) {
            custom_vars.run_start_time = Some(Instant::now());
            timer::start();

            if auto_splitter_is_loading(&vars, custom_vars, &settings) {
                timer::pause_game_time()
            } else {
                timer::resume_game_time();
            }
        }
    }
}
