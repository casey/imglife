use image::{DynamicImage, ImageBuffer, RgbaImage, Rgba, Pixel};
use game::Game;
use css_color_parser::Color;
use cell::Cell::*;

pub struct Codec {
  alive: Rgba<u8>,
  dead:  Rgba<u8>,
}

fn color_to_pixel(color: Color) -> Rgba<u8> {
  let Color{r, g, b, a} = color;
  Pixel::from_channels(r, g, b, (a * 255 as f32) as u8)
}

impl Codec {
  pub fn new(alive: Color, dead: Color) -> Codec {
    Codec {
      alive: color_to_pixel(alive),
      dead:  color_to_pixel(dead),
    }
  }

  pub fn decode(&self, image: DynamicImage) -> Game {
    let image = image.to_rgba();

    let cells = image.pixels().map(|pixel| if *pixel == self.alive {
      Alive
    } else {
      Dead
    }).collect();

    Game {
      width:  image.width()  as usize,
      height: image.height() as usize,
      cells,
    }
  }

  pub fn encode(&self, game: Game) -> RgbaImage {
    ImageBuffer::from_fn(game.width as u32, game.height as u32, |x, y| {
      let cell = game.cell(x as usize, y as usize);
      if cell == Alive {
        self.alive
      } else {
        self.dead
      }
    })
  }
}
