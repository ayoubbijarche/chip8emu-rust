use chip8core::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fs::File;
use std::io::Read;
use std::env;


const SCALE : u32 = 15;
const S_W : u32 = (W as u32) * SCALE;
const S_H : u32 = (H as u32) * SCALE;
const TICK_FRAME: usize = 10;

fn main() {
  //args
  let args : Vec<_> = env::args().collect();
  if args.len() != 2 {
    println!("running cargo at : path/to/game");
    return;
  }
  
  //sdl context
  let sdl_ctx = sdl2::init().unwrap();
  let video_subsystem = sdl_ctx.video().unwrap();
  let window = video_subsystem.window("chip8 emulator", S_W , S_H).position_centered().vulkan().build().unwrap();
  let mut canvas = window.into_canvas().present_vsync().build().unwrap();
  canvas.clear();
  canvas.present();
  let mut event = sdl_ctx.event_pump().unwrap();
  
  //chip8 setup
  let mut chip8 = Emu::new();
  let mut rom = File::open(&args[1]).expect("unable to open the file");
  let mut buffer = Vec::new();
  rom.read_to_end(&mut buffer).unwrap();
  chip8.load(&buffer);
  
  
  'gameloop: loop {
    for evt in event.poll_iter(){
      match evt{
        Event::Quit { .. } => {
          break 'gameloop;
        },
        Event::KeyDown{keycode: Some(key), ..} => {
          if let Some(k) = keytobtn(key) {
            chip8.keypress(k, true);
          }
        },
        Event::KeyUp{keycode: Some(key), ..} => {
          if let Some(k) = keytobtn(key) {
          chip8.keypress(k, false);
          }
        },
        _ => ()
      }
    }
    
    for _ in 0..TICK_FRAME{
      chip8.tick();
    }
    chip8.tick_time();
    draw(&chip8, &mut canvas);
  }
}

fn draw(emu : &Emu, canvas : &mut Canvas<Window>){
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas.clear();
  let screen_buffer = emu.getscreen();
  canvas.set_draw_color(Color::RGB(255, 255, 255));
  for (i , pixel) in screen_buffer.iter().enumerate(){
    if *pixel {
      let x = (i as u32 % S_W) as u32;
      let y = (i as u32 / S_W) as u32;
      let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32 , SCALE , SCALE);
      canvas.fill_rect(rect).unwrap();
    }
  }
  
  canvas.present();
}

fn keytobtn(key : Keycode) -> Option<usize>{
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