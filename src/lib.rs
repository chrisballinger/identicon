extern crate image;

use std::slice::Iter;

use image::{ImageBuffer, Rgb, RgbImage};

struct HSL {
    hue: f32,
    sat: f32,
    lum: f32,
}

impl HSL {
    pub fn new(hue: f32, sat: f32, lum: f32) -> HSL {
        HSL { hue: hue, sat: sat, lum: lum }
    }

    // http://www.w3.org/TR/css3-color/#hsl-color
    pub fn rgb(&self) -> Rgb<u8> {
        let hue = self.hue / 360.0;
        let sat = self.sat / 100.0;
        let lum = self.lum / 100.0;

        let b =
            if lum <= 0.5 {
                lum * (sat + 1.0)
            } else {
                lum + sat - lum * sat
            };
        let a = lum * 2.0 - b;

        let r = HSL::hue_to_rgb(a, b, hue + 1.0/3.0);
        let g = HSL::hue_to_rgb(a, b, hue);
        let b = HSL::hue_to_rgb(a, b, hue - 1.0/3.0);

        Rgb([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8])
    }

    fn hue_to_rgb(a: f32, b: f32, hue: f32) -> f32 {
        let h =
            if hue < 0.0 {
                hue + 1.0
            } else if hue > 1.0 {
                hue - 1.0
            } else {
                hue
            };
        if h < 1.0/6.0 { return a + (b - a) * 6.0 * h; }
        if h < 1.0/2.0 { return b; }
        if h < 2.0/3.0 { return a + (b - a) * (2.0/3.0 - h) * 6.0; }
        a
    }
}

struct Nibbler<'a> {
    byte: Option<u8>,
    bytes: Iter<'a, u8>,
}

impl<'a> Nibbler<'a> {
    pub fn new(bytes: &[u8]) -> Nibbler {
        Nibbler { bytes: bytes.iter(), byte: None }
    }
}

impl<'a> Iterator for Nibbler<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        match self.byte {
            Some(value) => {
                self.byte = None;
                Some(value)
            },
            None => {
                match self.bytes.next() {
                    Some(value) => {
                        let hi = *value & 0xf0;
                        let lo = *value & 0x0f;
                        self.byte = Some(lo);
                        Some(hi >> 4)
                    },
                    None => None
                }
            },
        }
    }
}

pub struct Identicon<'a> {
    source: &'a[u8],
    size: u32,
}

impl<'a> Identicon<'a> {
    pub fn new(source: &[u8]) -> Identicon {
        Identicon { source: source, size: 420 }
    }

    // https://processing.org/reference/map_.html
    fn map(value: u32, vmin: u32, vmax: u32, dmin: u32, dmax: u32) -> f32 {
        ((value - vmin) * (dmax - dmin)) as f32 / ((vmax - vmin) + dmin) as f32
    }

    fn foreground(&self) -> Rgb<u8> {
        // Use last 28 bits to determine HSL values.
        let h1 = (self.source[12] as u16 & 0x0f) << 8;
        let h2 = self.source[13] as u16;

        let h = (h1 | h2) as u32;
        let s = self.source[14] as u32;
        let l = self.source[15] as u32;

        let hue = Identicon::map(h, 0, 4095, 0, 360);
        let sat = Identicon::map(s, 0, 255, 0, 20);
        let lum = Identicon::map(l, 0, 255, 0, 20);

        HSL::new(hue, 65.0 - sat, 75.0 - lum).rgb()
    }

    fn rect(image: &mut RgbImage, x0: u32, y0: u32, x1: u32, y1: u32, color: Rgb<u8>) {
        for x in (x0..x1) {
            for y in (y0..y1) {
                image.put_pixel(x, y, color);
            }
        }
    }

    pub fn image(&self) -> RgbImage {
        let pixel_size = 70;
        let sprite_size = 5;
        let inner_size = sprite_size * pixel_size;

        let background = Rgb([240, 240, 240]);
        let foreground = self.foreground();

        let mut image: RgbImage = ImageBuffer::from_pixel(self.size, self.size, background);

        let half_axis = (sprite_size - 1) / 2;
        let margin = pixel_size / 2;

        let mut nibbles = Nibbler::new(self.source);
        let mut x = half_axis * pixel_size;

        while x >= 0 {
            let mut y = 0;
            while y < inner_size {
                if nibbles.next().unwrap() % 2 == 0 {
                    Identicon::rect(&mut image,
                        (x + margin) as u32,
                        (y + margin) as u32,
                        (x + pixel_size + margin) as u32,
                        (y + pixel_size + margin) as u32,
                        foreground);

                    // Mirror blocks across axis.
                    if x != half_axis * pixel_size {
                        let x_start = 2 * half_axis * pixel_size - x;
                        Identicon::rect(&mut image,
                            (x_start + margin) as u32,
                            (y + margin) as u32,
                            (x_start + pixel_size + margin) as u32,
                            (y + pixel_size + margin) as u32,
                            foreground);
                    }
                }
                y += pixel_size;
            }
            x -= pixel_size;
        }

        image
    }
}
