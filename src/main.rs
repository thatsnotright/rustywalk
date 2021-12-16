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
      rng.gen_range(Range { start: 0, end: 234 }),
      rng.gen_range(Range { start: 0, end: 234 }),
      rng.gen_range(Range { start: 0, end: 234 }),
    ),
  }
}

fn is_not_black(buffer: &[u8], pos: usize) -> bool {
  return buffer[pos + 1] > 0 || buffer[pos + 2] > 0 || buffer[pos + 3] > 0;
}

fn is_frozen(buffer: &mut [u8], pitch: usize, x: i32, y: i32, w: u32, h: u32) -> bool {
  let pos = pitch * (y as usize) + x as usize * 4;
  let len = pitch * h as usize;
  // above
  if pos > pitch && is_not_black(buffer, pos - pitch) {
    return true;
  }
  // below
  else if pos + pitch + 4 < len && is_not_black(buffer, pos + pitch) {
    return true;
  }
  // left
  else if pos > 3 && is_not_black(buffer, pos - 4) {
    return true;
  }
  // right
  else if pos + 4 < len && is_not_black(buffer, pos + 4) {
    return true;
  }
  // above left
  else if pos > pitch + 4 && is_not_black(buffer, pos - pitch - 4) {
    return true;
  }
  // above right
  else if pos > pitch && is_not_black(buffer, pos - pitch + 4) {
    return true;
  }
  // below left
  else if pos + pitch - 4 < len && is_not_black(buffer, pos + pitch - 4) {
    return true;
  }
  // below right
  else if pos + pitch + 4 < len && is_not_black(buffer, pos + pitch + 4) {
    return true;
  } else {
    return false;
  }
}
fn set_pixel(buffer: &mut [u8], pos: usize, r: u8, g: u8, b: u8) {
  buffer[pos] = 0;
  buffer[pos + 1] = b;
  buffer[pos + 2] = g;
  buffer[pos + 3] = r;
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
      PixelFormatEnum::RGBA8888,
      sdl2::render::TextureAccess::Streaming,
      WIDTH,
      HEIGHT,
    )
    .unwrap();
  let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
  let mut flames: std::vec::Vec<Pos> = std::vec::Vec::new();
  for _i in 0..50 {
    flames.push(create_random_start(WIDTH, HEIGHT));
  }
  texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
    // for i in (0..(pitch * HEIGHT as usize - 1 as usize)).step_by(4) {
    //   buffer[i] = 0x00;
    //   buffer[i + 1] = 0x00;
    //   buffer[i + 2] = 0x00;
    //   buffer[i + 3] = 0xff;
    // }
    for _i in 0..50 {
      let x = rng.gen_range(Range {
        start: 0,
        end: WIDTH,
      }) as usize;
      let y = rng.gen_range(Range {
        start: 0,
        end: HEIGHT,
      }) as usize;
      let pos = y * pitch + x * 4;
      buffer[pos] = 255;
      buffer[pos + 1] = 255;
      buffer[pos + 2] = 255;
      buffer[pos + 3] = 255;
    }
  })?;

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
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
      for f in &mut flames {
        let old_x = f.point.x;
        let old_y = f.point.y;
        f.point.x += rng.gen_range(Range { start: -1, end: 2 });
        f.point.y += rng.gen_range(Range { start: -1, end: 2 });
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
        let frozen = is_frozen(buffer, pitch, old_x, old_y, WIDTH, HEIGHT);
        let pos = pitch * (f.point.y() as usize) + (f.point.x() as usize) * 4;
        let old_pos = pitch * (old_y as usize) + old_x as usize * 4;
        if frozen {
          set_pixel(buffer, pos, f.flame.r, f.flame.g, f.flame.b);
          *f = create_random_start(WIDTH, HEIGHT);
        } else {
          set_pixel(buffer, old_pos, 0, 0, 0);
          set_pixel(buffer, pos, f.flame.r, f.flame.g, f.flame.b);
        }
      }
    })?;

    canvas.copy(&texture, None, None)?;
    canvas.present();
  }

  Ok(())
}
