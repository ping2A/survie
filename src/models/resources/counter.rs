use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct CounterInformation {
    pub highscore: u32,
    pub enemy_killed: u32
}

impl CounterInformation {
    pub fn reset(&mut self) {
        self.highscore = 0;
        self.enemy_killed = 0;
    }

    pub fn increase_kill(&mut self) {
        self.enemy_killed += 1;
    }

    pub fn kills(&self) -> u32 {
        self.enemy_killed
    }

    pub fn increase_highscore(&mut self, value: u32) {
        self.highscore += value;
    }

    pub fn highscore(&self) -> u32 {
        self.highscore
    }
}