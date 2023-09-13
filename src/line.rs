use std::io::Cursor;

use crate::model::Model;

use crate::util::RgbaImageExt;
use image::{ImageBuffer, Rgba, RgbaImage};

const W: u32 = 801;
const H: u32 = 801;

pub fn wireframe() -> Vec<u8> {
    let mut img: RgbaImage = ImageBuffer::new(W, H);
    let model = Model::new("obj/african_head/african_head.obj");

    for i in 0..model.nfaces() {
        let face = model.face(i);
        for j in 0..3 {
            let v0 = model.vert(face[j] as usize);
            let v1 = model.vert(face[(j + 1) % 3] as usize);
            let x0 = (((v0[0] + 1.0) * W as f64 - 1.0) / 2.0) as i32; // -1.0 是为了防止像素坐标越界
            let y0 = (((v0[1] + 1.0) * H as f64 - 1.0) / 2.0) as i32;
            let x1 = (((v1[0] + 1.0) * W as f64 - 1.0) / 2.0) as i32;
            let y1 = (((v1[1] + 1.0) * H as f64 - 1.0) / 2.0) as i32;
            line(x0, y0, x1, y1, &mut img, Rgba([0, 0, 0, 255]));
        }
    }

    img.flip_vertical();

    let mut bs: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bs), image::ImageOutputFormat::Png)
        .unwrap();
    bs
}

pub fn sample_line() -> Vec<u8> {
    let mut img: RgbaImage = ImageBuffer::new(W, H);
    let mut bs: Vec<u8> = Vec::new();
    line(0, 0, 400, 400, &mut img, Rgba([0, 0, 0, 255]));
    img.write_to(&mut Cursor::new(&mut bs), image::ImageOutputFormat::Png)
        .unwrap();
    bs
}

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32, img: &mut RgbaImage, color: Rgba<u8>) {
    let mut steep = false;
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let mut x1 = x1 as i32;
    let mut y1 = y1 as i32;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror = (dy as f32 / dx as f32).abs();
    let mut error: f32 = 0.0;
    let mut y = y0;
    for x in x0..x1 {
        if steep {
            img.put_pixel(y as u32, x as u32, color);
        } else {
            img.put_pixel(x as u32, y as u32, color);
        }
        error += derror;
        if error > 0.5 {
            y += if y1 > y0 { 1 } else { -1 };
            error -= 1.0;
        }
    }
}
