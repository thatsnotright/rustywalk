extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::mem::size_of;

struct Pos {
  point: sdl2::rect::Point,
  flame: Color,
}

fn create_random_start(w: u32, h: u32) -> Pos {
  Pos {
    point: sdl2::rect::Point::new(rand::random(), rand::random()),
    flame: Color::RGB(rand::random(), rand::random(), rand::random()),
  }
}

pub fn main() -> Result<(), String> {
  const w: u32 = 800;
  const h: u32 = 600;
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("rust-sdl2 demo: Window", w, h)
    .build()
    .map_err(|e| e.to_string())?;

  let mut canvas = window
    .into_canvas()
    .present_vsync()
    .build()
    .map_err(|e| e.to_string())?;

  let mut tick = 0;

  let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

  const flames: [&Pos; 8] = [
    &create_random_start(w, h),
    &create_random_start(w, h),
    &create_random_start(w, h),
    &create_random_start(w, h),
    &create_random_start(w, h),
    &create_random_start(w, h),
    &create_random_start(w, h),
    &create_random_start(w, h),
  ];

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }

    {
      tick += 1;
    }

    for f in flames.iter() {
      canvas.set_draw_color(f.flame);
      canvas.draw_point(f.point);
    }

    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();
    canvas.present();
  }

  Ok(())
}
