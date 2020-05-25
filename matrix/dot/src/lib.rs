/// matrix dot product
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, PartialEq)]
pub struct Matrix<T>
where
    T: Default,
    T: Ord,
    T: Copy,
    T: Add,
    T: AddAssign,
    T: Mul<Output = T>,
{
    data: Vec<Vec<T>>,
    columns: usize,
    rows: usize,
}

impl<T> Matrix<T>
where
    T: Default,
    T: Ord,
    T: Copy,
    T: Add,
    T: AddAssign,
    T: Mul<Output = T>,
{
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let columns = data.get(0).map_or(0, |x| x.len());
        Matrix {
            data,
            rows,
            columns,
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn dot(self, matrix: Matrix<T>) -> Result<Self, String> {
        if self.rows != matrix.columns {
            return Err(format!(
                "Invalid dimensions. Matrix was expected to have {} columns, found {}",
                self.rows, matrix.columns
            ));
        }

        let mut result: Vec<Vec<T>> = vec![];
        for sr in 0..self.rows {
            let mut result_row = vec![Default::default(); self.rows];
            for mc in 0..matrix.columns {
                for i in 0..self.columns {
                    result_row[mc] += self.data[sr][i] * matrix.data[i][mc];
                }
            }
            result.push(result_row);
        }

        Ok(Matrix::new(result))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::identity_op)]
    fn multiplies_two_square_matrices() {
        let m1 = super::Matrix::new(vec![vec![4, 1], vec![2, 3]]);
        let m2 = super::Matrix::new(vec![vec![2, 8], vec![5, 4]]);

        let expected = super::Matrix::new(vec![
            vec![(4 * 2) + (1 * 5), (4 * 8) + (1 * 4)],
            vec![(2 * 2) + (3 * 5), (2 * 8) + (3 * 4)],
        ]);

        assert_eq!(m1.dot(m2), Ok(expected));
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn multiplies_two_compatible_matrices() {
        let m1 = super::Matrix::new(vec![vec![2, 8, 3], vec![5, 4, 1]]);
        let m2 = super::Matrix::new(vec![vec![4, 1], vec![6, 3], vec![2, 4]]);
        let expected = super::Matrix::new(vec![
            vec![(2 * 4) + (8 * 6) + (3 * 2), (2 * 1) + (8 * 3) + (3 * 4)],
            vec![(5 * 4) + (4 * 6) + (1 * 2), (5 * 1) + (4 * 3) + (1 * 4)],
        ]);

        assert_eq!(m1.dot(m2), Ok(expected));
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn multiplies_two_compatible_matrices2() {
        let m1 = super::Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let m2 = super::Matrix::new(vec![vec![7, 8], vec![9, 10], vec![11, 12]]);
        let expected = super::Matrix::new(vec![vec![58, 64], vec![139, 154]]);

        assert_eq!(m1.dot(m2), Ok(expected));
    }
}
