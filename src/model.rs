use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use image::{Rgba, RgbaImage};

use crate::{
    geometry::{Vec2f, Vec2i, Vec3f, Vec3i},
    util::{splitext, RgbaImageExt},
};

#[derive(Debug)]
pub struct Model {
    pub verts: Vec<Vec3f>,
    pub faces: Vec<Vec<Vec3i>>, // vertex/uv/normal indexes
    pub norms: Vec<Vec3f>,
    pub uvs: Vec<Vec2f>,
    pub diffuse_map: RgbaImage,
    pub normal_map: RgbaImage,
    pub specular_map: RgbaImage,
}

impl Model {
    pub fn new(filename: &str) -> Self {
        if let Ok(lines) = Self::read_lines(filename) {
            let mut verts: Vec<Vec3f> = Vec::new();
            let mut faces: Vec<Vec<Vec3i>> = Vec::new();
            let mut norms: Vec<Vec3f> = Vec::new();
            let mut uvs: Vec<Vec2f> = Vec::new();
            for line in lines {
                let mut words = line.split_whitespace();
                match words.next() {
                    Some("v") => {
                        let mut vert: Vec3f = Vec3f::new();
                        for i in 0..3 {
                            vert[i] = words.next().unwrap().parse::<f64>().unwrap();
                        }
                        verts.push(vert);
                    }
                    Some("vn") => {
                        let mut norm: Vec3f = Vec3f::new();
                        for i in 0..3 {
                            norm[i] = words.next().unwrap().parse::<f64>().unwrap();
                        }
                        norms.push(norm);
                    }
                    Some("vt") => {
                        let mut uv = Vec2f::new();
                        for i in 0..2 {
                            uv[i] = words.next().unwrap().parse::<f64>().unwrap();
                        }
                        uvs.push(uv);
                    }
                    Some("f") => {
                        let mut face = Vec::new();
                        let mut tmp = Vec3i::new();
                        for word in words {
                            let mut nums = word.split('/');
                            for i in 0..3 {
                                tmp[i] = nums.next().unwrap().parse::<i32>().unwrap() - 1
                            }
                            face.push(tmp);
                        }
                        faces.push(face);
                    }
                    _ => {}
                }
            }

            let diffusemap = Self::load_texture(filename, "_diffuse.tga");
            let normalmap = Self::load_texture(filename, "_nm.tga");
            let specularmap = Self::load_texture(filename, "_spec.tga");
            let model = Self {
                verts,
                faces,
                norms,
                uvs,
                diffuse_map: diffusemap,
                normal_map: normalmap,
                specular_map: specularmap,
            };
            return model;
        }
        panic!("Failed to read file")
    }

    pub fn nverts(&self) -> usize {
        self.verts.len()
    }

    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    pub fn vert(&self, i: usize) -> &Vec3f {
        &self.verts[i]
    }

    pub fn vert_by(&self, iface: usize, nthvert: usize) -> &Vec3f {
        &self.verts[self.face(iface)[nthvert] as usize]
    }

    pub fn face(&self, idx: usize) -> Vec<i32> {
        let mut f = Vec::new();
        // &self.faces[i]
        for i in 0..self.faces[idx].len() {
            f.push(self.faces[idx][i][0]);
        }
        f
    }

    pub fn uv(&self, iface: usize, nthvert: usize) -> Vec2f {
        let idx = self.faces[iface][nthvert][1] as usize;
        self.uvs[idx]
    }

    pub fn norm(&self, iface: usize, nthvert: usize) -> Vec3f {
        let idx = self.faces[iface][nthvert][2] as usize;
        let mut r = self.norms[idx];
        r.normalize();
        r
    }

    pub fn norm_by(&self, uvf: Vec2f) -> Vec3f {
        let uv = (
            (uvf[0] * self.normal_map.width() as f64) as u32,
            (uvf[1] * self.normal_map.height() as f64) as u32,
        );
        let color = self.normal_map.get_pixel(uv.0, uv.1);
        let mut r = Vec3f::new();
        for i in 0..3 {
            r[2 - i] = (color[i] as f64) / 255.0 * 2.0 - 1.0;
        }
        r
    }

    pub fn diffuse(&self, uv0: f64, uv1: f64) -> Rgba<u8> {
        let uv = (
            (uv0 * self.diffuse_map.width() as f64) as u32,
            (uv1 * self.diffuse_map.height() as f64) as u32,
        );
        let color = self.diffuse_map.get_pixel(uv.0, uv.1);
        *color
    }

    pub fn specular(&self, uv0: f64, uv1: f64) -> f64 {
        let uv = (
            (uv0 * self.specular_map.width() as f64) as u32,
            (uv1 * self.specular_map.height() as f64) as u32,
        );
        let color = self.specular_map.get_pixel(uv.0, uv.1);
        color[0] as f64 / 1.0
    }

    pub fn load_texture(filename: &str, suffix: &str) -> RgbaImage {
        let (name, ext) = splitext(filename);
        let p = name + suffix;
        let mut img = image::open(p).unwrap().to_rgba8();
        img.flip_vertical();
        img
    }

    fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        let buf = BufReader::new(file);
        buf.lines().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_new() {
        let model = Model::new("obj/african_head/african_head.obj");
        assert!(model.nverts() > 0);
        assert!(model.nfaces() > 0);
        assert!(model.norms.len() > 0);
        println!("nverts:{:}, norms:{:}", model.nverts(), model.norms.len());
    }
}
