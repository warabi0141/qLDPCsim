use bitvec::prelude::*;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct BinarySparseMatrix {
    n_rows: usize,
    n_cols: usize,

    /// 各行に含まれる列のインデックス
    row_adj: Vec<Vec<usize>>,
    /// 各列に含まれる行のインデックス
    col_adj: Vec<Vec<usize>>,
}

/// バイナリ疎行列を表す構造体
/// パリティチェック行列を表現するときに使う
/// 行アクセス、列アクセスの両方に対応するため、行隣接リストと列隣接リストの両方を保持する
///
/// # Examples
/// ```rust
/// use bitvec::prelude::*;
/// use qldpc_sim::math::sparse_matrix::BinarySparseMatrix;
///
/// let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
/// let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
/// let matrix = BinarySparseMatrix::new(3, 4, row_adj, col_adj);
/// ```
impl BinarySparseMatrix {
    pub fn new(
        n_rows: usize,
        n_cols: usize,
        row_adj: Vec<Vec<usize>>,
        col_adj: Vec<Vec<usize>>,
    ) -> Self {
        assert_eq!(
            n_rows,
            row_adj.len(),
            "n_rows({})とrow_adjの長さ({})が一致しません",
            n_rows,
            row_adj.len()
        );
        assert_eq!(
            n_cols,
            col_adj.len(),
            "n_cols({})とcol_adjの長さ({})が一致しません",
            n_cols,
            col_adj.len()
        );

        for (row_idx, neighbor) in row_adj.iter().enumerate() {
            for &col_idx in neighbor {
                if col_idx >= n_cols {
                    panic!(
                        "row_adjの要素がn_cols({})を超えています: row_idx = {}, col_idx = {}",
                        n_cols, row_idx, col_idx
                    );
                } else if row_idx >= n_rows {
                    panic!(
                        "col_adjの要素がn_rows({})を超えています: col_idx = {}, row_idx = {}",
                        n_rows, col_idx, row_idx
                    );
                } else if !col_adj[col_idx].contains(&row_idx) {
                    panic!("row_adjとcol_adjが整合していません");
                } else {
                    continue;
                }
            }
        }

        Self {
            n_rows,
            n_cols,
            row_adj,
            col_adj,
        }
    }

    pub fn from_row_adj(n_rows: usize, n_cols: usize, row_adj: Vec<Vec<usize>>) -> Self {
        let mut col_adj = vec![vec![]; n_cols];

        for (row_idx, neighbor) in row_adj.iter().enumerate() {
            for &col_idx in neighbor {
                col_adj[col_idx].push(row_idx);
            }
        }
        Self::new(n_rows, n_cols, row_adj, col_adj)
    }

    pub fn from_col_adj(n_rows: usize, n_cols: usize, col_adj: Vec<Vec<usize>>) -> Self {
        let mut row_adj = vec![vec![]; n_rows];

        for (col_idx, neighbor) in col_adj.iter().enumerate() {
            for &row_idx in neighbor {
                row_adj[row_idx].push(col_idx);
            }
        }
        Self::new(n_rows, n_cols, row_adj, col_adj)
    }

    pub fn zeros(n_rows: usize, n_cols: usize) -> Self {
        let row_adj = vec![vec![]; n_rows];
        let col_adj = vec![vec![]; n_cols];
        Self::new(n_rows, n_cols, row_adj, col_adj)
    }

    pub fn rows(&self) -> usize {
        self.n_rows
    }

    pub fn cols(&self) -> usize {
        self.n_cols
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.n_rows, self.n_cols)
    }

    /// 疎行列のままランクを計算する（ガウスの消去法）
    ///
    /// # Examples
    /// ```
    /// use qldpc_sim::math::sparse_matrix::BinarySparseMatrix;
    ///
    /// let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
    /// let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
    /// let matrix = BinarySparseMatrix::new(3, 4, row_adj, col_adj);
    /// let rank = matrix.rank();
    /// assert_eq!(rank, 3);
    /// ```
    pub fn rank(&self) -> usize {
        let mut matrix = self.row_adj.clone();
        let mut rank = 0;

        for col in 0..self.n_cols {
            // ピボット行を探す（col を含む行）
            let pivot_row = (rank..self.n_rows).find(|&row| matrix[row].contains(&col));

            if let Some(pivot) = pivot_row {
                // ピボット行を rank 行に交換
                matrix.swap(rank, pivot);

                // 他の行を消去（col を含む行をすべて消去）
                for row in 0..self.n_rows {
                    if row != rank && matrix[row].contains(&col) {
                        // row と rank 行の XOR
                        let rank_row = matrix[rank].clone();
                        matrix[row] = Self::xor_neighbors(&matrix[row], &rank_row);
                    }
                }

                rank += 1;
            }
        }

        rank
    }

    /// 2つの隣接リストの XOR を計算する
    fn xor_neighbors(a: &[usize], b: &[usize]) -> Vec<usize> {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < a.len() && j < b.len() {
            if a[i] == b[j] {
                // 両方に含まれる要素は削除（XOR）
                i += 1;
                j += 1;
            } else if a[i] < b[j] {
                result.push(a[i]);
                i += 1;
            } else {
                result.push(b[j]);
                j += 1;
            }
        }

        result.extend_from_slice(&a[i..]);
        result.extend_from_slice(&b[j..]);

        result
    }

    /// 行列が線形独立かどうかを判定する
    ///
    /// # Examples
    /// ```
    /// use qldpc_sim::math::sparse_matrix::BinarySparseMatrix;
    ///
    /// let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
    /// let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
    /// let matrix = BinarySparseMatrix::new(3, 4, row_adj, col_adj);
    /// // ランク = 3 = 行数なので線形独立
    /// assert!(matrix.is_linearly_independent());
    /// ```
    pub fn is_linearly_independent(&self) -> bool {
        self.rank() == self.n_rows
    }

    pub fn transpose(&self) -> Self {
        BinarySparseMatrix::from_col_adj(self.n_cols, self.n_rows, self.row_adj.clone())
    }
}

/// バイナリ疎行列とバイナリベクトルの積を計算する
impl Mul<&BitVec<u64, Lsb0>> for &BinarySparseMatrix {
    type Output = BitVec<u64, Lsb0>;

    fn mul(self, rhs: &BitVec<u64, Lsb0>) -> Self::Output {
        assert_eq!(
            self.n_cols,
            rhs.len(),
            "行列の列数({})とベクトルの長さ({})が一致していません",
            self.n_cols,
            rhs.len()
        );

        let mut result = bitvec![u64, Lsb0; 0; self.n_rows];

        for (row_idx, neighbors) in self.row_adj.iter().enumerate() {
            let mut parity = false;

            for &col_idx in neighbors {
                if rhs[col_idx] {
                    parity = !parity;
                }
            }

            result.set(row_idx, parity);
        }
        /* `bitvec::vec::BitVec<u64>` value */
        result
    }
}

impl Mul<BitVec<u64, Lsb0>> for BinarySparseMatrix {
    type Output = BitVec<u64, Lsb0>;

    fn mul(self, rhs: BitVec<u64, Lsb0>) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&BitVec<u64, Lsb0>> for BinarySparseMatrix {
    type Output = BitVec<u64, Lsb0>;

    fn mul(self, rhs: &BitVec<u64, Lsb0>) -> Self::Output {
        &self * rhs
    }
}

impl Mul<BitVec<u64, Lsb0>> for &BinarySparseMatrix {
    type Output = BitVec<u64, Lsb0>;

    fn mul(self, rhs: BitVec<u64, Lsb0>) -> Self::Output {
        self * &rhs
    }
}

/// バイナリ疎行列とバイナリ疎行列の積を計算する
impl Mul<&BinarySparseMatrix> for &BinarySparseMatrix {
    type Output = BinarySparseMatrix;

    fn mul(self, rhs: &BinarySparseMatrix) -> Self::Output {
        assert_eq!(
            self.n_cols, rhs.n_rows,
            "左の列数({})と右の行数({})が一致しません",
            self.n_cols, rhs.n_rows
        );

        let mut result_row_adj: Vec<Vec<usize>> = Vec::with_capacity(self.n_rows);

        for row_idx in 0..self.n_rows {
            let mut result_neighbors: Vec<usize> = Vec::new();

            for col_idx in 0..rhs.n_cols {
                let mut parity = false;

                for &k in &self.row_adj[row_idx] {
                    if rhs.col_adj[col_idx].contains(&k) {
                        parity = !parity;
                    }
                }

                if parity {
                    result_neighbors.push(col_idx);
                }
            }

            result_row_adj.push(result_neighbors);
        }

        BinarySparseMatrix::from_row_adj(self.n_rows, rhs.n_cols, result_row_adj)
    }
}

impl Mul<BinarySparseMatrix> for BinarySparseMatrix {
    type Output = BinarySparseMatrix;

    fn mul(self, rhs: BinarySparseMatrix) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&BinarySparseMatrix> for BinarySparseMatrix {
    type Output = BinarySparseMatrix;

    fn mul(self, rhs: &BinarySparseMatrix) -> Self::Output {
        &self * rhs
    }
}

impl Mul<BinarySparseMatrix> for &BinarySparseMatrix {
    type Output = BinarySparseMatrix;

    fn mul(self, rhs: BinarySparseMatrix) -> Self::Output {
        self * &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "row_adjとcol_adjが整合していません")]
    fn test_new_panic() {
        let n_rows = 3;
        let n_cols = 4;
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![3]];
        BinarySparseMatrix::new(n_rows, n_cols, row_adj, col_adj);
    }

    #[test]
    fn test_new() {
        let n_rows = 3;
        let n_cols = 4;
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
        let _matrix = BinarySparseMatrix::new(n_rows, n_cols, row_adj, col_adj);
    }

    #[test]
    fn test_from_row_adj() {
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
        let matrix = BinarySparseMatrix::new(3, 4, row_adj, col_adj);
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let matrix_from_row_adj = BinarySparseMatrix::from_row_adj(3, 4, row_adj);
        assert_eq!(matrix_from_row_adj, matrix);
    }

    #[test]
    fn test_from_col_adj() {
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
        let matrix = BinarySparseMatrix::new(3, 4, row_adj, col_adj);
        let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
        let matrix_from_col_adj = BinarySparseMatrix::from_col_adj(3, 4, col_adj);
        assert_eq!(matrix_from_col_adj, matrix);
    }

    #[test]
    fn test_transpose() {
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let matrix = BinarySparseMatrix::from_row_adj(3, 4, row_adj);
        let transposed = matrix.transpose();
        let expected_row_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
        let expected = BinarySparseMatrix::from_row_adj(4, 3, expected_row_adj);
        assert_eq!(transposed, expected);
    }

    #[test]
    fn test_mul_binary_vec() {
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let matrix = BinarySparseMatrix::from_row_adj(3, 4, row_adj);
        let vec = bitvec![u64, Lsb0; 1, 0, 1, 0];
        let result = &matrix * &vec;
        let expected = bitvec![u64, Lsb0; 1, 1, 1];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_binary_sparse_matrix() {
        let row_adj_a = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let matrix_a = BinarySparseMatrix::from_row_adj(3, 4, row_adj_a);
        let row_adj_b = vec![vec![0, 2], vec![1], vec![2], vec![0, 1]];
        let matrix_b = BinarySparseMatrix::from_row_adj(4, 4, row_adj_b);
        let result = &matrix_a * &matrix_b;
        let expected_row_adj = vec![vec![0, 1, 2], vec![1, 2], vec![0, 1, 2]];
        let expected = BinarySparseMatrix::from_row_adj(3, 4, expected_row_adj);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_zero_matrix() {
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let matrix = BinarySparseMatrix::from_row_adj(3, 4, row_adj);
        let zero_matrix = BinarySparseMatrix::zeros(4, 5);
        let result = &matrix * &zero_matrix;
        let expected = BinarySparseMatrix::zeros(3, 5);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rank() {
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 3]];
        let matrix = BinarySparseMatrix::from_row_adj(4, 4, row_adj);
        let rank = matrix.rank();
        assert_eq!(rank, 3);
    }
}
