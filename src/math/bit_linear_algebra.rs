use bitvec::prelude::*;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryDenseMatrix {
    data: Vec<BitVec<u64, Lsb0>>,
}

impl BinaryDenseMatrix {
    pub fn new(data: Vec<BitVec<u64, Lsb0>>) -> Self {
        let n_cols = data[0].len();
        for vec in &data {
            assert_eq!(
                vec.len(),
                n_cols,
                "行列の各行の長さが一致しません: expected {}, got {}",
                n_cols,
                vec.len()
            );
        }
        Self { data }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![bitvec![u64, Lsb0; 0; cols]; rows];
        Self::new(data)
    }

    pub fn identity(size: usize) -> Self {
        let mut data: Vec<BitVec<u64, Lsb0>> = Vec::with_capacity(size);
        for i in 0..size {
            let mut row: BitVec<u64, Lsb0> = bitvec![u64, Lsb0; 0; size];
            row.set(i, true);
            data.push(row);
        }
        Self::new(data)
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    pub fn get_data(&self) -> &[BitVec<u64, Lsb0>] {
        &self.data
    }

    pub fn rank(&self) -> usize {
        rank(&self.data)
    }

    pub fn transpose(&self) -> Self {
        let mut transposed_data: Vec<BitVec<u64, Lsb0>> = Vec::with_capacity(self.cols());
        for col_idx in 0..self.cols() {
            let mut col_vec: BitVec<u64, Lsb0> = bitvec![u64, Lsb0; 0; self.rows()];
            for row_idx in 0..self.rows() {
                col_vec.set(row_idx, self.data[row_idx][col_idx]);
            }
            transposed_data.push(col_vec);
        }
        Self::new(transposed_data)
    }
}

/// バイナリ密行列とバイナリベクトルの積を計算する
impl Mul<&BitVec<u64, Lsb0>> for &BinaryDenseMatrix {
    type Output = BitVec<u64, Lsb0>;

    fn mul(self, rhs: &BitVec<u64, Lsb0>) -> Self::Output {
        assert_eq!(
            self.cols(),
            rhs.len(),
            "行列の列数({})とベクトルの長さ({})が一致していません",
            self.cols(),
            rhs.len()
        );

        let mut result: BitVec<u64, Lsb0> = bitvec![u64, Lsb0; 0; self.rows()];

        for (i, row) in self.data.iter().enumerate() {
            result.set(i, inner_product(row, rhs));
        }

        result
    }
}

impl Mul<BitVec<u64, Lsb0>> for BinaryDenseMatrix {
    type Output = BitVec<u64, Lsb0>;

    fn mul(self, rhs: BitVec<u64, Lsb0>) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&BitVec<u64, Lsb0>> for BinaryDenseMatrix {
    type Output = BitVec<u64, Lsb0>;

    fn mul(self, rhs: &BitVec<u64, Lsb0>) -> Self::Output {
        &self * rhs
    }
}

impl Mul<BitVec<u64, Lsb0>> for &BinaryDenseMatrix {
    type Output = BitVec<u64, Lsb0>;

    fn mul(self, rhs: BitVec<u64, Lsb0>) -> Self::Output {
        self * &rhs
    }
}

/// バイナリ密行列とバイナリ密行列の積を計算する
impl Mul<&BinaryDenseMatrix> for &BinaryDenseMatrix {
    type Output = BinaryDenseMatrix;

    fn mul(self, rhs: &BinaryDenseMatrix) -> Self::Output {
        assert_eq!(
            self.cols(),
            rhs.rows(),
            "左の列数({})と右の行数({})が一致しません",
            self.cols(),
            rhs.rows()
        );

        let mut result_data: Vec<BitVec<u64, Lsb0>> = Vec::with_capacity(self.rows());

        for row in &self.data {
            let mut result_row: BitVec<u64, Lsb0> = bitvec![u64, Lsb0; 0; rhs.cols()];

            for col_idx in 0..rhs.cols() {
                let mut col_vec: BitVec<u64, Lsb0> = bitvec![u64, Lsb0; 0; rhs.rows()];
                for row_idx in 0..rhs.rows() {
                    col_vec.set(row_idx, rhs.data[row_idx][col_idx]);
                }
                result_row.set(col_idx, inner_product(row, &col_vec));
            }

            result_data.push(result_row);
        }

        BinaryDenseMatrix::new(result_data)
    }
}

impl Mul<BinaryDenseMatrix> for BinaryDenseMatrix {
    type Output = BinaryDenseMatrix;

    fn mul(self, rhs: BinaryDenseMatrix) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&BinaryDenseMatrix> for BinaryDenseMatrix {
    type Output = BinaryDenseMatrix;

    fn mul(self, rhs: &BinaryDenseMatrix) -> Self::Output {
        &self * rhs
    }
}

impl Mul<BinaryDenseMatrix> for &BinaryDenseMatrix {
    type Output = BinaryDenseMatrix;

    fn mul(self, rhs: BinaryDenseMatrix) -> Self::Output {
        self * &rhs
    }
}

/// ビットベクトル同士の内積を計算する
///
/// # Examples
/// ```rust
/// use bitvec::prelude::*;
/// use qldpc_sim::math::bit_linear_algebra::inner_product;
///
/// let a = bitvec![u64, Lsb0; 1, 0, 1, 1];
/// let b = bitvec![u64, Lsb0; 1, 1, 0, 1];
/// let result = inner_product(&a, &b);
/// assert_eq!(result, false);
/// ```
pub fn inner_product(a: &BitVec<u64, Lsb0>, b: &BitVec<u64, Lsb0>) -> bool {
    assert_eq!(
        a.len(),
        b.len(),
        "ベクトルの長さが一致しません: a.len() = {}, b.len() = {}",
        a.len(),
        b.len()
    );

    let mut parity = false;

    for i in 0..a.len() {
        if a[i] & b[i] {
            parity = !parity;
        }
    }

    parity
}

/// ビット行列のランクを計算する
/// ビット行列は、`Vec<BitVec>`で表され、各`BitVec`が行を表す
///
/// # Examples
/// ```rust
/// use bitvec::prelude::*;
/// use qldpc_sim::math::bit_linear_algebra::rank;
///
/// let vectors = vec![
///     bitvec![u64, Lsb0; 1, 0, 0, 1],
///     bitvec![u64, Lsb0; 0, 1, 1, 0],
///     bitvec![u64, Lsb0; 1, 1, 1, 1],
/// ];
/// let rank = rank(&vectors);
/// assert_eq!(rank, 2);
/// ```
pub fn rank(bit_matrix: &[BitVec<u64, Lsb0>]) -> usize {
    let n = bit_matrix.len();
    if n == 0 {
        return 0;
    }
    let m = bit_matrix[0].len();
    for vec in bit_matrix {
        assert_eq!(vec.len(), m, "ベクトルの長さが一致しません");
    }

    let mut mat: Vec<BitVec<u64, Lsb0>> = bit_matrix.to_vec();

    let mut rank = 0;

    for col in 0..m {
        let mut pivot_row = None;
        for (row, _) in mat.iter().enumerate().take(n).skip(rank) {
            if mat[row][col] {
                pivot_row = Some(row);
                break;
            }
        }

        if let Some(pivot) = pivot_row {
            mat.swap(rank, pivot);

            for row in 0..n {
                if row != rank && mat[row][col] {
                    let rank_vec = mat[rank].clone();
                    mat[row] ^= rank_vec;
                }
            }

            rank += 1;
        }
    }

    rank
}

/// ビットベクトルの集合が線形独立かどうかを判定する
///
/// # Examples
/// ```rust
/// use bitvec::prelude::*;
/// use qldpc_sim::math::bit_linear_algebra::is_linearly_independent;
///
/// let vectors = vec![
///     bitvec![u64, Lsb0; 1, 0, 0, 1],
///     bitvec![u64, Lsb0; 0, 1, 1, 0],
///     bitvec![u64, Lsb0; 1, 1, 0, 1],
/// ];
/// let is_independent = is_linearly_independent(&vectors);
/// assert_eq!(is_independent, true);
/// ```
pub fn is_linearly_independent(vectors: &[BitVec<u64, Lsb0>]) -> bool {
    rank(vectors) == vectors.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_product() {
        let a = bitvec![u64, Lsb0; 1, 0, 1, 1];
        let b = bitvec![u64, Lsb0; 1, 1, 0, 1];
        assert!(!inner_product(&a, &b));

        let c = bitvec![u64, Lsb0; 1, 1, 0, 0];
        assert!(inner_product(&a, &c));
    }

    #[test]
    fn test_rank() {
        let vectors = vec![
            bitvec![u64, Lsb0; 1, 0, 0, 1],
            bitvec![u64, Lsb0; 0, 1, 1, 0],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
        ];
        assert_eq!(rank(&vectors), 2);
        let dependent_vectors = vec![
            bitvec![u64, Lsb0; 1, 0, 0, 1],
            bitvec![u64, Lsb0; 0, 1, 1, 0],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
        ];
        assert_eq!(rank(&dependent_vectors), 2);
    }

    #[test]
    fn test_is_linearly_independent() {
        let independent_vectors = vec![
            bitvec![u64, Lsb0; 1, 0, 0, 1],
            bitvec![u64, Lsb0; 0, 1, 1, 0],
            bitvec![u64, Lsb0; 1, 1, 0, 1],
        ];
        assert!(is_linearly_independent(&independent_vectors));
        let dependent_vectors = vec![
            bitvec![u64, Lsb0; 1, 0, 0, 1],
            bitvec![u64, Lsb0; 0, 1, 1, 0],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
        ];
        assert!(!is_linearly_independent(&dependent_vectors));
    }

    #[test]
    fn test_binary_dense_matrix_mul_bitvec() {
        let data = vec![
            bitvec![u64, Lsb0; 1, 0, 1],
            bitvec![u64, Lsb0; 0, 1, 1],
            bitvec![u64, Lsb0; 1, 1, 0],
        ];
        let matrix = BinaryDenseMatrix::new(data);
        let vec = bitvec![u64, Lsb0; 1, 0, 1];
        let result = &matrix * &vec;
        assert_eq!(result, bitvec![u64, Lsb0; 0, 1, 1]);
    }

    #[test]
    fn test_binary_dense_matrix_mul_matrix() {
        let data_a = vec![bitvec![u64, Lsb0; 1, 0, 1], bitvec![u64, Lsb0; 0, 1, 1]];
        let matrix_a = BinaryDenseMatrix::new(data_a);

        let data_b = vec![
            bitvec![u64, Lsb0; 1, 1],
            bitvec![u64, Lsb0; 0, 1],
            bitvec![u64, Lsb0; 1, 0],
        ];
        let matrix_b = BinaryDenseMatrix::new(data_b);

        let result_matrix = &matrix_a * &matrix_b;

        let expected_data = vec![bitvec![u64, Lsb0; 0, 1], bitvec![u64, Lsb0; 1, 1]];
        let expected_matrix = BinaryDenseMatrix::new(expected_data);

        assert_eq!(result_matrix, expected_matrix);
    }

    #[test]
    fn test_binary_dense_matrix_mul_zero_matrix() {
        let data = vec![bitvec![u64, Lsb0; 1, 0, 1], bitvec![u64, Lsb0; 0, 1, 1]];
        let matrix = BinaryDenseMatrix::new(data);
        let zero_matrix = BinaryDenseMatrix::zeros(3, 4);
        let result = &matrix * &zero_matrix;
        let expected = BinaryDenseMatrix::zeros(2, 4);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_binary_dense_matrix_mul_identity_matrix() {
        let data = vec![bitvec![u64, Lsb0; 1, 0, 1], bitvec![u64, Lsb0; 0, 1, 1]];
        let matrix = BinaryDenseMatrix::new(data);
        let identity_matrix = BinaryDenseMatrix::identity(3);
        let result = &matrix * &identity_matrix;
        let expected = matrix.clone();
        assert_eq!(result, expected);
    }
}
