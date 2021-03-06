use crate::algebra::linear::scalar::Scalar;
use fructose::operators::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DVector<T> {
    pub data: Vec<T>,
    pub len: usize,
}

impl<T> DVector<T> {
    pub fn new(data: Vec<T>) -> Self {
        let len = data.len();
        Self { data, len }
    }
}

impl<T: Default + Copy> DVector<T> {
    pub fn default_with_size(size: usize) -> Self {
        let data = vec![T::default(); size];
        DVector { data, len: size }
    }
}

impl<T: Scalar + ClosedAdd + ClosedMul> DVector<T> {
    pub fn dot(&self, other: Self) -> T {
        let mut sum = T::default();
        for i in 0..self.len {
            sum += self.data[i];
        }
        sum
    }
}

impl<T: Default + Copy> Default for DVector<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            len: 0,
        }
    }
}

impl<T: Scalar + ClosedAdd> Add for DVector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.len, rhs.len);
        let mut vec = self.clone();
        for i in 0..self.len {
            vec.data[i] += rhs.data[i];
        }
        vec
    }
}

impl<T: Scalar + ClosedAdd> AddAssign for DVector<T> {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.len, rhs.len);
        for i in 0..self.len {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T: Scalar + ClosedSub> Sub for DVector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.len, rhs.len);
        let mut vec = self.clone();
        for i in 0..self.len {
            vec.data[i] -= rhs.data[i];
        }
        vec
    }
}

impl<T: Scalar + ClosedSub> SubAssign for DVector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.len, rhs.len);
        for i in 0..self.len {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<T: Default + Copy> From<DMatrix<T>> for DVector<T> {
    fn from(rhs: DMatrix<T>) -> Self {
        assert_eq!(rhs.size.1, 1);
        DVector {
            data: rhs.data[0].clone(),
            len: rhs.size.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DMatrix<T> {
    pub data: Vec<Vec<T>>,
    pub size: (usize, usize),
}

impl<T> DMatrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let mut size = (0, 0);
        size.1 = data.len();
        if data.len() != 0 {
            size.0 = data.get(0).unwrap().len()
        }

        Self { data, size }
    }
}

impl<T: ToString> DMatrix<T> {
    pub fn to_string_vec(&self) -> DMatrix<String> {
        let data_str: Vec<Vec<String>> = self
            .data
            .iter()
            .map(|col| col.iter().map(|val| val.to_string()).collect())
            .collect();

        DMatrix {
            data: data_str,
            size: self.size,
        }
    }
}

impl<T: Default> Default for DMatrix<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            size: Default::default(),
        }
    }
}

impl<T: Default + Copy> DMatrix<T> {
    pub fn default_with_size(size: (usize, usize)) -> Self {
        let mut data = vec![vec![T::default(); size.0]; size.1];
        Self { data, size }
    }
}

impl<T: Scalar + ClosedAdd> Add for DMatrix<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        let mut mat = self.clone();
        for m in 0..self.size.0 {
            for n in 0..self.size.1 {
                mat.data[n][m] += rhs.data[n][m];
            }
        }
        mat
    }
}

impl<T: Scalar + ClosedAdd> AddAssign for DMatrix<T> {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.size, rhs.size);
        for m in 0..self.size.0 {
            for n in 0..self.size.1 {
                self.data[n][m] += rhs.data[n][m];
            }
        }
    }
}

impl<T: Scalar + ClosedSub> Sub for DMatrix<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        let mut mat = self.clone();
        for m in 0..self.size.0 {
            for n in 0..self.size.1 {
                mat.data[n][m] -= rhs.data[n][m];
            }
        }
        mat
    }
}

impl<T: Scalar + ClosedSub> SubAssign for DMatrix<T> {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.size, rhs.size);
        for m in 0..self.size.0 {
            for n in 0..self.size.1 {
                self.data[n][m] -= rhs.data[n][m];
            }
        }
    }
}

impl<T: Scalar + ClosedAdd + ClosedMul> Mul for DMatrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size.0, rhs.size.1);
        let mut mat = Self::default_with_size((rhs.size.1, self.size.0));
        for m in 0..self.size.0 {
            for p in 0..rhs.size.1 {
                for n in 0..self.size.1 {
                    mat.data[p][m] += self.data[n][m] * rhs.data[p][n];
                }
            }
        }
        mat
    }
}

impl<T: Scalar + ClosedMul> MulAssign for DMatrix<T> {
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.size.0, rhs.size.1);
        for m in 0..self.size.0 {
            for p in 0..rhs.size.1 {
                for n in 0..self.size.1 {
                    self.data[p][m] *= rhs.data[p][n];
                }
            }
        }
    }
}

impl<T: Default + Copy, const M: usize, const N: usize> From<[[T; M]; N]> for DMatrix<T> {
    fn from(rhs: [[T; M]; N]) -> Self {
        let mut mat = Self::default_with_size((M, N));
        for m in 0..M {
            for n in 0..N {
                mat.data[n][m] = rhs[n][m];
            }
        }
        mat
    }
}

impl<T: Default + Copy> From<DVector<T>> for DMatrix<T> {
    fn from(rhs: DVector<T>) -> Self {
        let len = rhs.len;
        DMatrix {
            data: vec![rhs.data],
            size: (len, 0),
        }
    }
}

impl<T: Scalar + ClosedAdd + ClosedMul> Mul<DVector<T>> for DMatrix<T> {
    type Output = DVector<T>;

    fn mul(self, rhs: DVector<T>) -> Self::Output {
        DVector::from(self * DMatrix::from(rhs))
    }
}

impl<T: Scalar + ClosedMul> Mul<T> for DMatrix<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut mat = self.clone();
        for m in 0..self.size.0 {
            for n in 0..self.size.1 {
                mat.data[m][n] *= rhs
            }
        }
        self
    }
}

impl<T: Scalar + ClosedMul> MulAssign<T> for DMatrix<T> {
    fn mul_assign(&mut self, rhs: T) {
        for m in 0..self.size.0 {
            for n in 0..self.size.1 {
                self.data[m][n] *= rhs
            }
        }
    }
}

impl<T: Scalar + ClosedDiv> Div<T> for DMatrix<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut mat = self.clone();
        for m in 0..self.size.0 {
            for n in 0..self.size.1 {
                mat.data[m][n] /= rhs
            }
        }
        self
    }
}

impl<T: Scalar + ClosedDiv> DivAssign<T> for DMatrix<T> {
    fn div_assign(&mut self, rhs: T) {
        for m in 0..self.size.0 {
            for n in 0..self.size.1 {
                self.data[m][n] /= rhs
            }
        }
    }
}

impl<T: Display + Copy> Display for DMatrix<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        let biggest = format!("{}", self.data.get(1).unwrap().get(1).unwrap()).len();
        for m in 0..self.size.0 {
            string.push_str("|");
            for n in 0..self.size.1 {
                if n == self.size.1 - 1 {
                    &string.push_str(&format!("{}", self.data.get(n).unwrap().get(m).unwrap()));
                    break;
                }
                let current = format!("{}", self.data.get(n).unwrap().get(m).unwrap()).len();

                string.push_str(&format!("{} ", self.data.get(n).unwrap().get(m).unwrap()));

                for _ in current..biggest + 1 {
                    string.push(' ');
                }
            }
            &string.push_str("|\n");
        }
        write!(f, "{}", string)
    }
}

impl<T: FromStr + Default> From<&str> for DVector<T> {
    fn from(rhs: &str) -> Self {
        let data = rhs
            .split(" ")
            .map(|val| val.parse::<T>().unwrap_or_else(|t| <T>::default()))
            .collect::<Vec<T>>();

        let len = data.len();
        Self { data, len }
    }
}

impl<T: FromStr + Default> From<String> for DVector<T> {
    fn from(rhs: String) -> Self {
        let data = rhs
            .split(" ")
            .map(|val| val.parse::<T>().unwrap_or_else(|t| <T>::default()))
            .collect::<Vec<T>>();

        let len = data.len();
        Self { data, len }
    }
}

impl<T: FromStr + Default> From<String> for DMatrix<T> {
    fn from(rhs: String) -> Self {
        let cols_str = rhs.split(";").collect::<Vec<&str>>();
        let cols_t = cols_str
            .iter()
            .map(|str| {
                str.split(" ")
                    .map(|val| val.parse::<T>().unwrap_or_else(|t| <T>::default()))
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();

        let size = (cols_t[0].len(), cols_t.len());
        Self { data: cols_t, size }
    }
}

impl<T: FromStr + Default> From<&str> for DMatrix<T> {
    fn from(rhs: &str) -> Self {
        let cols_t = rhs
            .split(";")
            .into_iter()
            .map(|str| {
                str.split(" ")
                    .map(|val| val.parse::<T>().unwrap_or_else(|t| <T>::default()))
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();

        let size = (cols_t[0].len(), cols_t.len());
        Self { data: cols_t, size }
    }
}

#[cfg(test)]
mod dynamic_mat_tests {
    use crate::algebra::linear::dynamic::DMatrix;
    use crate::algebra::linear::dynamic::DVector;

    #[test]
    fn add() {
        let mat1 = DMatrix::new(vec![vec![0.1, 3.0], vec![2.1, 4.0]]);
        let mat2 = DMatrix::new(vec![vec![0.4, 1.0], vec![5.0, -6.0]]);
        let mat3 = mat1 + mat2;
    }

    #[test]
    fn add_assign() {
        let mut mat1 = DMatrix::new(vec![vec![0.1, 3.0], vec![2.1, 4.0]]);
        let mat2 = DMatrix::new(vec![vec![0.4, 1.0], vec![5.0, -6.0]]);
        mat1 += mat2;
    }

    #[test]
    fn mul() {
        let mat1 = DMatrix::new(vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]);
        let mat2 = DMatrix::new(vec![vec![7.0, 9.0, 11.0], vec![8.0, 10.0, 12.0]]);
        let mat3 = mat1 * mat2;
    }

    #[test]
    fn from_arr() {
        let mat2 = DMatrix::from([[7.0, 9.0, 11.0], [8.0, 10.0, 12.0]]);
    }

    #[test]
    fn from_str() {
        let mat1 = DMatrix::<f64>::from("4 3 2;2 2 -1");
        let mat2 = DMatrix::<f64>::from("-2.5 3 2;-2 2.5 3");
        let mat = mat1 + mat2;

        let vec1 = DVector::<f64>::from("4 3 2");
        let vec2 = DVector::<f64>::from("-2.5 3 2");
        let vec = vec1 + vec2;
    }
}
