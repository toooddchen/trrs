use image::{Rgba, RgbaImage};

use crate::{
    camera::lookat,
    geometry::{embed, proj, Vec2f, Vec3f, Vec4f},
    matrix::Mat,
    util::{maxf, minf},
};

const DEPTH: f64 = 255.0;

#[derive(Debug)]
pub struct GL {
    pub model_view: Mat<f64>,
    pub projection: Mat<f64>,
    pub viewport: Mat<f64>,

    pub light_dir: Vec3f,
    pub width: u32,
    pub height: u32,
}

pub trait IShader {
    fn vertex(&mut self, iface: usize, nthvert: usize) -> Vec4f;
    fn fragment(&mut self, bc: Vec3f, gl_fragcoord: Vec3f) -> Rgba<u8>;
}

pub fn barycentric(pts: [Vec2f; 3], p: Vec2f) -> Vec3f {
    let abc: Mat<f64> = Mat::from(&vec![
        embed::<f64, 3, 2>(&pts[0], 1.0).to_vec(),
        embed::<f64, 3, 2>(&pts[1], 1.0).to_vec(),
        embed::<f64, 3, 2>(&pts[2], 1.0).to_vec(),
    ]);
    if abc.det() < 1e-3 {
        return Vec3f::from([-1.0, 1.0, 1.0]);
    }
    return Vec3f::from_vec(&(abc.invert_transpose()) * &embed::<f64, 3, 2>(&p, 1.0));
}

impl GL {
    pub fn new(light_dir: Vec3f, w: u32, h: u32) -> Self {
        Self {
            model_view: Mat::identity(4),
            projection: Mat::identity(4),
            viewport: Mat::identity(4),
            light_dir: light_dir,
            width: w,
            height: h,
        }
    }

    pub fn lookat(&mut self, eye: Vec3f, center: Vec3f, up: Vec3f) {
        self.model_view = lookat(eye, center, up);
    }

    pub fn projection(&mut self, coeff: f64) {
        self.projection[3][2] = coeff;
    }

    pub fn viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.viewport = Mat::from(&vec![
            vec![w as f64 / 2.0, 0.0, 0.0, x as f64 + w as f64 / 2.0],
            vec![0.0, h as f64 / 2.0, 0.0, y as f64 + h as f64 / 2.],
            vec![0.0, 0.0, DEPTH / 2.0, DEPTH / 2.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn triangle(
        &self,
        pts: [Vec4f; 3],
        shader: &mut impl IShader,
        img: &mut RgbaImage,
        zbuf: &mut Vec<f64>,
    ) {
        let pts2: [Vec2f; 3] = [
            proj::<_, 2, 4>(&(pts[0] / pts[0][3])),
            proj::<_, 2, 4>(&(pts[1] / pts[1][3])),
            proj::<_, 2, 4>(&(pts[2] / pts[2][3])),
        ];

        let mut bboxmin = Vec2f::from([std::f64::MAX, std::f64::MAX]);
        let mut bboxmax = Vec2f::from([-std::f64::MAX, -std::f64::MAX]);
        for i in 0..3 {
            for j in 0..2 {
                bboxmin[j] = minf(bboxmin[j], pts[i][j] / pts[i][3]);
                bboxmax[j] = maxf(bboxmax[j], pts[i][j] / pts[i][3]);
            }
        }

        let mut p = Vec2f::new();
        for x in bboxmin[0] as i32..=bboxmax[0] as i32 {
            for y in bboxmin[1] as i32..=bboxmax[1] as i32 {
                p[0] = x as f64;
                p[1] = y as f64;
                let bc_screen = barycentric(pts2, p);
                let z =
                    pts[0][2] * bc_screen[0] + pts[1][2] * bc_screen[1] + pts[2][2] * bc_screen[2];
                let w =
                    pts[0][3] * bc_screen[0] + pts[1][3] * bc_screen[1] + pts[2][3] * bc_screen[2];
                let frag_depth = maxf(0.0, minf(255.0, z / w + 0.5));
                let idx = (x + y * img.width() as i32) as usize;
                if bc_screen[0] < 0.0
                    || bc_screen[1] < 0.0
                    || bc_screen[2] < 0.0
                    || zbuf[idx] > frag_depth
                {
                    continue;
                }

                let color = shader.fragment(bc_screen, Vec3f::ZERO);
                zbuf[idx] = frag_depth;
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

impl Drop for GL {
    fn drop(&mut self) {
        // some debug outputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barycentric() {
        let pts = [
            Vec2f::from([0.0, 0.0]),
            Vec2f::from([1.0, 0.0]),
            Vec2f::from([0.0, 1.0]),
        ];
        let p = Vec2f::from([0.5, 0.5]);
        let bary = barycentric(pts, p);
        assert_eq!(bary, Vec3f::from([0.0, 0.5, 0.5]));
    }
}
