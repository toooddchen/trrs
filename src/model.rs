use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use image::RgbaImage;

use crate::geometry::{Vec3f, Vec3i};

#[derive(Debug)]
pub struct Model {
    pub verts: Vec<Vec3f>,
    pub faces: Vec<Vec<Vec3i>>, // vertex/uv/normal indexes
    pub norms: Vec<Vec3f>,
    pub uvs: Vec<Vec3f>,
    pub diffuse_map: RgbaImage,
}

impl Model {
    pub fn new(filename: &str) -> Self {
        if let Ok(lines) = Self::read_lines(filename) {
            let mut verts: Vec<Vec3f> = Vec::new();
            let mut faces: Vec<Vec<Vec3i>> = Vec::new();
            let mut norms: Vec<Vec3f> = Vec::new();
            let mut uvs: Vec<Vec3f> = Vec::new();
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
                        let mut uv: Vec3f = Vec3f::new();
                        for i in 0..3 {
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
            let mut model = Self {
                verts,
                faces,
                norms,
                uvs,
                diffuse_map: RgbaImage::new(1, 1),
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

    pub fn face(&self, idx: usize) -> Vec<i32> {
        let mut f = Vec::new();
        // &self.faces[i]
        for i in 0..self.faces[idx].len() {
            f.push(self.faces[idx][i][0]);
        }
        f
    }

    pub fn norm(&self, iface: usize, nthvert: usize) -> Vec3f {
        let idx = self.faces[iface][nthvert][2] as usize;
        let mut r = self.norms[idx];
        r.normalize();
        r
    }

    pub fn load_texture(&mut self, filename: &str) {
        self.diffuse_map = image::open(filename).unwrap().to_rgba8();
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
