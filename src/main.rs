use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

mod dla;
use dla::cell::{create_cell, Cell};
use dla::grid::Grid;

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
  let mut grid = Grid::new(WIDTH, HEIGHT, 50);

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
      for y in 0..HEIGHT {
        for x in 0..WIDTH {
          let cell = &grid.cells[(x as usize, y as usize)];
          match cell {
            Some(cell) => {
              if cell.is_frozen {
                let pos = pitch * cell.y as usize + (cell.x * 4) as usize;
                set_pixel(buffer, pos, cell.color.r, cell.color.g, cell.color.b);
              } else {
                let pos = pitch * cell.y as usize + (cell.x * 4) as usize;
                set_pixel(buffer, pos, cell.color.r, cell.color.g, cell.color.b);
              }
            }
            None => (),
          };
        }
      }
    })?;
    grid.cycle();
    canvas.copy(&texture, None, None)?;
    canvas.present();
  }

  Ok(())
}
