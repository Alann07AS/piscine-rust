use lalgebra_scalar::Scalar;

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