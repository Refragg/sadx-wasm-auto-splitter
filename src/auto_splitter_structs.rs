use core::ops::Index;

pub struct BossSplits {
    boss_18: bool,
    boss_22: bool,
}

impl BossSplits {
    pub fn new() -> Self {
        Self {
            boss_18: false,
            boss_22: false,
        }
    }

    pub fn set_boss_18(&mut self) {
        self.boss_18 = true;
    }

    pub fn set_boss_22(&mut self) {
        self.boss_22 = true;
    }

    pub fn reset(&mut self) {
        self.boss_18 = false;
        self.boss_22 = false
    }
}

impl Index<u8> for BossSplits {
    type Output = bool;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            18 => &self.boss_18,
            22 => &self.boss_22,
            _ => &false,
        }
    }
}

pub struct IgnoredStages {
    stage_36: bool,
    stage_37: bool,
}

impl IgnoredStages {
    pub fn new() -> Self {
        Self {
            stage_36: false,
            stage_37: false,
        }
    }

    pub fn set_stage_36(&mut self) {
        self.stage_36 = true;
    }

    pub fn set_stage_37(&mut self) {
        self.stage_37 = true;
    }

    pub fn reset(&mut self) {
        self.stage_36 = false;
        self.stage_37 = false
    }
}

impl Index<u8> for IgnoredStages {
    type Output = bool;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            36 => &self.stage_36,
            37 => &self.stage_37,
            _ => &false,
        }
    }
}
