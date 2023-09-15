use std::{io::Cursor, rc::Rc};

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{
    geometry::{embed, Vec2f, Vec3f, Vec4f},
    gl::{IShader, GL},
    matrix::Mat,
    model::Model,
    util::{maxf, RgbaImageExt},
};

#[derive(Debug)]
pub struct TextureShader<'a> {
    gl: Rc<&'a GL>,
    varying_intensity: Vec3f,
    varying_uv: Mat<f64>, // <2, 3>
    model: Rc<&'a Model>,
}

impl<'a> TextureShader<'a> {
    pub fn new(gl: Rc<&'a GL>, model: Rc<&'a Model>) -> Self {
        Self {
            gl,
            varying_intensity: Vec3f::from([0.0, 0.0, 0.0]),
            varying_uv: Mat::new(2, 3),
            model,
        }
    }
}

impl<'a> IShader for TextureShader<'a> {
    fn vertex(&mut self, iface: usize, nthvert: usize) -> Vec4f {
        self.varying_intensity[nthvert] =
            maxf(0.0, self.model.norm(iface, nthvert).dot(&self.gl.light_dir));
        self.varying_uv
            .set_col(nthvert, &self.model.uv(iface, nthvert).to_vec());
        let gl_vertex = embed::<_, 4, 3>(self.model.vert_by(iface, nthvert), 1.0);
        let r = &(&(&self.gl.viewport * &self.gl.projection) * &self.gl.model_view) * &gl_vertex;
        Vec4f::from_vec(r)
    }

    fn fragment(&mut self, bc: Vec3f) -> Rgba<u8> {
        let intensity = self.varying_intensity.dot(&bc);
        let uv = &self.varying_uv * &bc;
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

pub fn texture_render() -> Vec<u8> {
    const W: u32 = 800;
    const H: u32 = 800;
    let mut img: RgbaImage = ImageBuffer::from_pixel(W, H, Rgba([0, 0, 0, 255]));
    let model = Model::new("obj/african_head/african_head.obj");

    let light_dir = Vec3f::from([1.0, 1.0, 1.0]).normalize();

    let eye = Vec3f::from([1.0, 1.0, 3.0]);
    let center = Vec3f::from([0.0, 0.0, 0.0]);
    let up = Vec3f::from([0.0, 1.0, 0.0]);
    let mut zbuf: Vec<u8> = vec![0; (W * H) as usize];

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

    let mut shader = TextureShader::new(Rc::clone(&rcgl), Rc::new(&model));

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
