use crate::algebra::linear::scalar::Scalar;
use crate::numeric::cmp::Cmp;
use crate::numeric::sign::Signed;
use std::alloc::Layout;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

pub type SquareMatrix<T, const N: usize> = Matrix<T, { N }, { N }>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub data: [[T; M]; N],
}

impl<T: Default + Copy, const M: usize, const N: usize> Default for Matrix<T, { M }, { N }> {
    #[inline]
    fn default() -> Self {
        Self {
            data: [[T::default(); M]; N],
        }
    }
}

impl<T: Display, const M: usize, const N: usize> Display for Matrix<T, { M }, { N }> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for m in 0..M {
            &string.push_str("|");
            for n in 0..N {
                if n == N - 1 {
                    &string.push_str(&format!("{}", self[[n, m]]));
                    break;
                }
                &string.push_str(&format!("{} ", self[[n, m]]));
            }
            &string.push_str("|\n");
        }
        write!(f, "{}", string)
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]

    pub const fn new(data: [[T; M]; N]) -> Self {
        Self { data }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        M * N
    }

    #[inline]
    pub const fn size(&self) -> (usize, usize) {
        (M, N)
    }

    #[inline]
    pub fn layout() -> Layout {
        Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<[T; M]>())
            .unwrap()
    }

    #[inline]
    pub fn as_array(&self) -> &[T; N] {
        use std::convert::TryInto;
        self.as_slice().try_into().unwrap()
    }

    #[inline]
    pub fn as_array_mut(&mut self) -> &mut [T; N] {
        use std::convert::TryInto;
        self.as_slice_mut().try_into().unwrap()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // this is safe because the underlying data structure of a matrix has length M * N
        unsafe { std::slice::from_raw_parts(self as *const Self as *const T, M * N) }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut Self as *mut T, M * N) }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self as *const Self as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut Self as *mut T
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                M * N * std::mem::size_of::<Self>(),
            )
        }
    }
}

impl<T: Default + Copy, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn to_vectors(&self) -> [[T; M]; N] {
        self.data
    }

    #[inline]
    pub fn broadcast(value: T) -> Self {
        Self {
            data: [[value; M]; N],
        }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn map<F: Fn(T) -> T>(&self, f: F) -> Self {
        let mut vector = *self;
        vector
            .data
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = f(*e)));
        vector
    }

    #[inline]
    pub fn apply<F: Fn(T) -> T>(&mut self, f: F) {
        self.data
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = f(*e)));
    }

    /// constructs a new VecN with all values being 0
    #[inline]
    pub fn zero() -> Self {
        Self::broadcast(<T>::zero())
    }

    /// constructs a new VecN with all values being 1
    #[inline]
    pub fn one() -> Self {
        Self::broadcast(<T>::one())
    }
}

impl<T: Scalar + Signed, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn abs(&mut self) {
        self.data
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = e.abs()));
    }

    // I can't find a good name for this, expect a rename in the future
    #[inline]
    pub fn abs_copy(&self) -> Self {
        let mut vec = *self;
        vec.abs();
        vec
    }
}

impl<T: Default + Copy, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn from_other_type<U: Default + Copy>(rhs: Matrix<U, { M }, { N }>) -> Self {
        let pointer = &rhs as *const Matrix<U, { M }, { N }> as *const T;
        let new: Matrix<T, { M }, { N }> = unsafe { *pointer.cast() };
        new
    }
}

impl<T: Scalar + Cmp, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn clamp(&mut self, min: Self, max: Self) {
        for j in 0..M {
            for i in 0..N {
                self[[j, i]] = self[[j, i]].maximum(min[[j, i]]).minimum(max[[j, i]])
            }
        }
    }

    /// returns a VecN' with clamped values without consuming VecN
    #[inline]
    pub fn clamped(&self, min: Self, max: Self) -> Self {
        let mut mat = *self;
        mat.clamp(min, max);
        mat
    }

    /// returns a new VecN' with each component having the bigger number from either VecN1 or VecN2
    #[inline]
    pub fn max_by_component(&self, other: &Self) -> Self {
        let mut mat = *self;
        for j in 0..M {
            for i in 0..N {
                mat[[j, i]] = self[[j, i]].maximum(other[[j, i]]);
            }
        }
        mat
    }

    /// returns a new VecN' with each component having the smaller number from either VecN1 or VecN2
    #[inline]
    pub fn min_by_component(&self, other: &Self) -> Self {
        let mut mat = *self;
        for j in 0..M {
            for i in 0..N {
                mat[[j, i]] = self[[j, i]].minimum(other[[j, i]]);
            }
        }
        mat
    }
}

impl<T: Scalar, const M: usize> SquareMatrix<T, { M }> {
    #[inline]
    pub fn determinant(&self) -> T {
        match M {
            0 => T::one(),
            1 => self[[0, 0]],
            2 => self[[0, 0]] * self[[1, 1]] - self[[1, 0]] * self[[1, 0]],
            3 => {
                let e11 = self[[0, 0]];
                let e12 = self[[0, 1]];
                let e13 = self[[0, 2]];

                let e21 = self[[1, 0]];
                let e22 = self[[1, 1]];
                let e23 = self[[1, 2]];

                let e31 = self[[2, 0]];
                let e32 = self[[2, 1]];
                let e33 = self[[2, 2]];

                let minor_1 = e22 * e33 - e32 * e23;
                let minor_2 = e21 * e33 - e31 * e23;
                let minor_3 = e21 * e32 - e31 * e22;

                e11 * minor_1 - e12 * minor_2 + e13 * minor_3
            }
            _ => {
                unimplemented!("TODO: Add LU Decomposition")
            }
        }
    }
}

impl<T, const M: usize, const N: usize> Index<[usize; 2]> for Matrix<T, { M }, { N }> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[0]][index[1]]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<[usize; 2]> for Matrix<T, { M }, { N }> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.data[index[0]][index[1]]
    }
}

impl<T: Scalar, const M: usize, const N: usize> Add for Matrix<T, { M }, { N }> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut mat = Matrix::default();
        for i in 0..N {
            for j in 0..M {
                mat[[i, j]] = self[[i, j]] + rhs[[i, j]];
            }
        }

        mat
    }
}

impl<T: Scalar, const M: usize, const N: usize> AddAssign for Matrix<T, { M }, { N }> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..M {
                self[[i, j]] += rhs[[i, j]];
            }
        }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Sub for Matrix<T, { M }, { N }> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut mat = Matrix::default();
        for i in 0..N {
            for j in 0..M {
                mat[[i, j]] = self[[i, j]] - rhs[[i, j]];
            }
        }
        mat
    }
}

impl<T: Scalar, const M: usize, const N: usize> SubAssign for Matrix<T, { M }, { N }> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..M {
                self[[i, j]] -= rhs[[i, j]];
            }
        }
    }
}

impl<T: Scalar, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, { N }, { P }>>
    for Matrix<T, { M }, { N }>
{
    type Output = Matrix<T, { M }, { P }>;
    #[inline]
    fn mul(self, rhs: Matrix<T, { N }, { P }>) -> Self::Output {
        let mut mat = Matrix::default();
        for m in 0..M {
            for p in 0..P {
                for n in 0..N {
                    mat[[p, m]] += self[[n, m]] * rhs[[p, n]];
                }
            }
        }
        mat
    }
}

impl<T: Scalar, const M: usize, const N: usize> Mul<T> for Matrix<T, { M }, { N }> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut mat = self;
        for m in 0..M {
            for n in 0..N {
                mat[[m, n]] *= rhs
            }
        }
        self
    }
}

impl<T: Scalar, const M: usize, const N: usize> MulAssign<T> for Matrix<T, { M }, { N }> {
    fn mul_assign(&mut self, rhs: T) {
        for m in 0..M {
            for n in 0..N {
                self[[m, n]] *= rhs
            }
        }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Div<T> for Matrix<T, { M }, { N }> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut mat = self;
        for m in 0..M {
            for n in 0..N {
                mat[[m, n]] /= rhs
            }
        }
        self
    }
}

impl<T: Scalar, const M: usize, const N: usize> DivAssign<T> for Matrix<T, { M }, { N }> {
    fn div_assign(&mut self, rhs: T) {
        for m in 0..M {
            for n in 0..N {
                self[[m, n]] /= rhs
            }
        }
    }
}

#[test]
#[ignore]
fn speed() {
    use super::vec::Vector;
    use std::time::Instant;
    let mut vec_n = Vector::from([2.0, 3.0, -2.0, 1.0, 2.0, 3.0, -1.0, 1.0]);

    let start_n = Instant::now();
    for _ in 0..100000000 {
        vec_n.normalized();
    }
    let stop_n = start_n.elapsed();

    println!("Vector<f64, 4> took: {}", stop_n.as_secs_f64());
}