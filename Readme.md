<div align="center">
<h1> CHIP-8 Emulator</h1>
<img width="350px" src="https://github.com/user-attachments/assets/d8decd7e-f5c7-481f-9885-5aeac793ecec" alt="chip8-img" />

</div>
A CHIP-8 emulator implemented in Rust with both native SDL2 and WebAssembly frontends.
<br/>
Created following <a href="https://github.com/aquova/chip8-book">An Introduction to Chip-8 Emulation using the Rust Programming Language</a>

## Features

- Complete CHIP-8 instruction set implementation
- Cross-platform support (Native and Web)
- Configurable display colors (Native only)
- Real-time keyboard input
- Adjustable emulation speed
- Support for all standard CHIP-8 ROMs

## Prerequisites

- Rust toolchain (cargo, rustc)
- SDL2 development libraries: 
    [**Download from here**](https://github.com/libsdl-org/SDL/releases/tag/release-2.30.9)
    <br/>
    On MacOS extract and place the extracted folder in `/Library/Frameworks/`
- wasm-pack
- python3 (to run a server for the web version)

## Running

### Native

Run in `desktop` folder: `cargo run <path_to_rom> <color>`
<br/>
Color options: `red, blue, green, pink`

### Web

Run in `wasm` folder:
<br/>
`wasm-pack build --target web && mv pkg/wasm.js ../web/wasm.js && mv pkg/wasm_bg.wasm ../web/wasm_bg.wasm && cd ../web && python3 -m http.server`

## Technical Details

- Written in Rust for performance and safety
- Uses SDL2 for native graphics and input handling
- WebAssembly support for browser-based emulation
