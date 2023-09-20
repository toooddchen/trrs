use std::{cmp, io::Cursor, rc::Rc};

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{
    geometry::{embed, proj, Vec2f, Vec3f, Vec4f},
    gl::{IShader, GL},
    matrix::Mat,
    model::Model,
    util::{maxf, minf, RgbaImageExt},
};

const DEPTH: f64 = 255.0;

pub struct DepthShader<'a> {
    gl: Rc<&'a GL>,
    varying_tri: Mat<f64>, // <3, 3>
    model: Rc<&'a Model>,
}

#[derive(Debug)]
pub struct Pass2Shader<'a> {
    gl: Rc<&'a GL>,
    varying_uv: Mat<f64>,       // <2, 3>
    varying_tri: Mat<f64>,      // <3, 3>
    uniform_m: Mat<f64>,        // <4, 4>, Projection * ModelView
    uniform_m_it: Mat<f64>,     // <4, 4>, (Projection * ModelView).invert_transpose()
    uniform_m_shadow: Mat<f64>, // <4, 4>
    shadown_buffer: &'a mut Vec<u8>,
    model: Rc<&'a Model>,
}

impl<'a> DepthShader<'a> {
    pub fn new(gl: Rc<&'a GL>, model: Rc<&'a Model>) -> Self {
        Self {
            gl,
            varying_tri: Mat::new(3, 3),
            model,
        }
    }
}

impl<'a> IShader for DepthShader<'a> {
    fn vertex(&mut self, iface: usize, nthvert: usize) -> Vec4f {
        let gl_vertex = embed::<_, 4, 3>(self.model.vert_by(iface, nthvert), 1.0);
        let r = Vec4f::from_vec(
            &(&(&self.gl.viewport * &self.gl.projection) * &self.gl.model_view) * &gl_vertex,
        );
        self.varying_tri.set_col(nthvert, &(r / r[3]).to_vec());
        r
    }

    fn fragment(&mut self, bc: Vec3f, gl_fragcoord: Vec3f) -> Rgba<u8> {
        let p = &self.varying_tri * &bc;
        let intensity = p[2] / DEPTH;
        Rgba([
            (255.0 * intensity) as u8,
            (255.0 * intensity) as u8,
            (255.0 * intensity) as u8,
            255,
        ])
    }
}

impl<'a> Pass2Shader<'a> {
    pub fn new(gl: Rc<&'a GL>, model: Rc<&'a Model>, shadow_buffer: &'a mut Vec<u8>) -> Self {
        Self {
            gl,
            varying_uv: Mat::new(2, 3),
            varying_tri: Mat::identity(3),
            uniform_m: Mat::identity(4),
            uniform_m_it: Mat::identity(4),
            uniform_m_shadow: Mat::identity(4),
            shadown_buffer: shadow_buffer,
            model,
        }
    }
}

impl<'a> IShader for Pass2Shader<'a> {
    fn vertex(&mut self, iface: usize, nthvert: usize) -> Vec4f {
        self.varying_uv
            .set_col(nthvert, &self.model.uv(iface, nthvert).to_vec());
        let gl_vertex = embed::<_, 4, 3>(self.model.vert_by(iface, nthvert), 1.0);
        let r = Vec4f::from_vec(
            &(&(&self.gl.viewport * &self.gl.projection) * &self.gl.model_view) * &gl_vertex,
        );
        self.varying_tri.set_col(nthvert, &(r / r[3]).to_vec()); // 4 -> 3 proj
        r
    }

    fn fragment(&mut self, bc: Vec3f, gl_fragcoord: Vec3f) -> Rgba<u8> {
        let mut sb_p = Vec4f::from_vec(
            &self.uniform_m_shadow
                * &embed::<f64, 4, 3>(&Vec3f::from_vec(&self.varying_tri * &bc), 1.0),
        );
        sb_p = sb_p / sb_p[3];
        let idx = (sb_p[0] + sb_p[1] * self.gl.width as f64) as usize;
        let shadow = 0.3
            + 0.7
                * if (self.shadown_buffer[idx] as f64) < sb_p[2] {
                    1.0
                } else {
                    0.0
                };

        let uv = &self.varying_uv * &bc;
        let _n = &self.uniform_m
            * &embed::<f64, 4, 3>(&self.model.norm_by(Vec2f::from([uv[0], uv[1]])), 1.0);
        let n = Vec3f::from_vec(_n).normalize();
        let _l = &self.uniform_m_it * &embed::<f64, 4, 3>(&self.gl.light_dir, 1.0);
        let l = Vec3f::from_vec(_l).normalize();
        let r = (n * ((n.dot(&l)) * 2.0) - l).normalize(); // 注意n*l是dot, 不是*
        let spec = f64::powf(maxf(r[2], 0.0), self.model.specular(uv[0], uv[1]));
        let diff = maxf(0.0, n.dot(&l));
        let color = self.model.diffuse(uv[0], uv[1]);
        let mut rs = [0; 3];
        for i in 0..3 {
            rs[i] = minf(
                20.0 + (color[i] as f64 * shadow * (1.2 * diff + 0.6 * spec)),
                255.0,
            ) as u8;
        }
        Rgba([rs[0], rs[1], rs[2], 255])
    }
}

pub fn shadow_mapping_render() -> Vec<u8> {
    const W: u32 = 800;
    const H: u32 = 800;
    let model = Model::new("obj/diablo3_pose/diablo3_pose.obj");

    let light_dir = Vec3f::from([1.0, 1.0, 1.0]).normalize();

    let eye = Vec3f::from([1.0, 1.0, 3.0]);
    let center = Vec3f::from([0.0, 0.0, 0.0]);
    let up = Vec3f::from([0.0, 1.0, 0.0]);

    let mut shadowbuffer: Vec<u8> = vec![0; (W * H) as usize];

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

    let mut shader = DepthShader::new(Rc::clone(&rcgl), Rc::new(&model));

    println!("ModelView:\n {:}", &gl.model_view);
    println!("Viewport:\n {:}", &gl.viewport);
    println!("Projection:\n {:}", &gl.projection);
    println!(
        "Z:\n {:}",
        &(&gl.viewport * &gl.projection) * &gl.model_view
    );

    for i in 0..model.nfaces() {
        let mut screen_coords: [Vec4f; 3] = [Vec4f::from([0.0, 0.0, 0.0, 0.0]); 3];
        for j in 0..3 {
            screen_coords[j] = shader.vertex(i, j);
        }
        // gl.triangle(screen_coords, &mut shader, &mut depth, &mut shadowbuffer);
    }

    // 2-pass
    let mut img: RgbaImage = ImageBuffer::from_pixel(W, H, Rgba([0, 0, 0, 255]));
    let mut zbuf = vec![f64::MIN; (W * H) as usize];

    let mut p2shader = Pass2Shader::new(Rc::clone(&rcgl), Rc::new(&model), &mut shadowbuffer);

    for i in 0..model.nfaces() {
        let mut screen_coords: [Vec4f; 3] = [Vec4f::from([0.0, 0.0, 0.0, 0.0]); 3];
        for j in 0..3 {
            screen_coords[j] = p2shader.vertex(i, j);
        }
        gl.triangle(screen_coords, &mut p2shader, &mut img, &mut zbuf);
    }

    let mut bs: Vec<u8> = Vec::new();
    img.flip_vertical();
    img.write_to(&mut Cursor::new(&mut bs), image::ImageOutputFormat::Png)
        .unwrap();
    bs
}
