extern crate sdl2;
extern crate png;


use sdl2::pixels::Color;
use sdl2::render::{Canvas, BlendMode};
use sdl2::video::Window;
use sdl2::rect::Rect;

use self::png::{Decoder, ColorType, DecodingError};

use std::io::Read;

use drawable::{Drawable, Position};

#[derive(Clone)]
pub struct PngImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl PngImage {
    #[allow(unused)]
    pub fn load_from_path<R: Read>(r: R) -> Result<Self, DecodingError> {
        PngImage::load_from_path_transform(r, |x| x)
    }

    #[allow(unused)]
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
    fn content(&self) -> Vec<&dyn Drawable> { vec![] }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> { vec![] }

    fn draw(&mut self, canvas: &mut Canvas<Window>, pos: &Position) {
        let creator = canvas.texture_creator();
        let mut texture = creator
            .create_texture_target(None, self.width as u32, self.height as u32)
            .expect("Can't make texture");

        texture.set_blend_mode(BlendMode::Blend);

        texture
            .update(None, self.data.as_slice(), 4 * self.width)
            .expect("Can't update");

        let rect =
            match pos {
                Position::TopLeftCorner(point) => {
                    Rect::new(point.x, point.y, self.width as u32, self.height as u32)
                }
                Position::Center(point) => {
                    Rect::new(
                        point.x - self.width  as i32 / 2,
                        point.y - self.height as i32 / 2,
                        self.width as u32,
                        self.height as u32
                    )
                }
                Position::Rect(r) => {
                    Rect::new(
                        r.x, r.y,
                        self.width as u32,
                        self.height as u32
                    )
                }
            };

        canvas
            .copy(
                &texture,
                None,
                rect,
            )
            .expect("Can't copy");
    }
}
pub trait ImageContainer: Drawable {
    fn get_data(&self) -> &Vec<u8>;
    fn get_data_mut(&mut self) -> &mut Vec<u8>;
    fn into_data(self) -> Vec<u8>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl ImageContainer for PngImage {
    fn get_data(&self)         -> &Vec<u8>     { &self.data }
    fn get_data_mut(&mut self) -> &mut Vec<u8> { &mut self.data }
    fn into_data(self)         -> Vec<u8>      { self.data }
    fn width(&self)            -> usize        { self.width }
    fn height(&self)           -> usize        { self.height }
}
