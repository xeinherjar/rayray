use color::{Color};
use std::fs::OpenOptions;
use std::cmp::{min, max};
use std::path::{Path};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct Canvas {
    pub width: u64,
    pub height: u64,
    pub canvas: Vec<Color>
}

impl Canvas {
    pub fn new(width: u64, height: u64) -> Canvas {
        Canvas{width: width, height: height, canvas: vec![Color::default(); (width * height) as usize]
        }
    }

    pub fn get(self, x: u64, y: u64) -> Color {
        let position = (x * self.height) + y;
        self.canvas[position as usize]
    }

    pub fn put(&mut self, x: u64, y: u64, color: Color) -> () {
        let position = (x * self.height) + y;
        self.canvas[position as usize] = color;
    }

    pub fn save_ppm(self, path: String) -> () {
        let path = Path::new(&path);
        /* P3 is magic byte for PPM files
         * width and height of image
         * max color depth
         */
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);

        /*
         * Clamp colors 0 - 255
         * Lines should not exceed 70 characters
         * Last line is a newline
         * */
        /*
        let mut body = String::new();
        for row in self.canvas.chunks(min(self.width, 70) as usize) {
            let mut _row = String::new();
            for color in row.iter() {
                let r = min(max((color.r * 255.0) as i64, 0), 255);
                let g = min(max((color.g * 255.0) as i64, 0), 255);
                let b = min(max((color.b * 255.0) as i64, 0), 255);
                _row += &format!("{} {} {} ", r, g, b);
            }
            body += _row.trim();
            body += "\n";
        }
        */

        let mut body = String::new();
        let mut row = String::new();
        for (idx, color) in self.canvas.iter().enumerate() {
            let pixels = [
                min(max((color.r * 255.0) as i64, 0), 255),
                min(max((color.g * 255.0) as i64, 0), 255),
                min(max((color.b * 255.0) as i64, 0), 255)
            ];

            for pixel in pixels.iter() {
                let pixel = pixel.to_string();
                if row.len() + pixel.len() >= 70 {
                    body += &row;
                    body += "\n";
                    row = String::new();
                } else {
                    row += " ";
                }
                row += &pixel;
            }

        }
        body += &row;
        body += "\n";

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path).unwrap();

        let ppm = header + &body;
        file.write_all(ppm.as_bytes()).unwrap();
    }
}

mod teets {
    use super::*;

    #[test]
    fn color_get() {
        let cvs = Canvas::new(100, 100);
        let c1 = cvs.get(5, 5);
        let c2 = Color::new(0.0, 0.0, 0.0);
        assert_eq!(c1, c2);
    }

    #[test]
    fn color_put() {
        let mut cvs = Canvas::new(100, 100);
        let c1 = Color::new(0.1, 0.2, 0.3);
        cvs.put(5, 5, c1);
        let c2 = cvs.get(5, 5);
        assert_eq!(c1, c2);
    }

    #[test]
    fn save_ppm() {
        let mut cvs = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, -1.5, 0.0);
        let c3 = Color::new(0.0, 0.0, 1.5);
        cvs.put(0, 0, c1);
        cvs.put(1, 2, c2);
        cvs.put(2, 5, c3);

        cvs.save_ppm("testing.ppm".to_string());
    }
}
