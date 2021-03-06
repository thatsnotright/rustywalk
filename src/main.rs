#![feature(hash_drain_filter)]
use clap::Parser;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

mod dla;
mod rtmp;
mod universe;
use dla::cell::Cell;
use dla::grid::Grid;

fn set_pixel(buffer: &mut [u8], pos: usize, r: u8, g: u8, b: u8) {
  buffer[pos] = 0;
  buffer[pos + 1] = b;
  buffer[pos + 2] = g;
  buffer[pos + 3] = r;
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
  /// Canvas Width
  #[clap(short, long)]
  width: u32,
  /// Canvas Height
  #[clap(short, long)]
  height: u32,
  /// Destination IP
  #[clap(short, long)]
  ip: String,
  /// Destination Port
  #[clap(short, long)]
  port: u16,
}

#[tokio::main]
async fn main() -> Result<(), String> {
  let args = Args::parse();

  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("rust-sdl2 demo: Window", args.width * 4, args.height * 4)
    .build()
    .map_err(|e| e.to_string())?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
  let tc = canvas.texture_creator();
  let mut texture = tc
    .create_texture(
      PixelFormatEnum::RGBA8888,
      sdl2::render::TextureAccess::Streaming,
      args.width,
      args.height,
    )
    .unwrap();
  let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
  let mut grid = Grid::new(args.width, args.height, 100);

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        Event::KeyDown {
          keycode: Some(Keycode::Space),
          ..
        } => grid.cycle(),
        _ => {}
      }
    }
    canvas.clear();
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
      buffer.fill(0);
      for y in 0..args.height {
        for x in 0..args.width {
          let cell = &grid.cells[(x as usize, y as usize)];
          let pos = pitch * y as usize + (x * 4) as usize;

          match cell {
            Some(cell) => {
              set_pixel(buffer, pos, cell.color.r, cell.color.g, cell.color.b);
            }
            None => {}
          };
        }
      }
      for cell in &grid.active_cells {
        let pos = pitch * cell.y + (cell.x * 4) as usize;
        set_pixel(buffer, pos, cell.color.r, cell.color.g, cell.color.b);
      }
    })?;
    canvas.copy(&texture, None, None)?;
    canvas.present();
    grid.cycle();
  }

  Ok(())
}
