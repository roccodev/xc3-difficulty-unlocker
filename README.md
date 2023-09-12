# Difficulty unlocker for Xenoblade 3
This mod acts as a replacement for custom difficulty settings in Xenoblade 3, until we get an official solution.  
Additionally, it also allows you to select the hidden "Very Hard" difficulty that's left in the game's files.

> **Important**: While this mod does not deal with persistent data, you are still using it at your own risk. I am
not responsible for anything that could happen to your saves, game, console, account, etc.

## Installation

#### Switch
1. Download the latest version of the mod from the [Releases](releases/latest) page.
2. Extract the archive to root of your SD card.

#### Ryujinx
1. Download the latest version of the mod from the [Releases](releases/latest) page.
2. Open Ryujinx, then right-click on the game and select "Open Atmosphere Mods Directory".
3. From the archive, extract the `exefs` and `romfs` directory into the folder you opened.

## Configuration
By default, the mod changes Hard difficulty to Very Hard.  
You can edit the `/atmosphere/contents/010074f013262000/romfs/difficulty.toml` file to customize the settings to your liking.

Here are the default values for each difficulty:

|  Option  |  Easy  |  Normal  |  Hard  |  Very Hard  |  Custom Settings  |
| -------- | ------ | -------- | ------ | ----------- | ------- |
| Enemy attack power | 75% | **100%** | 125% | 125% | 25->250%, x10
| Enemy recharge speed | 75% | **100%** | 100% | 125% | 25->250%, x10 |
| Enemy HP | 50% | **100%** | 150% | 200% | 25->250%, x10 |
| Debuff duration from enemies | 50% | **100%** | 150% | 200% | 25->250%, x10 |
| Combo duration multiplier | 100% | **100%** | 75% | 75% | 25->250%, x10 |
| Chain attack gauge buildup | 150% | **100%** | 75% | 50% | 25->250%, x10 |
| Talent art buildup speed | 100% | **100%** | 75% | 50% | 25->250%, x10 |
| Healing power multiplier | 100% | **100%** | 75% | 50% | 25->250%, x10 |
| Chain attack damage ratio multiplier | 100% | **100%** | 50% | 50% | 25->250%, x10 |
| Rage clears chain gauge on next attack | -- | -- | -- | On | On/Off |
| Interlink level buildup speed | 125% | **100%** | 75% | 50% | 25->250%, x10 |
| Interlink heat buildup speed | 75% | **100%** | 125% | 125% | 25->250%, x10 |
| (DLC4) Pair special charge speed | 100% | **100%** | 75% | 50% | 25->250%, x10 |

## Build instructions
To build the project, install [Rust](https://rustup.rs/) and run
```sh
./build.sh
```

## License
This mod is distributed under the terms of the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html). See [COPYING](COPYING) for details.

## Credits
* [vaxherd](https://github.com/vaxherd) for [xb3tool](https://github.com/vaxherd/xb3tool)
