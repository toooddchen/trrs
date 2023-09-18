use std::{io::Cursor, mem::swap, path};

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{
    geometry::{Vec3f, Vec3i},
    matrix::Mat,
    model::Model,
    util::RgbaImageExt,
};

const DEPTH: f64 = 255.0;

pub fn move_camera() -> Vec<u8> {
    const W: u32 = 800;
    const H: u32 = 800;

    let eye = Vec3f::from([1.0, 1.0, 3.0]);
    let center = Vec3f::from([0.0, 0.0, 0.0]);
    let light_dir = Vec3f::from([1.0, -1.0, 1.0]).normalize();
    let up = Vec3f::from([0.0, 1.0, 0.0]);

    // let mut img: RgbaImage = ImageBuffer::new(W, H);
    let mut img: RgbaImage = ImageBuffer::from_pixel(W, H, Rgba([0, 0, 0, 255]));
    let mut zimg: RgbaImage = ImageBuffer::from_pixel(W, H, Rgba([0, 0, 0, 255]));
    let model = Model::new("obj/african_head/african_head.obj");

    let mut zbuf: Vec<f64> = vec![std::f64::MIN; (W * H) as usize];

    let model_view = lookat(eye, center, up);
    let mut projection: Mat<f64> = Mat::identity(4);
    let viewport = viewport(
        W as i32 / 8,
        H as i32 / 8,
        W as i32 * 3 / 4,
        H as i32 * 3 / 4,
    );
    projection[3][2] = -1.0 / (eye - center).norm();

    println!("ModelView:\n {:}", model_view);
    println!("Viewport:\n {:}", viewport);
    println!("Projection:\n {:}", projection);
    println!("Z:\n {:}", &(&viewport * &projection) * &model_view);

    for i in 0..model.nfaces() {
        let face = model.face(i);
        let mut screen_coords = Vec::new();
        let mut world_coords = Vec::new();
        let mut intensities = Vec::new();
        for j in 0..3 {
            let v = model.vert(face[j] as usize);
            let vv: Vec<f64> = vec![v[0], v[1], v[2], 1.0];
            let p: Vec<f64> = &(&(&viewport * &projection) * &model_view) * &vv;
            screen_coords.push(Vec3f::from([p[0] / p[3], p[1] / p[3], p[2] / p[3]]).to_vec3i());
            world_coords.push(v);
            intensities.push(model.norm(i, j).dot(&light_dir));
        }

        triangle(
            screen_coords[0],
            screen_coords[1],
            screen_coords[2],
            intensities[0],
            intensities[1],
            intensities[2],
            &mut img,
            &mut zbuf,
        );
    }

    for i in 0..W {
        for j in 0..H {
            let c = zbuf[(i + j * W) as usize];
            zimg.put_pixel(i, j, Rgba::from([c as u8, c as u8, c as u8, 255]));
        }
    }
    zimg.flip_vertical();
    zimg.save(path::Path::new("zbuf.png")).unwrap();

    let mut bs: Vec<u8> = Vec::new();
    img.flip_vertical();
    img.write_to(&mut Cursor::new(&mut bs), image::ImageOutputFormat::Png)
        .unwrap();
    bs
}

pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> Mat<f64> {
    let z = (eye - center).normalize();
    println!("lookat(): eye={:?} center={:?} z={:?}", eye, center, z);
    let x = (up ^ z).normalize();
    println!("lookat(): up={:?} z={:?} x={:?}", up, z, x);
    let y = (z ^ x).normalize();
    println!("lookat(): z={:?} x={:?} y={:?}", z, x, y);
    // let mut minv: Mat<f64> = Mat::identity(4);
    // let mut tr: Mat<f64> = Mat::identity(4);
    let mut res: Mat<f64> = Mat::identity(4);
    for i in 0..3 {
        // minv[0][i] = x[i];
        // minv[1][i] = y[i];
        // minv[2][i] = z[i];
        // tr[i][3] = -eye[i];
        res[0][i] = x[i];
        res[1][i] = y[i];
        res[2][i] = z[i];
        res[i][3] = -center[i];
    }
    // &minv * &tr
    res
}

pub fn viewport(x: i32, y: i32, w: i32, h: i32) -> Mat<f64> {
    let mut res: Mat<f64> = Mat::identity(4);
    res[0][3] = x as f64 + w as f64 / 2.0;
    res[1][3] = y as f64 + h as f64 / 2.0;
    res[2][3] = DEPTH / 2.0;
    res[0][0] = w as f64 / 2.0;
    res[1][1] = h as f64 / 2.0;
    res[2][2] = DEPTH / 2.0;
    res
}

pub fn triangle(
    t0: Vec3i,
    t1: Vec3i,
    t2: Vec3i,
    ity0: f64,
    ity1: f64,
    ity2: f64,
    img: &mut RgbaImage,
    zbuf: &mut Vec<f64>,
) {
    if t0[1] == t1[1] && t0[1] == t2[1] {
        return;
    }
    let mut t0 = t0;
    let mut t1 = t1;
    let mut t2 = t2;
    let mut ity0 = ity0;
    let mut ity1 = ity1;
    let mut ity2 = ity2;
    if t0[1] > t1[1] {
        //.y
        swap(&mut t0, &mut t1);
        swap(&mut ity0, &mut ity1);
    }
    if t0[1] > t2[1] {
        swap(&mut t0, &mut t2);
        swap(&mut ity0, &mut ity2);
    }
    if t1[1] > t2[1] {
        swap(&mut t1, &mut t2);
        swap(&mut ity1, &mut ity2);
    }

    let total_height = (t2[1] - t0[1]) as i32;
    for i in 0..total_height {
        let second_half = i > (t1[1] - t0[1]) as i32 || t1[1] == t0[1];
        let segment_height = if second_half {
            t2[1] - t1[1]
        } else {
            t1[1] - t0[1]
        } as i32;
        let alpha = i as f64 / total_height as f64;
        let beta = (i as f64
            - if second_half {
                t1[1] as f64 - t0[1] as f64
            } else {
                0.0
            })
            / segment_height as f64;
        let mut a = (Vec3f::from_vec3i(&t0) + Vec3f::from_vec3i(&(t2 - t0)) * alpha).to_vec3i();
        let mut b = (if second_half {
            Vec3f::from_vec3i(&t1) + Vec3f::from_vec3i(&(t2 - t1)) * beta
        } else {
            Vec3f::from_vec3i(&t0) + Vec3f::from_vec3i(&(t1 - t0)) * beta
        })
        .to_vec3i();
        let mut itya = ity0 + (ity2 - ity0) * alpha;
        let mut ityb = if second_half {
            ity1 + (ity2 - ity1) * beta
        } else {
            ity0 + (ity1 - ity0) * beta
        };
        if a[0] > b[0] {
            swap(&mut a, &mut b);
            swap(&mut itya, &mut ityb);
        }

        for j in a[0] as i32..(b[0] + 1) {
            let phi: f64 = if b[0] == a[0] {
                1.0
            } else {
                (j as f64 - a[0] as f64) / (b[0] as f64 - a[0] as f64)
            };
            let p = (Vec3f::from_vec3i(&a) + Vec3f::from_vec3i(&(b - a)) * phi).to_vec3i();
            let idx = (p[0] + p[1] * img.width() as i32) as usize;
            if p[0] >= img.width() as i32 || p[1] >= img.height() as i32 || p[0] < 0 || p[1] < 0 {
                continue;
            }
            let mut ityp = itya + (ityb - itya) * phi;
            if zbuf[idx] < p[2] as f64 {
                zbuf[idx] = p[2] as f64;
                ityp = if ityp > 1.0 {
                    1.0
                } else if ityp < 0.0 {
                    0.0
                } else {
                    ityp
                };
                let color = (ityp * 255.0) as u8;
                img.put_pixel(p[0] as u32, p[1] as u32, Rgba([color, color, color, 255]));
            }
        }
    }
}
