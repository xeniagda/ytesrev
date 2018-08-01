extern crate sdl2;
extern crate png;


use sdl2::pixels::Color;
use sdl2::render::{Canvas, BlendMode};
use sdl2::video::Window;
use sdl2::rect::{Rect, Point};

use self::png::{Decoder, ColorType, DecodingError};

use std::io::Read;

use scene::Drawable;

#[derive(Clone)]
pub struct PngImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl PngImage {
    pub fn load_from_path<R: Read>(r: R) -> Result<Self, DecodingError> {
        PngImage::load_from_path_transform(r, |x| x)
    }

    pub fn load_from_path_transform<R: Read, F: Fn(Color) -> Color>(r: R, transform: F)
            -> Result<Self, DecodingError>
    {
        let (info, mut reader) = Decoder::new(r).read_info()?;

        let (width, height) = (info.width as usize, info.height as usize);

        let mut data = vec![0; width * height * 4];

        for y in 0..height {
            if let Some(row) = reader.next_row()? {
                assert_eq!(row.len(), width * info.color_type.samples());

                for (x, col) in row.chunks(info.color_type.samples()).enumerate() {

                    let sdl_col = match info.color_type {
                        ColorType::RGB  => { Color::RGB(col[0], col[1], col[2]) },
                        ColorType::RGBA => { Color::RGBA(col[0], col[1], col[2], col[3]) },
                        _ => { unimplemented!() }
                    };

                    let sdl_col = transform(sdl_col);

                    data[(y * width + x) * 4 + 0] = sdl_col.b;
                    data[(y * width + x) * 4 + 1] = sdl_col.g;
                    data[(y * width + x) * 4 + 2] = sdl_col.r;
                    data[(y * width + x) * 4 + 3] = sdl_col.a;
                }
            }
        }

        Ok(PngImage {
            width:  width,
            height: height,
            data:   data,
        })
    }

}

impl Drawable for PngImage {
    fn draw(&self, canvas: &mut Canvas<Window>, point: &Point) {
        let creator = canvas.texture_creator();
        let mut texture = creator
            .create_texture_target(None, self.width as u32, self.height as u32)
            .expect("Can't make texture");

        //println!("Color mod: {}", texture.alpha_mod());
        texture.set_blend_mode(BlendMode::Blend);

        texture
            .update(None, self.data.as_slice(), 4 * self.width)
            .expect("Can't update");

        canvas
            .copy(
                &texture,
                None,
                Rect::new(point.x, point.y, self.width as u32, self.height as u32),
            )
            .expect("Can't copy");
    }
}
