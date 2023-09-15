use std::{io::Cursor, rc::Rc};

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{
    geometry::{embed, Vec3f, Vec4f},
    gl::{IShader, GL},
    model::Model,
    util::{maxf, RgbaImageExt},
};

#[derive(Debug)]
pub struct Gouraud6LShader<'a> {
    gl: Rc<&'a GL>,
    varying_intensity: Vec3f,
    model: Rc<&'a Model>,
}

impl<'a> Gouraud6LShader<'a> {
    pub fn new(gl: Rc<&'a GL>, model: Rc<&'a Model>) -> Self {
        Self {
            gl,
            varying_intensity: Vec3f::from([0.0, 0.0, 0.0]),
            model,
        }
    }
}

impl<'a> IShader for Gouraud6LShader<'a> {
    #[inline]
    fn vertex(&mut self, iface: usize, nthvert: usize) -> Vec4f {
        self.varying_intensity[nthvert] =
            maxf(0.0, self.model.norm(iface, nthvert).dot(&self.gl.light_dir));
        let gl_vertex = embed::<_, 4, 3>(self.model.vert_by(iface, nthvert), 1.0);
        let r = &(&(&self.gl.viewport * &self.gl.projection) * &self.gl.model_view) * &gl_vertex;
        Vec4f::from_vec(r)
    }

    #[inline]
    fn fragment(&mut self, bc: Vec3f) -> Rgba<u8> {
        let intensity = match self.varying_intensity.dot(&bc) {
            i if i > 0.85 => 1.0,
            i if i > 0.60 => 0.80,
            i if i > 0.45 => 0.60,
            i if i > 0.30 => 0.45,
            i if i > 0.15 => 0.30,
            _ => 0.0,
        };
        let color = Rgba([
            (255.0 * intensity) as u8,
            (155.0 * intensity) as u8,
            (0.0 * intensity) as u8,
            255,
        ]);
        color
    }
}

pub fn gouraud6l_render() -> Vec<u8> {
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

    let mut shader = Gouraud6LShader::new(Rc::clone(&rcgl), Rc::new(&model));

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
