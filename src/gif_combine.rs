use bytes::Bytes;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, ImageError, ImageFormat};
use std::io::Write;

pub struct GifCombine<W: Write> {
    fps: u8,
    encoder: GifEncoder<W>,
}

impl<W: Write> GifCombine<W> {
    pub fn new(fps: u8, output: W) -> Self {
        let mut encoder = GifEncoder::new(output);
        encoder
            .set_repeat(Repeat::Infinite)
            .expect("Failed to set GIF infinite repeat");

        GifCombine { fps, encoder }
    }

    pub fn add_frame(&mut self, bytes: &Bytes) -> Result<(), ImageError> {
        let image = image::load_from_memory_with_format(bytes, ImageFormat::Png)?;

        let frame = Frame::from_parts(
            image.into_rgba8(),
            0,
            0,
            Delay::from_numer_denom_ms(1000, self.fps as u32),
        );

        self.encoder.encode_frame(frame)
    }
}
