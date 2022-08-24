use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    game: GameInfo,
}

#[derive(Deserialize)]
struct GameInfo {
    version: String,
}

fn main() {
    println!("cargo:rerun-if-changed=build.toml");

    let config: Config =
        toml::from_str(&fs::read_to_string("build.toml").expect("couldn't read build config"))
            .expect("invalid TOML in build config");

    println!("cargo:rustc-env=XC3_VER={}", config.game.version);
}
