use std::fmt::Debug;

use image::{Rgba, RgbaImage};
use num_traits::{Float, NumCast, NumOps, One, Zero};

pub trait NumLike: PartialEq + NumOps + NumCast + Zero + One + Copy + Debug {}

impl NumLike for f64 {}
impl NumLike for f32 {}
impl NumLike for i32 {}

pub fn maxf<T: Float>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn minf<T: Float>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub trait RgbaImageExt {
    fn flip_horizontal(&mut self);
    fn flip_vertical(&mut self);
}

impl RgbaImageExt for RgbaImage {
    fn flip_horizontal(&mut self) {
        let width = self.width();
        let height = self.height();
        let mut temp_row = vec![Rgba([0, 0, 0, 0]); width as usize];
        for y in 0..height {
            for x in 0..width / 2 {
                let left = self.get_pixel(x, y);
                let right = self.get_pixel(width - x - 1, y);
                temp_row[x as usize] = *right;
                temp_row[(width - x - 1) as usize] = *left;
            }
            for x in 0..width {
                self.put_pixel(x, y, temp_row[x as usize]);
            }
        }
    }

    fn flip_vertical(&mut self) {
        let width = self.width();
        let height = self.height();
        let mut temp_col = vec![Rgba([0, 0, 0, 0]); height as usize];
        for x in 0..width {
            for y in 0..height / 2 {
                let top = self.get_pixel(x, y);
                let bottom = self.get_pixel(x, height - y - 1);
                temp_col[y as usize] = *bottom;
                temp_col[(height - y - 1) as usize] = *top;
            }
            for y in 0..height {
                self.put_pixel(x, y, temp_col[y as usize]);
            }
        }
    }
}
