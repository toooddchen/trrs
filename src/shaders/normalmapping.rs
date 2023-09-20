use std::{io::Cursor, rc::Rc};

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{
    geometry::{embed, proj, Vec2f, Vec3f, Vec4f},
    gl::{IShader, GL},
    matrix::Mat,
    model::Model,
    util::{maxf, RgbaImageExt},
};

#[derive(Debug)]
pub struct NormalMappingShader<'a> {
    gl: Rc<&'a GL>,
    varying_uv: Mat<f64>,   // <2, 3>
    uniform_m: Mat<f64>,    // <4, 4>, Projection * ModelView
    uniform_m_it: Mat<f64>, // <4, 4>, (Projection * ModelView).invert_transpose()
    model: Rc<&'a Model>,
}

impl<'a> NormalMappingShader<'a> {
    pub fn new(gl: Rc<&'a GL>, model: Rc<&'a Model>) -> Self {
        Self {
            gl,
            varying_uv: Mat::new(2, 3),
            uniform_m: Mat::identity(4),
            uniform_m_it: Mat::identity(4),
            model,
        }
    }
}

impl<'a> IShader for NormalMappingShader<'a> {
    fn vertex(&mut self, iface: usize, nthvert: usize) -> Vec4f {
        self.varying_uv
            .set_col(nthvert, &self.model.uv(iface, nthvert).to_vec());
        let gl_vertex = embed::<_, 4, 3>(self.model.vert_by(iface, nthvert), 1.0);
        let r = &(&(&self.gl.viewport * &self.gl.projection) * &self.gl.model_view) * &gl_vertex;
        Vec4f::from_vec(r)
    }

    fn fragment(&mut self, bc: Vec3f, gl_fragcoord: Vec3f) -> Rgba<u8> {
        let uv = &self.varying_uv * &bc;
        let _n = &self.uniform_m
            * &embed::<f64, 4, 3>(&self.model.norm_by(Vec2f::from([uv[0], uv[1]])), 1.0);
        let n = Vec3f::from([_n[0], _n[1], _n[2]]).normalize();
        let _l = &self.uniform_m_it * &embed::<f64, 4, 3>(&self.gl.light_dir, 1.0);
        let l = Vec3f::from([_l[0], _l[1], _l[2]]).normalize();
        let intensity = maxf(0.0, n.dot(&l));
        let color = self.model.diffuse(uv[0], uv[1]);
        let color = Rgba([
            (color[0] as f64 * intensity) as u8,
            (color[1] as f64 * intensity) as u8,
            (color[2] as f64 * intensity) as u8,
            255,
        ]);
        color
    }
}

pub fn normal_mapping_render() -> Vec<u8> {
    const W: u32 = 800;
    const H: u32 = 800;
    let mut img: RgbaImage = ImageBuffer::from_pixel(W, H, Rgba([0, 0, 0, 255]));
    let model = Model::new("obj/african_head/african_head.obj");

    let light_dir = Vec3f::from([1.0, 1.0, 1.0]).normalize();

    let eye = Vec3f::from([1.0, 1.0, 3.0]);
    let center = Vec3f::from([0.0, 0.0, 0.0]);
    let up = Vec3f::from([0.0, 1.0, 0.0]);
    let mut zbuf = vec![f64::MIN; (W * H) as usize];

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

    let mut shader = NormalMappingShader::new(Rc::clone(&rcgl), Rc::new(&model));
    shader.uniform_m = &gl.projection * &gl.model_view;
    shader.uniform_m_it = shader.uniform_m.invert_transpose();

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
        gl.triangle(screen_coords, &mut shader, &mut img, &mut zbuf);
    }
    let mut bs: Vec<u8> = Vec::new();
    img.flip_vertical();
    img.write_to(&mut Cursor::new(&mut bs), image::ImageOutputFormat::Png)
        .unwrap();
    bs
}
