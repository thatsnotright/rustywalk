use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
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
      rng.gen_range(Range { start: 0, end: 255 }),
      rng.gen_range(Range { start: 0, end: 255 }),
      rng.gen_range(Range { start: 0, end: 255 }),
    ),
  }
}

pub fn main() -> Result<(), String> {
  const w: u32 = 800;
  const h: u32 = 600;
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let mut rng = rand::thread_rng();

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
  let mut flames: std::vec::Vec<Pos> = std::vec::Vec::new();
  flames.push(create_random_start(w, h));
  flames.push(create_random_start(w, h));
  flames.push(create_random_start(w, h));
  flames.push(create_random_start(w, h));
  flames.push(create_random_start(w, h));
  flames.push(create_random_start(w, h));
  flames.push(create_random_start(w, h));
  flames.push(create_random_start(w, h));

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

    for f in &mut flames {
      f.point.x -= rng.gen_range(Range { start: -1, end: 2 });
      f.point.y -= rng.gen_range(Range { start: -1, end: 2 });
      if f.point.x < 0 {
        f.point.x = w as i32;
      }
      if f.point.x >= w as i32 {
        f.point.x = 0;
      }
      if f.point.y < 0 {
        f.point.y = h as i32;
      }
      if f.point.y >= h as i32 {
        f.point.y = 0;
      }
      canvas.set_draw_color(f.flame);
      canvas.draw_point(f.point);
    }
    canvas.present();
  }

  Ok(())
}
