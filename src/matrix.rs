use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut, Mul},
};

use crate::{geometry::Vec_, util::NumLike};

/// A generic matrix struct with elements of type `T`, with `M` rows and `N` columns.
#[derive(Debug)]
pub struct Mat<T: NumLike> {
    pub M: usize,
    pub N: usize,
    pub data: Vec<Vec<T>>,
}

impl<T: NumLike> Mat<T> {
    pub fn new(m: usize, n: usize) -> Self {
        let data = vec![vec![T::zero(); n]; m];
        Self { M: m, N: n, data }
    }

    pub fn from(data: &Vec<Vec<T>>) -> Self {
        if data.len() == 0 {
            panic!("Cannot create matrix from empty data")
        }
        let mut mat = Self::new(data.len(), data[0].len());
        // println!("mat: {:?} {:?}", mat.M, mat.N);
        for i in 0..mat.M {
            for j in 0..mat.N {
                mat.data[i][j] = data[i][j];
            }
        }
        mat
    }

    pub fn from_vec(m: usize, n: usize, v: Vec<T>) -> Mat<T> {
        if m * n != v.len() {
            panic!("Cannot create matrix from vector of incorrect length");
        }
        let mut mat = Self::new(m, n);
        for i in 0..m {
            for j in 0..n {
                mat.data[i][j] = v[i * n + j];
            }
        }
        mat
    }

    pub fn identity(m: usize) -> Self {
        let mut mat = Self::new(m, m);
        for i in 0..m {
            mat[i][i] = T::one();
        }
        mat
    }

    pub fn col(&self, idx: usize) -> Vec<T> {
        let mut col = vec![T::zero(); self.M];
        for i in 0..self.M {
            col[i] = self.data[i][idx];
        }
        col
    }

    pub fn set_col(&mut self, idx: usize, col: &Vec<T>) {
        for i in 0..self.M {
            self.data[i][idx] = col[i];
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Mat<T> {
        let mut sub: Mat<T> = Mat::new(self.M - 1, self.N - 1);
        for i in 0..self.M - 1 {
            for j in 0..self.N - 1 {
                sub.data[i][j] =
                    self.data[i + if i >= row { 1 } else { 0 }][j + if j >= col { 1 } else { 0 }];
            }
        }
        sub
    }

    /// Calculate the cofactor of the element at row `row` and column `col`.
    pub fn cofactor(&self, row: usize, col: usize) -> T {
        if (row + col) % 2 == 0 {
            self.submatrix(row, col).det()
        } else {
            self.submatrix(row, col).det() * T::from(-1).unwrap()
        }
    }

    pub fn det(&self) -> T {
        if self.M != self.N {
            panic!("Cannot calculate determinant of non-square matrix");
        } else if self.M == 1 {
            self.data[0][0]
        } else if self.M == 2 {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        } else {
            let mut sum = T::zero();
            for i in 0..self.N {
                sum = sum + self.data[0][i] * self.cofactor(0, i);
            }
            sum
        }
    }

    pub fn adjugate(&self) -> Mat<T> {
        let mut adj = Self::new(self.M, self.N);
        for i in 0..self.M {
            for j in 0..self.N {
                adj.data[i][j] = self.cofactor(i, j);
            }
        }
        adj // .transpose()
    }

    pub fn transpose(&self) -> Mat<T> {
        let mut ret = Self::new(self.N, self.M);
        for i in 0..self.N {
            ret[i] = self.col(i);
        }
        ret
    }

    pub fn invert_transpose(&self) -> Mat<T> {
        let det = self.det();
        if det == T::zero() {
            panic!("Cannot invert matrix with zero determinant");
        }
        let adj = self.adjugate();
        let mut ret = Self::new(self.M, self.N);
        for i in 0..self.M {
            for j in 0..self.N {
                ret.data[i][j] = adj.data[i][j] / det;
            }
        }
        ret
    }

    pub fn invert(&self) -> Mat<T> {
        self.invert_transpose().transpose()
    }
}

impl<T: NumLike> Index<usize> for Mat<T> {
    type Output = Vec<T>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl<T: NumLike> IndexMut<usize> for Mat<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

impl<T: NumLike> Mul<T> for &Mat<T> {
    type Output = Mat<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut ret = Mat::new(self.M, self.N);
        for i in 0..self.M {
            for j in 0..self.N {
                ret.data[i][j] = self.data[i][j] * rhs;
            }
        }
        ret
    }
}

impl<T: NumLike> Mul<&Mat<T>> for &Mat<T> {
    type Output = Mat<T>;

    fn mul(self, rhs: &Mat<T>) -> Self::Output {
        if self.N != rhs.M {
            panic!("Cannot multiply matrices of incompatible dimensions");
        }
        let mut ret = Mat::new(self.M, rhs.N);
        for i in 0..self.M {
            for j in 0..rhs.N {
                for k in 0..self.N {
                    ret.data[i][j] = ret.data[i][j] + self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        ret
    }
}

impl<T: NumLike> Mul<&Vec<T>> for &Mat<T> {
    type Output = Vec<T>;

    fn mul(self, rhs: &Vec<T>) -> Self::Output {
        if self.N != rhs.len() {
            panic!(
                "Cannot multiply matrix and vector of incompatible dimensions.{:},{:}",
                self.N,
                rhs.len()
            );
        }
        let mut ret = vec![T::zero(); self.M];
        for i in 0..self.M {
            for j in 0..self.N {
                ret[i] = ret[i] + self.data[i][j] * rhs[j];
            }
        }
        ret
    }
}

impl<T: NumLike, const N: usize> Mul<&Vec_<T, N>> for &Mat<T> {
    type Output = Vec<T>;

    fn mul(self, rhs: &Vec_<T, N>) -> Self::Output {
        if self.N != N {
            panic!(
                "Cannot multiply matrix and vector of incompatible dimensions.{:},{:}",
                self.N, N
            );
        }
        let mut ret = vec![T::zero(); self.M];
        for i in 0..self.M {
            for j in 0..self.N {
                ret[i] = ret[i] + self.data[i][j] * rhs[j];
            }
        }
        ret
    }
}

impl<T: NumLike> PartialEq for Mat<T> {
    fn eq(&self, other: &Self) -> bool {
        self.M == other.M && self.N == other.N && self.data == other.data
    }
}

impl<T: NumLike> Display for Mat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.M {
            for j in 0..self.N {
                write!(f, "{:?}\t", self.data[i][j]).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mat: Mat<i32> = Mat::new(2, 3);
        assert_eq!(mat.data, vec![[0, 0, 0], [0, 0, 0]]);
    }

    #[test]
    fn test_identity() {
        let mat: Mat<i32> = Mat::identity(3);
        assert_eq!(mat.data, vec![[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    }

    #[test]
    fn test_col() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mat: Mat<i32> = Mat::from(&data);
        println!("{:?}", mat.col(0));
        assert_eq!(mat.col(0), [1 as i32, 4 as i32]);
        assert_eq!(mat.col(1), [2 as i32, 5 as i32]);
        assert_eq!(mat.col(2), [3 as i32, 6 as i32]);
    }

    #[test]
    fn test_set_col() {
        let mut mat: Mat<i32> = Mat::new(2, 3);
        mat.set_col(0, &vec![1, 4]);
        mat.set_col(1, &vec![2, 5]);
        mat.set_col(2, &vec![3, 6]);
        assert_eq!(mat.data, [[1, 2, 3], [4, 5, 6]]);
    }

    #[test]
    fn test_submatrix() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mat: Mat<i32> = Mat::from(&data);
        let sub: Mat<i32> = mat.submatrix(1, 1);
        assert_eq!(sub.data, [[1, 3], [7, 9]]);
    }

    #[test]
    fn test_cofactor() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mat: Mat<i32> = Mat::from(&data);
        assert_eq!(mat.cofactor(0, 0), -3);
        assert_eq!(mat.cofactor(0, 1), 6);
        assert_eq!(mat.cofactor(0, 2), -3);
        assert_eq!(mat.cofactor(1, 0), 6);
        assert_eq!(mat.cofactor(1, 1), -12);
        assert_eq!(mat.cofactor(1, 2), 6);
        assert_eq!(mat.cofactor(2, 0), -3);
        assert_eq!(mat.cofactor(2, 1), 6);
        assert_eq!(mat.cofactor(2, 2), -3);
    }

    #[test]
    fn test_det() {
        let data = vec![vec![1, 2], vec![3, 4]];
        let mat: Mat<i32> = Mat::from(&data);
        assert_eq!(mat.det(), -2);

        let data = vec![vec![1, 2], vec![4, 5]];
        let mat: Mat<i32> = Mat::from(&data);
        assert_eq!(mat.det(), -3);

        let data = vec![vec![5, 6], vec![8, 9]];
        let mat: Mat<i32> = Mat::from(&data);
        assert_eq!(mat.det(), -3);

        let data = vec![vec![2, 3], vec![5, 6]];
        let mat: Mat<i32> = Mat::from(&data);
        assert_eq!(mat.det(), -3);

        let data = vec![vec![4, 5], vec![7, 8]];
        let mat: Mat<i32> = Mat::from(&data);
        assert_eq!(mat.det(), -3);

        let data = vec![vec![1, 3], vec![7, 9]];
        let mat: Mat<i32> = Mat::from(&data);
        assert_eq!(mat.det(), -12);

        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mat: Mat<i32> = Mat::from(&data);
        assert_eq!(mat.det(), 0);
    }

    #[test]
    fn test_transpose() {
        let mat = Mat::from(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let transposed = mat.transpose();
        assert_eq!(transposed.M, 3);
        assert_eq!(transposed.N, 2);
        assert_eq!(transposed.data, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }

    #[test]
    fn test_invert_transpose() {
        let mat: Mat<f64> = Mat::from_vec(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 0.0]);
        let inverted_transposed = mat.invert_transpose();
        println!("{:?}", inverted_transposed.data)
    }

    #[test]
    fn test_invert() {
        let mat: Mat<f64> = Mat::from_vec(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 0.0]);
        let inverted = mat.invert();
        println!("{:?}", inverted.data);
    }

    #[test]
    fn test_mul_matrix_vector() {
        let mat: Mat<i32> = Mat::new(2, 3);
        let vec = vec![1, 2, 3];
        let result = &mat * &vec;
        assert_eq!(result, vec![0, 0]);

        let mat: Mat<i32> = Mat::from_vec(2, 2, vec![1, 1, 2, 2]);
        let vec = vec![1, 1];
        let result = &mat * &vec;
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    #[should_panic(expected = "Cannot multiply matrix and vector of incompatible dimensions")]
    fn test_mul_matrix_vector_incompatible_dimensions() {
        let mat: Mat<i32> = Mat::new(2, 3);
        let vec = vec![1, 2];
        let _result = &mat * &vec;
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Mat::from_vec(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Mat::from_vec(3, 2, vec![7, 8, 9, 10, 11, 12]);
        let c = Mat::from_vec(2, 2, vec![58, 64, 139, 154]);
        assert_eq!(&a * &b, c);
    }

    #[test]
    #[should_panic(expected = "Cannot multiply matrices of incompatible dimensions")]
    fn test_matrix_multiplication_incompatible_dimensions() {
        let a = Mat::from_vec(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Mat::from_vec(2, 2, vec![7, 8, 9, 10]);
        let _c = &a * &b;
    }
}
