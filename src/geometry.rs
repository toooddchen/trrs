use std::{fmt::Display, ops};

use crate::util::NumLike;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec_<T: NumLike, const N: usize> {
    pub data: [T; N],
}

pub type Vec2<T> = Vec_<T, 2>;
pub type Vec3<T> = Vec_<T, 3>;
pub type Vec4<T> = Vec_<T, 4>;
pub type Vec2f = Vec2<f64>;
pub type Vec3f = Vec3<f64>;
pub type Vec4f = Vec4<f64>;
pub type Vec2i = Vec2<i32>;
pub type Vec3i = Vec3<i32>;

/// Embed a vector into a larger vector
pub fn embed<T: NumLike, const N1: usize, const N2: usize>(
    v: &Vec_<T, N2>,
    fill: T,
) -> Vec_<T, N1> {
    let mut rs = Vec_::<T, N1>::new();
    for i in 0..N1 {
        if i < N2 {
            rs[i] = v[i];
        } else {
            rs[i] = fill;
        }
    }
    rs
}

/// Project a vector into a smaller vector
pub fn proj<T: NumLike, const N1: usize, const N2: usize>(v: &Vec_<T, N2>) -> Vec_<T, N1> {
    let mut rs = Vec_::<T, N1>::new();
    for i in 0..N1 {
        rs[i] = v[i];
    }
    rs
}

impl<T: NumLike, const N: usize> Vec_<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::zero(); N],
        }
    }

    pub fn from(arr: [T; N]) -> Self {
        Self { data: arr }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        assert!(vec.len() >= N);
        let mut rs = Self::new();
        for i in 0..N {
            rs[i] = vec[i];
        }
        rs
    }

    pub fn zero() -> Self {
        Self {
            data: [T::zero(); N],
        }
    }

    pub fn dot(&self, other: &Self) -> T {
        let mut sum = T::zero();
        for i in 0..N {
            sum = sum + self[i] * other[i];
        }
        sum
    }

    pub fn norm(&self) -> f64 {
        let sum = self.dot(self).to_f64().unwrap();
        sum.sqrt()
    }

    pub fn normalize(&mut self) -> Self {
        let norm = self.norm();
        for i in 0..N {
            self[i] = self[i] / T::from(norm).unwrap();
        }
        return *self;
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut rs = Vec::with_capacity(N);
        for i in 0..N {
            rs.push(self[i]);
        }
        rs
    }
}

impl<T: NumLike, const N: usize> ops::Add for Vec_<T, N> {
    type Output = Vec_<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut rs: Vec_<T, N> = Vec_::new();
        for i in 0..N {
            rs[i] = self[i] + rhs[i];
        }
        rs
    }
}

impl<T: NumLike, const N: usize> ops::Sub for Vec_<T, N> {
    type Output = Vec_<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut rs: Vec_<T, N> = Vec_::new();
        for i in 0..N {
            rs[i] = self[i] - rhs[i];
        }
        rs
    }
}

pub fn cross<T: NumLike>(v0: &Vec3<T>, v1: &Vec3<T>) -> Vec3<T> {
    Vec3::<T>::from([
        v0[1] * v1[2] - v0[2] * v1[1],
        v0[2] * v1[0] - v0[0] * v1[2],
        v0[0] * v1[1] - v0[1] * v1[0],
    ])
}

impl<T: NumLike, const N: usize> ops::Mul<Vec_<T, N>> for Vec_<T, N> {
    type Output = Vec_<T, N>;

    fn mul(self, rhs: Vec_<T, N>) -> Self::Output {
        let mut rs: Vec_<T, N> = Vec_::new();
        for i in 0..N {
            rs[i] = self[i] * rhs[i];
        }
        rs
    }
}

impl<T: NumLike, const N: usize> ops::Mul<T> for Vec_<T, N> {
    type Output = Vec_<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut rs: Vec_<T, N> = Vec_::new();
        for i in 0..N {
            rs[i] = self[i] * rhs;
        }
        rs
    }
}

impl<T: NumLike, const N: usize> ops::Div for Vec_<T, N> {
    type Output = Vec_<T, N>;

    fn div(self, rhs: Self) -> Self::Output {
        let mut rs: Vec_<T, N> = Vec_::new();
        for i in 0..N {
            rs[i] = self[i] / rhs[i];
        }
        rs
    }
}

impl<T: NumLike, const N: usize> ops::Div<T> for Vec_<T, N> {
    type Output = Vec_<T, N>;

    fn div(self, rhs: T) -> Self::Output {
        let mut rs: Vec_<T, N> = Vec_::new();
        for i in 0..N {
            rs[i] = self[i] / rhs;
        }
        rs
    }
}

impl<T: NumLike, const N: usize> ops::Index<usize> for Vec_<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: NumLike, const N: usize> ops::IndexMut<usize> for Vec_<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: NumLike> ops::BitXor for Vec3<T> {
    type Output = Vec3<T>;

    // bitxor is cross product
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut rs: Vec3<T> = Vec3::new();
        rs[0] = self[1] * rhs[2] - self[2] * rhs[1];
        rs[1] = self[2] * rhs[0] - self[0] * rhs[2];
        rs[2] = self[0] * rhs[1] - self[1] * rhs[0];
        rs
    }
}

impl<T: NumLike, const N: usize> Display for Vec_<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Vec3f {
    pub fn to_vec3i(&self) -> Vec3i {
        Vec3i::from([
            (self[0] + 0.5) as i32,
            (self[1] + 0.5) as i32,
            (self[2] + 0.5) as i32,
        ])
    }

    pub fn from_vec3i(v: &Vec3i) -> Vec3f {
        Vec3f::from([v[0] as f64, v[1] as f64, v[2] as f64])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v: Vec_<f64, 3> = Vec_::new();
        assert_eq!(v.data, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec_ {
            data: [1.0, 2.0, 3.0],
        };
        let v2 = Vec_ {
            data: [4.0, 5.0, 6.0],
        };
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_norm() {
        let v = Vec_ {
            data: [1.0, 2.0, 2.0],
        };
        assert_eq!(v.norm(), 3.0);
    }

    #[test]
    fn test_normalize() {
        let mut v = Vec_ {
            data: [1.0, 2.0, 2.0],
        };
        v.normalize();
        assert_eq!(v.data, [1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0]);
    }

    #[test]
    fn test_add() {
        let v1 = Vec_ {
            data: [1.0, 2.0, 3.0],
        };
        let v2 = Vec_ {
            data: [4.0, 5.0, 6.0],
        };
        let v3 = Vec_ {
            data: [5.0, 7.0, 9.0],
        };
        assert_eq!(v1 + v2, v3);
    }
    #[test]
    fn test_bitxor() {
        let v1 = Vec3 {
            data: [1.0, 2.0, 3.0],
        };
        let v2 = Vec3 {
            data: [4.0, 5.0, 6.0],
        };
        let expected = Vec3 {
            data: [-3.0, 6.0, -3.0],
        };
        assert_eq!(v1 ^ v2, expected);
    }
}
