use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SparseMatrix {
    pub n_rows: usize,
    pub n_cols: usize,

    /// 各行に含まれる列のインデックス
    pub row_adj: Vec<Vec<usize>>,
    /// 各列に含まれる行のインデックス
    pub col_adj: Vec<Vec<usize>>,
}

impl SparseMatrix {
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
                if !col_adj[col_idx].contains(&row_idx) {
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

    /// 疎行列とバイナリベクトルの積を計算する
    ///
    /// # Arguments
    /// * `rhs` - 右側から掛けるビットベクトル (長さは self.n_cols と一致する必要がある)
    pub fn multiply_with_bitvec(&self, rhs: &BitVec<u64, Lsb0>) -> BitVec<u64, Lsb0> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_with_bitvec() {
        // 3x4 の行列 H
        // [1 1 0 0]
        // [0 1 1 0]
        // [0 0 1 1]
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
        let matrix = SparseMatrix::new(3, 4, row_adj, col_adj);

        // エラーベクトル e = [1, 0, 1, 0]
        let mut error: BitVec<u64, Lsb0> = bitvec![u64, Lsb0; 0; 4];
        error.set(0, true);
        error.set(2, true);

        // 計算: s = H * e
        // s[0] = e[0] ^ e[1] = 1 ^ 0 = 1
        // s[1] = e[1] ^ e[2] = 0 ^ 1 = 1
        // s[2] = e[2] ^ e[3] = 1 ^ 0 = 1
        let syndrome = matrix.multiply_with_bitvec(&error);

        assert_eq!(syndrome, bitvec![1, 1, 1]);
    }

    #[test]
    #[should_panic(expected = "row_adjとcol_adjが整合していません")]
    fn test_new_panic() {
        let n_rows = 3;
        let n_cols = 4;
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![3]];
        SparseMatrix::new(n_rows, n_cols, row_adj, col_adj);
    }

    #[test]
    fn test_new() {
        let n_rows = 3;
        let n_cols = 4;
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
        let _matrix = SparseMatrix::new(n_rows, n_cols, row_adj, col_adj);
    }

    #[test]
    fn test_from_row_adj() {
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let col_adj = vec![vec![0], vec![0, 1], vec![1, 2], vec![2]];
        let matrix = SparseMatrix::new(3, 4, row_adj, col_adj);
        let row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let matrix_from_row_adj = SparseMatrix::from_row_adj(3, 4, row_adj);
        assert_eq!(matrix_from_row_adj, matrix);
    }
}
