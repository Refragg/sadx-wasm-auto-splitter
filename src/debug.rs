use super::GameStatePair;
use arrayvec::ArrayString;
use asr::print_message;
use numtoa::NumToA;

const MAX_STRING_SIZE: usize = 4096;

pub fn print_game_state(vars: &GameStatePair) {
    let mut num_buffer = [0u8; 20];
    let mut final_string: ArrayString<MAX_STRING_SIZE> = ArrayString::new();

    macro_rules! push_if_changed_num {
        ($var:ident) => {
            if vars.$var.changed() {
                final_string.push_str(stringify!($var));
                final_string.push_str(": ");
                final_string.push_str(vars.$var.current.numtoa_str(10, &mut num_buffer));
                final_string.push('\n');
            }
        };
    }

    macro_rules! push_if_changed_bool {
        ($var:ident) => {
            if vars.$var.changed() {
                final_string.push_str(stringify!($var));
                final_string.push_str(": ");
                final_string.push_str(match vars.$var.current {
                    true => "true",
                    false => "false",
                });
                final_string.push('\n');
            }
        };
    }

    macro_rules! push_if_changed {
        ($var:ident) => {
            if vars.$var.changed() {
                final_string.push_str(stringify!($var));
                final_string.push('\n');
            }
        };
    }

    macro_rules! _push_num {
        ($var:ident) => {
            final_string.push_str(stringify!($var));
            final_string.push_str(": ");
            final_string.push_str(vars.$var.current.numtoa_str(10, &mut num_buffer));
            final_string.push('\n');
        };
    }

    macro_rules! _push_bool {
        ($var:ident) => {
            final_string.push_str(stringify!($var));
            final_string.push_str(": ");
            final_string.push_str(match vars.$var.current {
                true => "true",
                false => "false",
            });
            final_string.push('\n');
        };
    }

    macro_rules! _push {
        ($var:ident) => {
            final_string.push_str(stringify!($var));
            final_string.push('\n');
        };
    }

    //push_if_changed_num!(global_frame_count);
    //push_if_changed_num!(output_frame_count);

    push_if_changed_bool!(in_cutscene);
    push_if_changed_num!(game_status);
    push_if_changed_num!(game_mode);
    push_if_changed_bool!(in_prerendered_movie);
    push_if_changed_num!(current_character);
    push_if_changed_num!(last_story);
    push_if_changed_num!(timer_start);
    push_if_changed_num!(demo_playing);
    push_if_changed_num!(boss_health);
    push_if_changed_num!(level);
    push_if_changed_num!(level_copy);
    push_if_changed_num!(act);
    push_if_changed_num!(lives);
    push_if_changed_num!(e101_flag);
    push_if_changed_num!(adventure_data);
    push_if_changed_num!(rm_flag);
    push_if_changed_num!(mk2_value);
    push_if_changed_num!(walker_value);

    push_if_changed_num!(angel_cutscene);
    push_if_changed_num!(selected_character);
    push_if_changed_num!(music);
    push_if_changed_num!(lw_flag);

    push_if_changed!(emblem_bytes);
    push_if_changed!(event_bytes);

    push_if_changed_num!(light_shoes);
    push_if_changed_num!(crystal_ring);
    push_if_changed_num!(ancient_light);
    push_if_changed_num!(jet_anklet);
    push_if_changed_num!(rhythm_badge);
    push_if_changed_num!(shovel_claw);
    push_if_changed_num!(fighting_gloves);
    push_if_changed_num!(warrior_feather);
    push_if_changed_num!(long_hammer);
    push_if_changed_num!(life_belt);
    push_if_changed_num!(power_rod);
    push_if_changed_num!(lure_ec);
    push_if_changed_num!(lure_ic);
    push_if_changed_num!(lure_ss);
    push_if_changed_num!(lure_mr);
    push_if_changed_num!(jet_booster);
    push_if_changed_num!(laser_blaster);

    if !final_string.is_empty() {
        print_message(final_string.trim_end());
    }
}
