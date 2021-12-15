use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use std::ops::Range;

#[derive(Debug)]
struct Pos {
  point: sdl2::rect::Point,
  flame: Color,
}

fn create_random_start(w: u32, h: u32) -> Pos {
  let mut rng = rand::thread_rng();

  Pos {
    point: sdl2::rect::Point::new(
      rng.gen_range(Range {
        start: 0,
        end: w as i32,
      }),
      rng.gen_range(Range {
        start: 0,
        end: h as i32,
      }),
    ),
    flame: Color::RGB(
      rng.gen_range(Range { start: 0, end: 24 }),
      rng.gen_range(Range { start: 0, end: 24 }),
      rng.gen_range(Range { start: 0, end: 24 }),
    ),
  }
}

pub fn main() -> Result<(), String> {
  const WIDTH: u32 = 800;
  const HEIGHT: u32 = 600;
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let mut rng = rand::thread_rng();

  let window = video_subsystem
    .window("rust-sdl2 demo: Window", WIDTH, HEIGHT)
    .build()
    .map_err(|e| e.to_string())?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
  let tc = canvas.texture_creator();
  let mut texture = tc
    .create_texture(
      PixelFormatEnum::RGB888,
      sdl2::render::TextureAccess::Streaming,
      WIDTH,
      HEIGHT,
    )
    .unwrap();
  let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
  let mut flames: std::vec::Vec<Pos> = std::vec::Vec::new();
  flames.push(create_random_start(WIDTH, HEIGHT));
  flames.push(create_random_start(WIDTH, HEIGHT));
  flames.push(create_random_start(WIDTH, HEIGHT));
  flames.push(create_random_start(WIDTH, HEIGHT));
  flames.push(create_random_start(WIDTH, HEIGHT));
  flames.push(create_random_start(WIDTH, HEIGHT));
  flames.push(create_random_start(WIDTH, HEIGHT));
  flames.push(create_random_start(WIDTH, HEIGHT));

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
    canvas.clear();
    for f in &mut flames {
      f.point.x -= rng.gen_range(Range { start: -1, end: 2 });
      f.point.y -= rng.gen_range(Range { start: -1, end: 2 });
      if f.point.x < 0 {
        f.point.x = WIDTH as i32;
      }
      if f.point.x >= WIDTH as i32 {
        f.point.x = 0;
      }
      if f.point.y < 0 {
        f.point.y = HEIGHT as i32;
      }
      if f.point.y >= HEIGHT as i32 {
        f.point.y = 0;
      }
      texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let pos = pitch * (f.point.y() as usize) + f.point.x() as usize * 4;
        let halt = rng.gen_range(Range {
          start: 0,
          end: 1000,
        });
        let rinc = rng.gen_range(Range { start: 0, end: 2 });
        if halt > 990 {
          buffer[pos] = 255;
          buffer[pos + (1 as usize)] = 255;
          buffer[pos + (2 as usize)] = 255;
          buffer[pos + (3 as usize)] = 255;
          let new_flame = &create_random_start(WIDTH, HEIGHT);
          f.flame = new_flame.flame;
          f.point = new_flame.point;
        } else {
          buffer[pos] = 0;
          buffer[pos + (1 as usize)] = f.flame.r;
          buffer[pos + (2 as usize)] = f.flame.g;
          buffer[pos + (3 as usize)] = f.flame.b;
          f.flame.r = (f.flame.r + rinc) % 255;
          f.flame.b = (f.flame.b + rinc) % 255;
          f.flame.g = (f.flame.g + rinc) % 255;
        }
      })?
    }
    canvas.copy(&texture, None, None);
    canvas.present();
  }

  Ok(())
}
