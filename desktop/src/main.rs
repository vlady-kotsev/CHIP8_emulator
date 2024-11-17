use chip8_core::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};
use std::{env, fs::File, io::Read, time::Duration};

const TICKS_PER_FRAME: usize = 15;
const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run path/to/game color <red,green,blue,pink>");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Emulator::new();

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Failed to read rom");

    chip8.load(&buffer);

    let mut color = Color::RGB(255, 255, 255);
    if args.len() == 3 {
        color = get_color(&args[2]);
    }
    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'gameloop;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(key) = key_to_btn(key) {
                        chip8.keypressed(key, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(key) = key_to_btn(key) {
                        chip8.keypressed(key, false);
                    }
                }
                _ => (),
            }
        }
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timers();
        draw_to_screen(&mut canvas, &chip8, &color);
        std::thread::sleep(Duration::from_millis(2));
    }
}

fn draw_to_screen(canvas: &mut Canvas<Window>, emu: &Emulator, color: &Color) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_display();

    canvas.set_draw_color(*color);
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn get_color(color_str: &str) -> Color {
    match color_str.to_lowercase().as_str() {
        "red" => Color::RGB(255, 0, 0),
        "green" => Color::RGB(0, 255, 0),
        "blue" => Color::RGB(0, 0, 255),
        "pink" => Color::RGB(255, 0, 255),
        _ => Color::RGB(255, 255, 255),
    }
}

fn key_to_btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
