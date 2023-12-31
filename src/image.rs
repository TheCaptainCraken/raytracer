use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::linear_algebra::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub fn to_vec(self) -> Vec<u8> {
        vec![self.red, self.green, self.blue]
    }

    pub fn to_vec3(self) -> Vector3 {
        Vector3::new(self.red as f64, self.green as f64, self.blue as f64)
    }

    pub fn from_vec3(vector: Vector3) -> Self {
        Self::new(
            vector.x.clamp(0f64, 255f64) as u8,
            vector.y.clamp(0f64, 255f64) as u8,
            vector.z.clamp(0f64, 255f64) as u8,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels: Vec<Color> = Vec::with_capacity(width * height);

        for _ in 0..(width * height) {
            pixels.push(Color::new(30, 30, 30));
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

    pub fn set_pixel(&mut self, pos: Position, pixel: Color) {
        let half_width = (self.width / 2) as i64;
        let half_height = (self.height / 2) as i64;

        if pos.x > half_width || pos.x < -half_width || pos.y > half_height || pos.y < -half_height
        {
            dbg!(pos);
            panic!("Position invalid!");
        } else {
            let x_pos = pos.x + half_width;
            let y_pos = -pos.y + half_height;

            self.pixels[(y_pos * self.width as i64 + x_pos) as usize] = pixel;
        }
    }

    pub fn set_all_pixels<F: Fn(Position) -> Color>(&mut self, function: F) {
        let half_width = (self.width / 2) as i64;
        let half_height = (self.height / 2) as i64;

        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position {
                    x: x as i64 - half_width,
                    y: y as i64 - half_height,
                };
                self.set_pixel(position, function(position));
            }
        }
    }
}
