use crate::code::error_vector::{ErrorVector, Syndrome};
use crate::code::quantum_code::QuantumCode;
use crate::code::stabilizer_code::StabilizerCode;
use crate::math::sparse_matrix::BinarySparseMatrix;

struct CssCode {
    code_name: String,
    hz: BinarySparseMatrix,
    hx: BinarySparseMatrix,
}

impl CssCode {
    pub fn new(code_name: String, hz: BinarySparseMatrix, hx: BinarySparseMatrix) -> Self {
        Self { code_name, hz, hx }
    }

    pub fn from_stabilizer_code(stabilizer_code: &StabilizerCode) -> Self {
        todo!("Implement conversion from StabilizerCode to CssCode")
    }

    pub fn from_parity_check_matrices(
        code_name: &str,
        hz: BinarySparseMatrix,
        hx: BinarySparseMatrix,
    ) -> Self {
        let val = &hx * &hz.transpose();
        assert_eq!(
            val,
            BinarySparseMatrix::zeros(hx.rows(), hz.rows()),
            "H_ZとH_Xが直交していません"
        );
        let k = hz.cols() - hz.rank() - hx.rank();
        assert!(k > 0, "論理量子ビットが存在しません: k = {}", k);
        CssCode::new(code_name.to_string(), hz, hx)
    }

    pub fn hx(&self) -> &BinarySparseMatrix {
        &self.hx
    }

    pub fn hz(&self) -> &BinarySparseMatrix {
        &self.hz
    }

    pub fn lx(&self) -> BinarySparseMatrix {
        todo!("Implement L_X generation")
    }

    pub fn lz(&self) -> BinarySparseMatrix {
        todo!("Implement L_Z generation")
    }

    pub fn num_stabilizers(&self) -> usize {
        self.hz.rows() + self.hx.rows()
    }

    pub fn num_qubits(&self) -> usize {
        self.hz.cols()
    }

    /// 誤りベクトルに対するシンドロームを計算する
    /// シンドロームや
    pub fn syndrome(&self, error_vector: &ErrorVector) -> Syndrome {
        let z_part = error_vector.z_part();
        let x_part = error_vector.x_part();
        let syndrome_z = &self.hz * x_part;
        let syndrome_x = &self.hx * z_part;
        Syndrome::new(syndrome_z, syndrome_x)
    }
}

impl QuantumCode for CssCode {
    fn code_name(&self) -> &str {
        &self.code_name
    }

    fn n(&self) -> usize {
        self.hz.cols()
    }

    fn k(&self) -> usize {
        let n = self.n();
        let rank_hz = self.hz.rank();
        let rank_hx = self.hx.rank();
        n - rank_hz - rank_hx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::sparse_matrix::BinarySparseMatrix;
    use bitvec::prelude::*;

    #[test]
    fn test_css_code_new() {
        let hz_row_adj = vec![
            vec![0, 1],
            vec![1, 2],
            vec![3, 4],
            vec![4, 5],
            vec![6, 7],
            vec![7, 8],
        ];
        let hx_row_adj = vec![vec![0, 1, 2, 3, 4, 5], vec![3, 4, 5, 6, 7, 8]];
        let hz = BinarySparseMatrix::from_row_adj(6, 9, hz_row_adj);
        let hx = BinarySparseMatrix::from_row_adj(2, 9, hx_row_adj);
        let css_code = CssCode::from_parity_check_matrices("TestCSS", hz, hx);
        assert_eq!(css_code.num_qubits(), 9);
        assert_eq!(css_code.k(), 1);
        assert_eq!(css_code.num_stabilizers(), 8);
    }

    #[test]
    #[should_panic(expected = "H_ZとH_Xが直交していません")]
    fn test_css_code_non_orthogonal() {
        let hz_row_adj = vec![vec![0, 1], vec![1, 2]];
        let hx_row_adj = vec![vec![1, 2], vec![2, 3]];
        let hz = BinarySparseMatrix::from_row_adj(2, 4, hz_row_adj);
        let hx = BinarySparseMatrix::from_row_adj(2, 4, hx_row_adj);
        let _css_code = CssCode::from_parity_check_matrices("NonOrthogonalCSS", hz, hx);
    }

    #[test]
    #[should_panic(expected = "論理量子ビットが存在しません")]
    fn test_css_code_no_logical_qubits() {
        let hz_row_adj = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let hx_row_adj = vec![vec![0, 1, 2, 3]];
        let hz = BinarySparseMatrix::from_row_adj(3, 4, hz_row_adj);
        let hx = BinarySparseMatrix::from_row_adj(1, 4, hx_row_adj);
        let _css_code = CssCode::from_parity_check_matrices("NoLogicalQubitsCSS", hz, hx);
    }

    #[test]
    fn test_css_code_syndrome() {
        let hz_row_adj = vec![
            vec![0, 1],
            vec![1, 2],
            vec![3, 4],
            vec![4, 5],
            vec![6, 7],
            vec![7, 8],
        ];
        let hx_row_adj = vec![vec![0, 1, 2, 3, 4, 5], vec![3, 4, 5, 6, 7, 8]];
        let hz = BinarySparseMatrix::from_row_adj(6, 9, hz_row_adj);
        let hx = BinarySparseMatrix::from_row_adj(2, 9, hx_row_adj);
        let css_code = CssCode::from_parity_check_matrices("TestCSS", hz, hx);
        let error_z = bitvec![u64, Lsb0; 1, 0, 0, 0, 0, 0, 0, 0, 0];
        let error_x = bitvec![u64, Lsb0; 0, 1, 0, 0, 0, 0, 0, 0, 0];
        let error_vector = ErrorVector::new(error_x, error_z);
        let syndrome = css_code.syndrome(&error_vector);
        let expected_z_syndrome = bitvec![u64, Lsb0; 1, 1, 0, 0, 0, 0];
        let expected_x_syndrome = bitvec![u64, Lsb0; 1, 0];
        assert_eq!(syndrome.z_syndrome(), &expected_z_syndrome);
        assert_eq!(syndrome.x_syndrome(), &expected_x_syndrome);
    }
}
