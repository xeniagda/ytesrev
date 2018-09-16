//! Utilities to load PNG images

extern crate png;
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;

use self::png::{ColorType, Decoder, DecodingError};

use std::io::Read;

use drawable::{DrawSettings, Drawable, KnownSize, Position, State};

/// A PNG image. Currently only supports RGB and RGBA color types
#[derive(Clone)]
pub struct PngImage {
    /// The width of the image
    pub width: usize,
    /// The height of the image
    pub height: usize,
    /// The data in the image, stored in chunks of 4 per pixel, containing the image in ABGR order
    pub data: Vec<u8>,
}

impl PngImage {
    /// Load an image from a specified source.
    pub fn load_from_path<R: Read>(r: R) -> Result<Self, DecodingError> {
        PngImage::load_from_path_transform(r, |x| x)
    }

    /// Load an image and apply a function to each pixel. Mostly used by [`LatexObj`] to fix alpha
    ///
    /// [`LatexObj`]: ../latex/latex_obj/struct.LatexObj.html
    pub fn load_from_path_transform<R: Read, F: Fn(Color) -> Color>(
        r: R,
        transform: F,
    ) -> Result<Self, DecodingError> {
        let (info, mut reader) = Decoder::new(r).read_info()?;

        let (width, height) = (info.width as usize, info.height as usize);

        let mut data = vec![0; width * height * 4];

        for y in 0..height {
            if let Some(row) = reader.next_row()? {
                assert_eq!(row.len(), width * info.color_type.samples());

                for (x, col) in row.chunks(info.color_type.samples()).enumerate() {
                    let sdl_col = match info.color_type {
                        ColorType::RGB => Color::RGB(col[0], col[1], col[2]),
                        ColorType::RGBA => Color::RGBA(col[0], col[1], col[2], col[3]),
                        _ => unimplemented!(),
                    };

                    let sdl_col = transform(sdl_col);

                    data[(y * width + x) * 4] = sdl_col.b;
                    data[(y * width + x) * 4 + 1] = sdl_col.g;
                    data[(y * width + x) * 4 + 2] = sdl_col.r;
                    data[(y * width + x) * 4 + 3] = sdl_col.a;
                }
            }
        }

        Ok(PngImage {
            width,
            height,
            data,
        })
    }
}

impl Drawable for PngImage {
    fn content(&self) -> Vec<&dyn Drawable> {
        vec![]
    }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![]
    }

    fn draw(&self, canvas: &mut Canvas<Window>, pos: &Position, _settings: DrawSettings) {
        let creator = canvas.texture_creator();
        let mut texture = creator
            .create_texture_target(None, self.width as u32, self.height as u32)
            .expect("Can't make texture");

        texture.set_blend_mode(BlendMode::Blend);

        texture
            .update(None, self.data.as_slice(), 4 * self.width)
            .expect("Can't update");

        let rect = pos.into_rect_with_size_unbounded(self.width as u32, self.height as u32);

        canvas.copy(&texture, None, rect).expect("Can't copy");
    }

    fn step(&mut self) {}
    fn state(&self) -> State {
        State::Final
    }
}

/// Something that can act as, or contains, an image.
pub trait ImageContainer: KnownSize + Sized {
    /// Retrieve the data in the image
    fn get_data(&self) -> &Vec<u8>;

    /// Retrieve the data in the image, mutably
    fn get_data_mut(&mut self) -> &mut Vec<u8>;

    /// Retrieve the data in the image, consuming the object
    fn into_data(self) -> Vec<u8>;

    /// Convert the object to a dynamic KnownSize object, as rust doesn't support calling KnownSize
    /// -methods directly on this object
    fn as_knownsize(&self) -> &dyn KnownSize {
        self
    }
}

impl KnownSize for PngImage {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

impl ImageContainer for PngImage {
    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    fn into_data(self) -> Vec<u8> {
        self.data
    }
}
