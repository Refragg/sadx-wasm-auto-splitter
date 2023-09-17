use asr::{
    watcher::{Pair, Watcher},
    Address,
};

macro_rules! define_address {
    ($name:ident, $addr:expr) => {
        pub const $name: Address = Address::new($addr);
    };
}

define_address!(GLOBAL_FRAME_COUNT_ADDRESS, 0x03B2C6C8);
define_address!(OUTPUT_FRAME_COUNT_ADDRESS, 0x03ABDF58);
define_address!(IN_CUTSCENE_ADDRESS, 0x03B2C55C);
define_address!(GAME_STATUS_ADDRESS, 0x03B22DE4);
define_address!(GAME_MODE_ADDRESS, 0x03ABDC7C);
define_address!(IN_PRERENDERED_MOVIE_ADDRESS, 0x03A8B0D0);
define_address!(CURRENT_CHARACTER_ADDRESS, 0x03B22DC0);
define_address!(LAST_STORY_ADDRESS, 0x03B18DB4);
define_address!(TIMER_START_ADDRESS, 0x00912DF0);
define_address!(DEMO_PLAYING_ADDRESS, 0x03B2A2E4);
define_address!(BOSS_HEALTH_ADDRESS, 0x03C58150);
define_address!(LEVEL_ADDRESS, 0x03B22DCC);
define_address!(LEVEL_COPY_ADDRESS, 0x03B22DC4);
define_address!(ACT_ADDRESS, 0x03B22DEC);
define_address!(LIVES_ADDRESS, 0x03B0EF34);
define_address!(E101_FLAG_ADDRESS, 0x03B189A8);
define_address!(ADVENTURE_DATA_ADDRESS, 0x03B183F0);
define_address!(RM_FLAG_ADDRESS, 0x03B189A5);
define_address!(MK2_VALUE_ADDRESS, 0x03C6C760);
define_address!(WALKER_VALUE_ADDRESS, 0x03C5A7ED);

define_address!(ANGEL_CUTSCENE_ADDRESS, 0x03C83D08);
define_address!(SELECTED_CHARACTER_ADDRESS, 0x03B2A2FD);
define_address!(MUSIC_ADDRESS, 0x00912698);
define_address!(LW_FLAG_ADDRESS, 0x03B1892A);

define_address!(EMBLEM_BYTES_ADDRESS, 0x03B2B5E8);
define_address!(EVENT_BYTES_ADDRESS, 0x03B18888);

define_address!(LIGHT_SHOES_ADDRESS, 0x03B18895);
define_address!(CRYSTAL_RING_ADDRESS, 0x03B18896);
define_address!(ANCIENT_LIGHT_ADDRESS, 0x03B188A7);
define_address!(JET_ANKLET_ADDRESS, 0x03B188D5);
define_address!(RHYTHM_BADGE_ADDRESS, 0x03B188E3);
define_address!(SHOVEL_CLAW_ADDRESS, 0x03B18921);
define_address!(FIGHTING_GLOVES_ADDRESS, 0x03B18922);
define_address!(WARRIOR_FEATHER_ADDRESS, 0x03B1895A);
define_address!(LONG_HAMMER_ADDRESS, 0x03B18966);
define_address!(LIFE_BELT_ADDRESS, 0x03B189D8);
define_address!(POWER_ROD_ADDRESS, 0x03B189D9);
define_address!(LURE_EC_ADDRESS, 0x03B189E6);
define_address!(LURE_IC_ADDRESS, 0x03B189E7);
define_address!(LURE_SS_ADDRESS, 0x03B189E8);
define_address!(LURE_MR_ADDRESS, 0x03B189E9);
define_address!(JET_BOOSTER_ADDRESS, 0x03B18991);
define_address!(LASER_BLASTER_ADDRESS, 0x03B18992);

#[derive(Default)]
pub struct GameState {
    pub global_frame_count: Watcher<i32>,
    pub output_frame_count: Watcher<i32>,
    pub in_cutscene: Watcher<bool>,
    pub game_status: Watcher<u8>,
    pub game_mode: Watcher<u8>,
    pub in_prerendered_movie: Watcher<bool>,
    pub current_character: Watcher<u8>,
    pub last_story: Watcher<u8>,
    pub timer_start: Watcher<u8>,
    pub demo_playing: Watcher<u8>,
    pub boss_health: Watcher<u8>,
    pub level: Watcher<u8>,
    pub level_copy: Watcher<u8>,
    pub act: Watcher<u8>,
    pub lives: Watcher<u8>,
    pub e101_flag: Watcher<u8>,
    pub adventure_data: Watcher<u8>,
    pub rm_flag: Watcher<u8>,
    pub mk2_value: Watcher<u8>,
    pub walker_value: Watcher<u8>,

    pub angel_cutscene: Watcher<u8>,
    pub selected_character: Watcher<u8>,
    pub music: Watcher<u8>,
    pub lw_flag: Watcher<u8>,

    pub emblem_bytes: Watcher<[u8; 17]>,
    pub event_bytes: Watcher<[u8; 353]>,

    pub light_shoes: Watcher<u8>,
    pub crystal_ring: Watcher<u8>,
    pub ancient_light: Watcher<u8>,
    pub jet_anklet: Watcher<u8>,
    pub rhythm_badge: Watcher<u8>,
    pub shovel_claw: Watcher<u8>,
    pub fighting_gloves: Watcher<u8>,
    pub warrior_feather: Watcher<u8>,
    pub long_hammer: Watcher<u8>,
    pub life_belt: Watcher<u8>,
    pub power_rod: Watcher<u8>,
    pub lure_ec: Watcher<u8>,
    pub lure_ic: Watcher<u8>,
    pub lure_ss: Watcher<u8>,
    pub lure_mr: Watcher<u8>,
    pub jet_booster: Watcher<u8>,
    pub laser_blaster: Watcher<u8>,
}

#[derive()]
pub struct GameStatePair {
    pub global_frame_count: Pair<i32>,
    pub output_frame_count: Pair<i32>,
    pub in_cutscene: Pair<bool>,
    pub game_status: Pair<u8>,
    pub game_mode: Pair<u8>,
    pub in_prerendered_movie: Pair<bool>,
    pub current_character: Pair<u8>,
    pub last_story: Pair<u8>,
    pub timer_start: Pair<u8>,
    pub demo_playing: Pair<u8>,
    pub boss_health: Pair<u8>,
    pub level: Pair<u8>,
    pub level_copy: Pair<u8>,
    pub act: Pair<u8>,
    pub lives: Pair<u8>,
    pub e101_flag: Pair<u8>,
    pub adventure_data: Pair<u8>,
    pub rm_flag: Pair<u8>,
    pub mk2_value: Pair<u8>,
    pub walker_value: Pair<u8>,

    pub angel_cutscene: Pair<u8>,
    pub selected_character: Pair<u8>,
    pub music: Pair<u8>,
    pub lw_flag: Pair<u8>,

    pub emblem_bytes: Pair<[u8; 17]>,
    pub event_bytes: Pair<[u8; 353]>,

    pub light_shoes: Pair<u8>,
    pub crystal_ring: Pair<u8>,
    pub ancient_light: Pair<u8>,
    pub jet_anklet: Pair<u8>,
    pub rhythm_badge: Pair<u8>,
    pub shovel_claw: Pair<u8>,
    pub fighting_gloves: Pair<u8>,
    pub warrior_feather: Pair<u8>,
    pub long_hammer: Pair<u8>,
    pub life_belt: Pair<u8>,
    pub power_rod: Pair<u8>,
    pub lure_ec: Pair<u8>,
    pub lure_ic: Pair<u8>,
    pub lure_ss: Pair<u8>,
    pub lure_mr: Pair<u8>,
    pub jet_booster: Pair<u8>,
    pub laser_blaster: Pair<u8>,
}
