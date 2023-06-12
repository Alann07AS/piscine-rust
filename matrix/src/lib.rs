use lalgebra_scalar::Scalar;
use std::ops::{ Add, Sub, Mul} ;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T> + Clone>  Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Matrix::zero(n, n);

        for i in 0..n {
            matrix.0[i][i] = T::one();
        }
    
        matrix
	}
}




impl<T: Scalar<Item = T> + Clone> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() ||
        self.0.len() == 0 || rhs.0.len() == 0 ||
        self.0.iter().any(|v| v.len() != self.0[0].len() || v.len() == 0) ||
        rhs.0.iter().any(|v| v.len() != rhs.0[0].len() || v.len() == 0) ||
        self.0[0].len() != rhs.0[0].len()
        {
            return None;
        }
        let mut result = Matrix::zero(self.0.len(),self.0[0].len());//Self(vec![vec![T::zero(); self.0[0].len()]; self.0.len()]);

        self.0.iter().enumerate().for_each(|(i, v1)| {
            let v2 = rhs.0[i].to_owned();
            v1.iter().enumerate().for_each(|(i2, t)| {
                result.0[i][i2] = t.to_owned() + v2[i2].to_owned();
            })
        });
        Some(result)
    }
}

impl<T: Scalar<Item = T> + Clone> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() ||
        self.0.len() == 0 || rhs.0.len() == 0 ||
        self.0.iter().any(|v| v.len() != self.0[0].len() || v.len() == 0) ||
        rhs.0.iter().any(|v| v.len() != rhs.0[0].len() || v.len() == 0) ||
        self.0[0].len() != rhs.0[0].len()
        {
            return None;
        }
        let mut result =  Matrix::zero(self.0.len(),self.0[0].len());//Self(vec![vec![T::zero(); self.0[0].len()]; self.0.len()]);

        self.0.iter().enumerate().for_each(|(i, v1)| {
            let v2 = rhs.0[i].to_owned();
            v1.iter().enumerate().for_each(|(i2, t)| {
                result.0[i][i2] = t.to_owned() - v2[i2].to_owned();
            })
        });
        Some(result)
    }
}



impl<T: Scalar<Item = T> + Clone> Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
	}

	pub fn number_of_rows(&self) -> usize {
        self.0.len()
	}

	pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
	}

	pub fn col(&self, n: usize) -> Vec<T> {
        self.0.clone().iter().map(|l| {
            l[n].to_owned()
        }).collect()
	}
}

impl<T: Scalar<Item = T> + Clone + Copy> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() ||
        self.0.len() == 0 || rhs.0.len() == 0 ||
        self.0.iter().any(|v| v.len() != self.0[0].len() || v.len() == 0) ||
        rhs.0.iter().any(|v| v.len() != rhs.0[0].len() || v.len() == 0) ||
        self.0[0].len() != rhs.0[0].len()
        {
            return None;
        }
        let mut result =  Matrix::zero(self.0.len(),self.0[0].len());//Self(vec![vec![T::zero(); self.0[0].len()]; self.0.len()]);

        self.0.iter().enumerate().for_each(|(i, v1)| {
            let v2 = rhs.0[i].to_owned();
            v1.iter().enumerate().for_each(|(i2, t)| {
                let mut calc_i = 0;
                result.0[i][i2] = self.row(i).into_iter().fold(T::zero(), |acc, n| {
                    let r =acc + n * rhs.0[calc_i][i2];
                    calc_i+=1;
                    r
                });
            })
        });
        Some(result)
    }
}