use std::io::Cursor;

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{
    geometry::{Vec2f, Vec3f},
    model::Model,
    util::{maxf, minf, RgbaImageExt},
};

pub fn barycentric(pts: &Vec<Vec3f>, p: Vec3f) -> Vec3f {
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

pub fn triangle(pts: Vec<Vec3f>, zbuf: &mut Vec<f64>, img: &mut RgbaImage, color: Rgba<u8>) {
    let mut bboxmin = Vec2f::from([img.width() as f64 - 1.0, img.height() as f64 - 1.0]);
    let mut bboxmax = Vec2f::from([0.0, 0.0]);
    let clamp = Vec2f::from([img.width() as f64 - 1.0, img.height() as f64 - 1.0]);
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = maxf(0.0, minf(bboxmin[j], pts[i][j]));
            bboxmax[j] = minf(clamp[j], maxf(bboxmax[j], pts[i][j]));
        }
    }
    for x in bboxmin[0] as i32..=bboxmax[0] as i32 {
        for y in bboxmin[1] as i32..=bboxmax[1] as i32 {
            let bc_screen = barycentric(&pts, Vec3f::from([x as f64, y as f64, 0.0]));
            if bc_screen[0] < 0.0 || bc_screen[1] < 0.0 || bc_screen[2] < 0.0 {
                continue;
            }
            let z = pts[0][2] as f64 * bc_screen[0]
                + pts[1][2] as f64 * bc_screen[1]
                + pts[2][2] as f64 * bc_screen[2];
            let idx = (x + y * img.width() as i32) as usize;
            if zbuf[idx] < z {
                zbuf[idx] = z;
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn z_buf() -> Vec<u8> {
    const W: u32 = 800;
    const H: u32 = 800;
    let mut img: RgbaImage = ImageBuffer::new(W, H);
    let model = Model::new("obj/african_head/african_head.obj");

    let light_dir = Vec3f::from([0.0, 0.0, -1.0]);
    let mut zbuf: Vec<f64> = vec![std::f64::MIN; (W * H) as usize];

    for i in 0..model.nfaces() {
        let face = model.face(i);
        let mut screen_coords = Vec::new();
        let mut world_coords = Vec::new();
        for j in 0..3 {
            let v = model.vert(face[j] as usize);
            let x = ((v[0] + 1.0) * W as f64 - 1.0) / 2.0; // -1.0 是为了防止像素坐标越界
            let y = ((v[1] + 1.0) * H as f64 - 1.0) / 2.0;
            screen_coords.push(Vec3f::from([x, y, 0.0]));
            world_coords.push(v);
        }

        let mut n = (*world_coords[2] - *world_coords[0]) ^ (*world_coords[1] - *world_coords[0]);
        n.normalize();
        let intensity = n.dot(&light_dir);
        if intensity > 0.0 {
            triangle(
                screen_coords,
                &mut zbuf,
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
