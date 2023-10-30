use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Pixel { red, green, blue }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![self.red, self.green, self.blue]
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels: Vec<Pixel> = Vec::with_capacity(width * height);

        for _ in 0..(width * height) {
            pixels.push(Pixel::new(30, 30, 30));
        }

        Image {
            pixels,
            width,
            height,
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut vector = Vec::with_capacity(3 * self.height * self.height);

        for pixel in &self.pixels {
            vector.append(&mut pixel.to_vec());
        }

        vector
    }

    pub fn export(&self, title: &str) {
        let pixels = self.to_vec();

        let file_name = format!("{title}.png");

        let path = Path::new(&file_name);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&pixels).unwrap(); // Save
    }

    pub fn set_pixel(&mut self, pos: Position, pixel: Pixel) {
        let half_width = (self.width / 2) as i64;
        let half_height = (self.height / 2) as i64;

        if pos.x > half_width || pos.x < -half_width || pos.y > half_height || pos.y < -half_height
        {
            panic!("Position invalid!");
        } else {
            let x_pos = pos.x + half_width;
            let y_pos = -pos.y + half_height;

            self.pixels[(y_pos * self.width as i64 + x_pos) as usize] = pixel;
        }
    }

    pub fn set_all_pixels<F: Fn(Position) -> Pixel>(&mut self, function: F) {
        let half_width = (self.width / 2) as i64;
        let half_height = (self.height / 2) as i64;

        for y in (-half_height + 1)..half_height {
            for x in (-half_width + 1)..half_width {
                self.set_pixel(Position { x, y }, function(Position { x, y }));
            }
        }
    }
}
