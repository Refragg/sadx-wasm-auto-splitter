use asr::settings::Gui;

#[derive(Gui)]
pub struct Settings {
    /// Auto Reset on Main Menu
    #[default = false]
    pub reset: bool,
    /// Splits when entering a story
    #[default = false]
    pub char_split: bool,
    /// ...But not when in trial
    #[default = false]
    pub char_split_ban_trial: bool,
    /// Don't start when in trial
    #[default = false]
    pub char_start_ban_trial: bool,

    ///// Stages NOT To Split
    //_stages: Title,
    /// --- Stages NOT To Split ---
    #[default = true]
    _stages: bool,

    /// Sky Chase Act 1
    #[default = true]
    pub stage_36: bool,
    /// Sky Chase Act 2
    #[default = true]
    pub stage_37: bool,

    ///// Final Boss To Split
    //_bosses: Title,
    /// --- Final Boss To Split ---
    #[default = true]
    _bosses: bool,

    /// Egg Viper
    #[default = true]
    pub boss_22: bool,
    /// Egg Walker
    #[default = true]
    pub boss_21: bool,
    /// Chaos 6
    #[default = true]
    pub boss_18: bool,
    /// ZERO
    #[default = true]
    pub boss_23: bool,
    /// E-101 'Beta' mkII
    #[default = true]
    pub boss_25: bool,
    /// Perfect Chaos
    #[default = true]
    pub boss_19: bool,

    ///// Sonic Bosses
    //_sonic_boss: Title,
    /// --- Sonic Bosses ---
    #[default = true]
    _sonic_boss: bool,

    /// Chaos 0
    #[default = true]
    pub zero_sonic: bool,
    /// Egg Hornet
    #[default = true]
    pub egg_hornet_sonic: bool,
    /// Knuckles
    #[default = true]
    pub knuckles_sonic: bool,
    /// Chaos 4
    #[default = true]
    pub four_sonic: bool,
    /// Gamma
    #[default = true]
    pub gamma_sonic: bool,
    /// Chaos 6
    #[default = true]
    pub six_sonic: bool,

    ///// Tails Bosses
    //_tails_boss: Title,
    /// --- Tails Bosses ---
    #[default = true]
    _tails_boss: bool,

    /// Egg Hornet
    #[default = true]
    pub egg_hornet_tails: bool,
    /// Knuckles
    #[default = true]
    pub knuckles_tails: bool,
    /// Chaos 4
    #[default = true]
    pub four_tails: bool,
    /// Gamma
    #[default = true]
    pub gamma_tails: bool,

    ///// Knuckles Bosses
    //_knuckles_boss: Title,
    /// --- Knuckles Bosses ---
    #[default = true]
    _knuckles_boss: bool,

    /// Chaos 2
    #[default = true]
    pub two_knuckles: bool,
    /// Sonic
    #[default = true]
    pub sonic_knuckles: bool,
    /// Chaos 4
    #[default = true]
    pub four_knuckles: bool,

    ///// Gamma Bosses
    //_gamma_boss: Title,
    /// --- Gamma Bosses ---
    #[default = true]
    _gamma_boss: bool,

    /// E101 BETA
    #[default = false]
    pub e101_gamma: bool,
    /// Sonic
    #[default = true]
    pub sonic_gamma: bool,

    ///// Miscellaneous
    //_global_misc: Title,
    /// --- Miscellaneous ---
    #[default = true]
    _global_misc: bool,

    /// Split between the Acts (might not work for characters other than sonic)
    #[default = false]
    pub act_split: bool,

    ///// Sonic Miscellaneous
    //_sonic_misc: Title,
    /// --- Sonic Miscellaneous ---
    #[default = true]
    _sonic_misc: bool,

    /// Enter Sewers
    #[default = false]
    pub enter_sewer_sonic: bool,
    /// Enter Casino Area (To Red Mountain)
    #[default = false]
    pub enter_casino_hub_sonic: bool,
    /// Ship Transformation
    #[default = false]
    pub ship_transform: bool,
    /// Splits when going back to the present after Lost World
    #[default = false]
    pub back_from_past: bool,
    /// Enter Station Square from the Train Station
    #[default = false]
    pub enter_ss: bool,
    /// Enter Mystic Ruins from the Train Station
    #[default = false]
    pub enter_mr: bool,

    ///// Knuckles Miscellaneous
    //_knuckles_misc: Title,
    /// --- Knuckles Miscellaneous ---
    #[default = true]
    _knuckles_misc: bool,

    /// Death Warp Mystic Ruin
    #[default = false]
    pub death_mr_knuckles: bool,
    /// Enter Egg Carrier (Before Sky Deck)
    #[default = false]
    pub enter_egg_carrier_knuckles: bool,

    ///// Amy Miscellaneous
    //_amy_misc: Title,
    /// --- Amy Miscellaneous ---
    #[default = true]
    _amy_misc: bool,

    /// Enter Casino Area
    #[default = false]
    pub enter_casino_hub_amy: bool,
    /// Enter Jungle to Final Egg
    #[default = false]
    pub enter_jungle_to_fe_amy: bool,
    /// Exit Jungle from Final Egg
    #[default = false]
    pub exit_jungle_to_ec_amy: bool,
    /// Enter Egg Carrier (Before Zero)
    #[default = false]
    pub enter_egg_carrier_amy: bool,

    ///// Gamma Miscellaneous
    //_gamma_misc: Title,
    /// --- Gamma Miscellaneous ---
    #[default = true]
    _gamma_misc: bool,

    /// Death Mystic Ruin (Windy Valley Flag)
    #[default = false]
    pub death_wv_gamma: bool,
    /// Death Warp (Post Red Mountain)
    #[default = false]
    pub death_mr_gamma: bool,
    /// Enter Egg Carrier (Before MKII)
    #[default = false]
    pub egg_carrier_mk2_gamma: bool,

    ///// Super Sonic Miscellaneous
    //_super_sonic_misc: Title,
    /// --- Super Sonic Miscellaneous ---
    #[default = true]
    _super_sonic_misc: bool,

    /// Enter Cave
    #[default = false]
    pub enter_cave_super_sonic: bool,
    /// Angel Island Cutscene
    #[default = false]
    pub angel_island_super_sonic: bool,
    /// Tikal Cutscene
    #[default = false]
    pub tikal_super_sonic: bool,
    /// Death Warp post Tikal Cutscene
    #[default = false]
    pub death_super_sonic: bool,
    /// Enter Jungle
    #[default = false]
    pub jungle_super_sonic: bool,

    ///// Enter Boss
    //_boss_enter: Title,
    /// --- Enter Boss ---
    #[default = true]
    _boss_enter: bool,

    /// Enter Egg Hornet
    #[default = false]
    pub enter_egg_hornet: bool,
    /// Enter Chaos 2
    #[default = false]
    pub enter_chaos_2: bool,
    /// Enter Chaos 4
    #[default = false]
    pub enter_chaos_4: bool,
    /// Enter Chaos 6
    #[default = false]
    pub enter_chaos_6: bool,
    /// Enter E-101 'Beta' mkI
    #[default = false]
    pub enter_beta_1: bool,
    /// Enter E-101 'Beta' mkII
    #[default = false]
    pub enter_beta_2: bool,

    ///// Enter Stage
    //_stage_enter: Title,
    /// --- Enter Stage ---
    #[default = true]
    _stage_enter: bool,

    /// Enter Emerald Coast
    #[default = false]
    pub enter_ec: bool,
    /// Enter Windy Valley
    #[default = false]
    pub enter_wv: bool,
    /// Enter Casinopolis
    #[default = false]
    pub enter_c: bool,
    /// Enter Ice Cap
    #[default = false]
    pub enter_ic: bool,
    /// Enter Twinkle Park
    #[default = false]
    pub enter_tp: bool,
    /// Enter Speed Highway
    #[default = false]
    pub enter_sh: bool,
    /// Enter Red Mountain
    #[default = false]
    pub enter_rm: bool,
    /// Enter Sky Deck
    #[default = false]
    pub enter_sd: bool,
    /// Enter Lost World
    #[default = false]
    pub enter_lw: bool,
    /// Enter Final Egg
    #[default = false]
    pub enter_fe: bool,
    /// Enter Hot Shelter
    #[default = false]
    pub enter_hs: bool,

    ///// Upgrades
    //_upgrades: Title,
    /// --- Upgrades ---
    #[default = true]
    _upgrades: bool,

    /// Light Speed Shoes
    #[default = false]
    pub lss_upgrade_sonic: bool,
    /// Crystal Ring
    #[default = false]
    pub cr_upgrade_sonic: bool,
    /// Ancient Light
    #[default = false]
    pub al_upgrade_sonic: bool,
    /// Jet Anklet
    #[default = false]
    pub ja_upgrade_tails: bool,
    /// Rhythm Badge
    #[default = false]
    pub rb_upgrade_tails: bool,
    /// Shovel Claw
    #[default = false]
    pub sc_upgrade_knuckles: bool,
    /// Fighting Gloves
    #[default = false]
    pub fg_upgrade_knuckles: bool,
    /// Warrior Feather
    #[default = false]
    pub wf_upgrade_amy: bool,
    /// Long Hammer
    #[default = false]
    pub lh_upgrade_amy: bool,
    /// Power Rod
    #[default = false]
    pub pr_upgrade_big: bool,
    /// Life Belt
    #[default = false]
    pub lb_upgrade_big: bool,
    /// Lure
    #[default = false]
    pub l_upgrade_big: bool,
    /// Jet Booster
    #[default = false]
    pub jb_upgrade_gamma: bool,
    /// Laser Blaster
    #[default = false]
    pub lb_upgrade_gamma: bool,
}
