# riichi-tools-rs
v0.0.63

![Build Status](https://github.com/harphield/riichi-tools-rs/workflows/Build,%20test%20and%20Clippy/badge.svg)

A tool library for riichi mahjong written in Rust, made mostly to be used as a WASM component.

Lots of inspiration from other mahjong tools, like https://euophrys.itch.io/mahjong-efficiency-trainer, http://tenhou.net/2/, 
http://kobalab.net/majiang/dapai.html etc.

Showcase / frontend of this library can be found at https://riichi.harphield.com/tools/

## Roadmap
- South 4 Simulator
    - A game where you try to win in the last round [DONE] [moved to a separate project, riichi-tools-wasm]
- Hand analysis
    - Shanten for 13 tiles [DONE]
    - Shanten for 14 tiles with complete hand indication [DONE]
    - Ukeire, tile acceptance [DONE]    
    - Hand value + yaku for complete hand (14 tiles) [DONE]
    - Hand value + yaku for tenpai hand with possible outcomes [DONE]
    - All of the above also for hands with calls [DONE]
    - Rule variants (Tenhou vs WRC vs MahjongSoul etc.)
- Table analysis
    - Safe tiles
    - Wait probability percentages
- Replay analysis
    - Tenhou replay parsing
    - Majsoul replay parsing
    - Discard rating

## Hand representation parsing
`Hand::from_text()` method accepts a string representation in the following format:

- **1-9 m, s or p** for manzu, souzu and pinzu tiles
- **0 m, s or p** are red 5 (support is mostly WIP at the moment)
- **1-7 z** for honor tiles, where
    - 1 = east
    - 2 = south
    - 3 = west
    - 4 = north
    - 5 = white dragon
    - 6 = green dragon
    - 7 = red dragon
- Open shapes
    - **(XYZCI)** is chi, where XYZ are consecutive numbers, C is the color (m, p, s) and I is the index of the called tile (0-2).
    - **(pXCI)** is pon, where X is 0-9, C is the color (m, p, s, z) and I is the index of the player from who we called (1 = shimocha, 2 = toimen, 3 = kamicha). In case the pon has a red 5, the representation will use `0` - for example **(p0m2)**.
    - **(kXCI)** is kan. Same rules as pon apply, but **I** is optional - if not available, the kan is considered closed.