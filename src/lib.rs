#![feature(once_cell)]
#![feature(cstr_from_bytes_until_nul)]

use config::Difficulty;
use skyline::{
    hook,
    hooks::InlineCtx,
    nn::{self, oe::DisplayVersion},
};
use std::sync::LazyLock;
use std::{ffi::CStr, ops::Deref};

use crate::config::Config;

mod config;

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    std::fs::read_to_string("rom:/difficulty/config.toml")
        .map(|f| toml::from_str(&f).unwrap_or_default())
        .unwrap_or_default()
});

const OPTION_OFFSETS: [isize; 11] = [
    0x168, 0x16c, 0x170, 0x174, 0x178, 0x17c, 0x180, 0x184, 0x190, 0x188, 0x18c,
];

#[hook(offset = 0x0018ee24, inline)]
unsafe fn load_set_difficulty_hook(ctx: &mut InlineCtx) {
    replace_difficulty(ctx, 20);
}

#[hook(offset = 0x0018f030, inline)]
unsafe fn load_replace_options_hook(ctx: &mut InlineCtx) {
    let custom = &CONFIG.custom;
    if custom.enabled {
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
        set_f32(9, custom.reaction_chance);
        set_f32(10, custom.interlink_heat_buildup);

        *(base_ptr.offset(OPTION_OFFSETS[8])) = custom.rage_strikes as u8;
    }
}

#[hook(offset = 0x0026b038, inline)]
unsafe fn hp_set_difficulty_hook(ctx: &mut InlineCtx) {
    if CONFIG.very_hard.health {
        replace_difficulty(ctx, 0);
    }
}

#[hook(offset = 0x0026b078, inline)]
unsafe fn hp_replace_hook(ctx: &mut InlineCtx) {
    let custom = &CONFIG.custom;
    if custom.enabled {
        let original_health = *ctx.registers[20].w.as_ref();

        // Game performs the same lossy cast
        *ctx.registers[8].w.as_mut() =
            (original_health as f32 * percent_multiple_of_25(custom.enemy_health)) as u32;
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

    LazyLock::force(&CONFIG);

    #[cfg(debug_assertions)]
    println!("Loaded config {:#?}", CONFIG.deref());

    println!("[XC3-DU] Installing hooks");
    skyline::install_hooks!(
        load_set_difficulty_hook,
        load_replace_options_hook,
        hp_set_difficulty_hook,
        hp_replace_hook
    );

    println!("[XC3-DU] Loaded!");
}

fn percent_multiple_of_25(int: u32) -> f32 {
    let clamped = (((int + 24) / 25) * 25).max(25).min(250);
    clamped as f32 * 0.01
}

unsafe fn replace_difficulty(ctx: &mut InlineCtx, register: usize) {
    let difficulty_ptr = ctx.registers[register].w.as_mut();
    let very_hard_config = &CONFIG.very_hard;

    if very_hard_config.enabled && very_hard_config.overwrite == Difficulty::from(*difficulty_ptr) {
        *difficulty_ptr = Difficulty::VeryHard as u32;
    }
}
