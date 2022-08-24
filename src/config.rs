use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Normal = 0,
    Easy = 1,
    Hard = 2,
    VeryHard = 3,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "veryhard")]
    pub very_hard: VeryHardConfig,
    pub custom: CustomConfig,
}

#[derive(Debug, Deserialize)]
pub struct VeryHardConfig {
    pub enabled: bool,
    pub overwrite: Difficulty,
    pub health: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CustomConfig {
    pub enabled: bool,
    pub enemy_attack_power: u32,
    pub enemy_recharge_speed: u32,
    pub enemy_health: u32,
    pub enemy_debuff_resist: u32,
    pub combo_duration: u32,
    pub chain_attack_buildup: u32,
    pub talent_buildup: u32,
    pub player_healing: u32,
    pub chain_damage_ratio: u32,
    pub rage_strikes: bool,
    pub interlink_level_buildup: u32,
    pub interlink_heat_buildup: u32,
}

impl From<u32> for Difficulty {
    fn from(n: u32) -> Self {
        match n {
            0 => Self::Normal,
            1 => Self::Easy,
            2 => Self::Hard,
            3 => Self::VeryHard,
            n => panic!("Unknown difficulty ID {}", n),
        }
    }
}

impl Default for VeryHardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            overwrite: Difficulty::Hard,
            health: true,
        }
    }
}

impl Default for CustomConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            enemy_attack_power: 100,
            enemy_recharge_speed: 100,
            enemy_health: 100,
            enemy_debuff_resist: 100,
            combo_duration: 100,
            chain_attack_buildup: 100,
            talent_buildup: 100,
            player_healing: 100,
            chain_damage_ratio: 100,
            rage_strikes: false,
            interlink_level_buildup: 100,
            interlink_heat_buildup: 100,
        }
    }
}
