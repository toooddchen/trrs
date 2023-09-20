use std::{f64::consts::PI, io::Cursor, rc::Rc};

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{
    geometry::{embed, Vec2f, Vec3f, Vec4f},
    gl::{IShader, GL},
    matrix::Mat,
    model::Model,
    util::{maxf, minf, RgbaImageExt},
};

pub struct ZShader<'a> {
    gl: Rc<&'a GL>,
    varying_tri: Mat<f64>, // <4, 3>
    model: Rc<&'a Model>,
}

impl<'a> ZShader<'a> {
    pub fn new(gl: Rc<&'a GL>, model: Rc<&'a Model>) -> Self {
        Self {
            gl,
            varying_tri: Mat::new(4, 3),
            model,
        }
    }
}

impl<'a> IShader for ZShader<'a> {
    fn vertex(&mut self, iface: usize, nthvert: usize) -> Vec4f {
        let gl_vertex = embed::<_, 4, 3>(self.model.vert_by(iface, nthvert), 1.0);
        let r = Vec4f::from_vec(
            &(&(&self.gl.viewport * &self.gl.projection) * &self.gl.model_view) * &gl_vertex,
        );
        self.varying_tri.set_col(nthvert, &(r / r[3]).to_vec());
        r
    }

    fn fragment(&mut self, bc: Vec3f, gl_fragcoord: Vec3f) -> Rgba<u8> {
        Rgba([0, 0, 0, 255])
    }
}

pub fn max_elevation_angle(
    zbuf: &mut Vec<f64>,
    width: u32,
    height: u32,
    p: Vec2f,
    dir: Vec2f,
) -> f64 {
    let mut max_angle = 0.0;
    let mut t = 0.0;
    while t < 1000.0 {
        t += 1.0;
        let cur = p + dir * t;
        if cur[0] >= width as f64 || cur[1] >= height as f64 || cur[0] < 0.0 || cur[1] < 0.0 {
            return max_angle;
        }

        let distance = (p - cur).norm();
        if distance < 1.0 {
            continue;
        }
        let elevation = zbuf[cur[0] as usize + cur[1] as usize * width as usize]
            - zbuf[p[0] as usize + p[1] as usize * width as usize];
        max_angle = maxf(max_angle, f64::atan(elevation / distance));
    }
    max_angle
}

pub fn ambient_occlusion_render() -> Vec<u8> {
    const W: u32 = 800;
    const H: u32 = 800;
    // let model = Model::new("obj/diablo3_pose/diablo3_pose.obj");
    let model = Model::new("obj/african_head/african_head.obj");

    let light_dir = Vec3f::from([1.0, 1.0, 1.0]).normalize();

    let eye = Vec3f::from([1.2, -0.8, 3.0]);
    let center = Vec3f::from([0.0, 0.0, 0.0]);
    let up = Vec3f::from([0.0, 1.0, 0.0]);

    let mut gl = GL::new(light_dir, W, H);
    gl.lookat(eye, center, up);
    gl.viewport(
        W as i32 / 8,
        H as i32 / 8,
        W as i32 * 3 / 4,
        H as i32 * 3 / 4,
    );
    gl.projection(-1.0 / (eye - center).norm());

    let rcgl = Rc::new(&gl);

    println!("ModelView:\n {:}", &gl.model_view);
    println!("Viewport:\n {:}", &gl.viewport);
    println!("Projection:\n {:}", &gl.projection);
    println!(
        "Z:\n {:}",
        &(&gl.viewport * &gl.projection) * &gl.model_view
    );

    // 2-pass
    let mut img: RgbaImage = ImageBuffer::from_pixel(W, H, Rgba([0, 0, 0, 255]));
    let mut zbuf: Vec<f64> = vec![f64::MIN; (W * H) as usize];
    let mut zshader = ZShader::new(Rc::clone(&rcgl), Rc::new(&model));

    for i in 0..model.nfaces() {
        for j in 0..3 {
            zshader.vertex(i, j);
        }
        let pt0 = Vec4f::from_vec(zshader.varying_tri.col(0));
        let pt1 = Vec4f::from_vec(zshader.varying_tri.col(1));
        let pt2 = Vec4f::from_vec(zshader.varying_tri.col(2));
        gl.triangle([pt0, pt1, pt2], &mut zshader, &mut img, &mut zbuf);
    }

    for x in 0..W {
        for y in 0..H {
            if zbuf[x as usize + y as usize * W as usize] < -1e5 {
                continue;
            }

            let mut total = 0.0;
            let mut a = 0.0;
            while a < (PI * 2.0 - 1e-4) {
                total += PI / 2.0
                    - max_elevation_angle(
                        &mut zbuf,
                        W,
                        H,
                        Vec2f::from([x as f64, y as f64]),
                        Vec2f::from([a.cos(), a.sin()]),
                    );
                a += PI / 4.0;
            }
            total /= (PI / 2.0) * 8.0; // pi / 2 * 8
            total = f64::powf(total, 10.0);
            img.put_pixel(
                x,
                y,
                Rgba([
                    (total * 255.0) as u8,
                    (total * 255.0) as u8,
                    (total * 255.0) as u8,
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
