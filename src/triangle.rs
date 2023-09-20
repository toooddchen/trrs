use std::{cmp, io::Cursor};

use image::{ImageBuffer, Rgba, RgbaImage};
use rand::Rng;

use crate::{
    geometry::{Vec2i, Vec3f},
    model::Model,
    util::RgbaImageExt,
};

pub fn barycentric(pts: &Vec<Vec2i>, p: Vec2i) -> Vec3f {
    let u = Vec3f::from([
        (pts[2][0] - pts[0][0]) as f64,
        (pts[1][0] - pts[0][0]) as f64,
        (pts[0][0] - p[0]) as f64,
    ]) ^ (Vec3f::from([
        (pts[2][1] - pts[0][1]) as f64,
        (pts[1][1] - pts[0][1]) as f64,
        (pts[0][1] - p[1]) as f64,
    ]));
    if u[2].abs() < 1.0 {
        return Vec3f::from([-1.0, 1.0, 1.0]);
    }
    return Vec3f::from([1.0 - (u[0] + u[1]) / u[2], u[1] / u[2], u[0] / u[2]]);
}

pub fn triangle(pts: Vec<Vec2i>, img: &mut RgbaImage, color: Rgba<u8>) {
    let mut bboxmin = Vec2i::from([img.width() as i32 - 1, img.height() as i32 - 1]);
    let mut bboxmax = Vec2i::from([0, 0]);
    let clamp = Vec2i::from([img.width() as i32 - 1, img.height() as i32 - 1]);
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = cmp::max(0, cmp::min(bboxmin[j], pts[i][j]));
            bboxmax[j] = cmp::min(clamp[j], cmp::max(bboxmax[j], pts[i][j]));
        }
    }
    // println!("bboxmin: {:?}, bboxmax: {:?}", bboxmin, bboxmax);
    for x in bboxmin[0]..=bboxmax[0] {
        for y in bboxmin[1]..=bboxmax[1] {
            let bc_screen = barycentric(&pts, Vec2i::from([x, y]));
            if bc_screen[0] < 0.0 || bc_screen[1] < 0.0 || bc_screen[2] < 0.0 {
                continue;
            }
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}

pub fn sample_triangle() -> Vec<u8> {
    const W: u32 = 200;
    const H: u32 = 200;
    let mut img: RgbaImage = ImageBuffer::new(W, H);
    triangle(
        vec![
            Vec2i::from([10, 10]),
            Vec2i::from([100, 30]),
            Vec2i::from([190, 160]),
        ],
        &mut img,
        Rgba([255, 0, 0, 255]),
    );
    let mut bs: Vec<u8> = Vec::new();
    img.flip_vertical();
    img.write_to(&mut Cursor::new(&mut bs), image::ImageOutputFormat::Png)
        .unwrap();
    bs
}

pub fn flat_shading() -> Vec<u8> {
    const W: u32 = 800;
    const H: u32 = 800;
    let mut img: RgbaImage = ImageBuffer::new(W, H);
    let model = Model::new("obj/african_head/african_head.obj");

    let mut rng = rand::thread_rng();

    for i in 0..model.nfaces() {
        let face = model.face(i);
        let mut screen_coords = Vec::new();
        for j in 0..3 {
            let v = model.vert(face[j] as usize);
            let x = (((v[0] + 1.0) * W as f64 - 1.0) / 2.0) as i32; // -1.0 是为了防止像素坐标越界
            let y = (((v[1] + 1.0) * H as f64 - 1.0) / 2.0) as i32;
            screen_coords.push(Vec2i::from([x, y]));
        }
        triangle(
            screen_coords,
            &mut img,
            Rgba([
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
                255,
            ]),
        );
    }
    let mut bs: Vec<u8> = Vec::new();
    img.flip_vertical();
    img.write_to(&mut Cursor::new(&mut bs), image::ImageOutputFormat::Png)
        .unwrap();
    bs
}

pub fn linear_light() -> Vec<u8> {
    const W: u32 = 800;
    const H: u32 = 800;
    let mut img: RgbaImage = ImageBuffer::new(W, H);
    let model = Model::new("obj/african_head/african_head.obj");

    let light_dir = Vec3f::from([0.0, 0.0, -1.0]);

    for i in 0..model.nfaces() {
        let face = model.face(i);
        let mut screen_coords = Vec::new();
        let mut world_coords = Vec::new();
        for j in 0..3 {
            let v = model.vert(face[j] as usize);
            let x = (((v[0] + 1.0) * W as f64 - 1.0) / 2.0) as i32; // -1.0 是为了防止像素坐标越界
            let y = (((v[1] + 1.0) * H as f64 - 1.0) / 2.0) as i32;
            screen_coords.push(Vec2i::from([x, y]));
            world_coords.push(v);
        }

        let mut n = (*world_coords[2] - *world_coords[0]) ^ (*world_coords[1] - *world_coords[0]);
        n.normalize();
        let intensity = n.dot(&light_dir);
        if intensity > 0.0 {
            triangle(
                screen_coords,
                &mut img,
                Rgba([
                    (intensity * 255.0) as u8,
                    (intensity * 255.0) as u8,
                    (intensity * 255.0) as u8,
                    255,
                ]),
            );
        }
    }
    let mut bs: Vec<u8> = Vec::new();
    img.flip_vertical();
    img.write_to(&mut Cursor::new(&mut bs), image::ImageOutputFormat::Png)
        .unwrap();
    bs
}
