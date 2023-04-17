#![feature(once_cell)]
#![feature(cstr_from_bytes_until_nul)]

use config::Difficulty;
use skyline::{
    hook,
    hooks::InlineCtx,
    nn::{self, oe::DisplayVersion},
};
use std::ffi::CStr;
use std::sync::{
    atomic::{AtomicU8, Ordering},
    OnceLock,
};

use crate::config::Config;

mod config;

static CONFIG: OnceLock<Config> = OnceLock::new();
static LEVEL_TEMPLATES: [&[u8]; 2] = ["%d+\0".as_bytes(), "%d-\0".as_bytes()]; // "[ML:icon icon=btn_plus ]" // [ML:icon icon=btn_minus ]

static REAL_DIFFICULTY: AtomicU8 = AtomicU8::new(0);
static CURRENT_DIFFICULTY: AtomicU8 = AtomicU8::new(0);

const OPTION_OFFSETS: [isize; 11] = [
    0x168, 0x16c, 0x170, 0x174, 0x178, 0x17c, 0x180, 0x184, 0x190, 0x188, 0x18c,
];

#[hook(offset = 0x0019565c, inline)]
unsafe fn load_set_difficulty_hook(ctx: &mut InlineCtx) {
    let current_difficulty = *ctx.registers[20].w.as_ref();

    REAL_DIFFICULTY.store(current_difficulty as u8, Ordering::Release);
    if replace_difficulty(ctx, 20, get_config()) {
        CURRENT_DIFFICULTY.store(Difficulty::VeryHard as u8, Ordering::Release);
    } else {
        CURRENT_DIFFICULTY.store(0, Ordering::Release);
    }
}

#[hook(offset = 0x001958b0, inline)]
unsafe fn load_replace_options_hook(ctx: &mut InlineCtx) {
    let config = get_config();
    let custom = &config.custom;

    let real_difficulty = REAL_DIFFICULTY.load(Ordering::Acquire);

    if custom.enabled && real_difficulty == config.overrides.custom as u8 {
        CURRENT_DIFFICULTY.store(Difficulty::Custom as u8, Ordering::Release);

        let base_ptr = *ctx.registers[19].x.as_ref() as *mut u8;
        let set_f32 = |idx, val: u32| {
            *(base_ptr.offset(OPTION_OFFSETS[idx]) as *mut f32) = percent_multiple_of_25(val)
        };

        set_f32(0, custom.enemy_attack_power);
        set_f32(1, custom.enemy_recharge_speed);
        set_f32(2, custom.enemy_debuff_resist);
        set_f32(3, custom.combo_duration);
        set_f32(4, custom.chain_attack_buildup);
        set_f32(5, custom.talent_buildup);
        set_f32(6, custom.player_healing);
        set_f32(7, custom.chain_damage_ratio);
        set_f32(9, custom.interlink_level_buildup);
        set_f32(10, custom.interlink_heat_buildup);

        *(base_ptr.offset(OPTION_OFFSETS[8])) = custom.rage_strikes as u8;
    }

    // Reset real difficulty before it gets saved. Prevents spamming this function every frame
    *ctx.registers[20].w.as_mut() = real_difficulty as u32;
}

#[hook(offset = 0x002d6774, inline)]
unsafe fn hp_set_difficulty_hook(ctx: &mut InlineCtx) {
    let config = get_config();
    if config.very_hard.health {
        replace_difficulty(ctx, 0, config);
    }
}

#[hook(offset = 0x002d67c8, inline)]
unsafe fn hp_replace_hook(ctx: &mut InlineCtx) {
    let config = get_config();
    let custom = &config.custom;
    if custom.enabled && REAL_DIFFICULTY.load(Ordering::Acquire) == config.overrides.custom as u8 {
        let original_health = *((ctx.registers[8].x.as_ref() + 0x20) as *const u32);

        // Game performs the same lossy cast
        *ctx.registers[9].w.as_mut() =
            (original_health as f32 * percent_multiple_of_25(custom.enemy_health)) as u32;
    }
}

#[hook(offset = 0x0070c660, inline)] // rect_TextEnemyLvNo
unsafe fn level_text(ctx: &mut InlineCtx) {
    // This adds a "+" next to the enemy's level if the difficulty is Very Hard,
    // or a "-" if custom settings are applied.
    match CURRENT_DIFFICULTY.load(Ordering::Acquire) {
        n if n == Difficulty::Custom as u8 => {
            *ctx.registers[1].x.as_mut() = LEVEL_TEMPLATES[1].as_ptr() as u64
        }
        n if n == Difficulty::VeryHard as u8 => {
            *ctx.registers[1].x.as_mut() = LEVEL_TEMPLATES[0].as_ptr() as u64
        }
        _ => {}
    }
}

#[skyline::main(name = "xc3_difficulty_unlocker")]
pub fn main() {
    println!("[XC3-DU] Loading...");

    let mut game_ver: DisplayVersion = DisplayVersion { name: [0; 16] };
    unsafe {
        nn::oe::GetDisplayVersion(&mut game_ver as *mut _);
    }
    let build_ver = env!("XC3_VER");
    match CStr::from_bytes_until_nul(&game_ver.name).map(|s| s.to_string_lossy()) {
        Ok(s) if s == build_ver => {}
        r => {
            println!(
                "[XC3-DU] Version mismatch! (Built for {}, got {:?}) Unloading...",
                build_ver, r
            );
            return;
        }
    }

    let config = std::fs::read_to_string("rom:/difficulty.toml")
        .map(|f| toml::de::from_str(&f).unwrap_or_default())
        .unwrap_or_default();
    CONFIG.set(config).unwrap();

    #[cfg(debug_assertions)]
    println!("Loaded config {:#?}", get_config());

    println!("[XC3-DU] Installing hooks");
    skyline::install_hooks!(
        load_set_difficulty_hook,
        load_replace_options_hook,
        hp_set_difficulty_hook,
        hp_replace_hook,
        level_text
    );

    println!("[XC3-DU] Loaded!");
}

fn get_config() -> &'static Config {
    CONFIG.get().unwrap()
}

fn percent_multiple_of_25(int: u32) -> f32 {
    let clamped = (((int + 24) / 25) * 25).max(25).min(250);
    clamped as f32 * 0.01
}

unsafe fn replace_difficulty(ctx: &mut InlineCtx, register: usize, config: &Config) -> bool {
    let difficulty_ptr = ctx.registers[register].w.as_mut();

    if config.very_hard.enabled && config.overrides.very_hard == Difficulty::from(*difficulty_ptr) {
        *difficulty_ptr = Difficulty::VeryHard as u32;
        true
    } else {
        false
    }
}
