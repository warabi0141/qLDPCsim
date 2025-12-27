use crate::code::paulis::{Paulis, Phase};
use bitvec::prelude::*;

pub struct ErrorVector {
    x_part: BitVec<u64, Lsb0>,
    z_part: BitVec<u64, Lsb0>,
}

impl ErrorVector {
    pub fn new(x_part: BitVec<u64, Lsb0>, z_part: BitVec<u64, Lsb0>) -> Self {
        assert_eq!(
            x_part.len(),
            z_part.len(),
            "X部分とZ部分の長さが一致しません"
        );
        Self { x_part, z_part }
    }

    pub fn from_paulis(paulis: &Paulis) -> Self {
        Self::new(paulis.x_part().clone(), paulis.z_part().clone())
    }

    pub fn from_string(s: &str) -> Self {
        let paulis = Paulis::from_string(s);
        Self::from_paulis(&paulis)
    }

    pub fn x_part(&self) -> &BitVec<u64, Lsb0> {
        &self.x_part
    }

    pub fn z_part(&self) -> &BitVec<u64, Lsb0> {
        &self.z_part
    }

    pub fn num_qubits(&self) -> usize {
        self.x_part.len()
    }

    pub fn num_errors(&self) -> usize {
        let error_vec = self.x_part().clone() | self.z_part().clone();
        error_vec.count_ones()
    }

    pub fn to_paulis(&self) -> Paulis {
        Paulis::new(
            self.num_qubits(),
            Phase::One,
            self.x_part.clone(),
            self.z_part.clone(),
        )
    }
}

/// シンドロームを表す構造体
/// 現状はCSS符号のみをサポートしているため、ZシンドロームとXシンドロームを別々に持つ
pub struct Syndrome {
    z_syndrome: BitVec<u64, Lsb0>,
    x_syndrome: BitVec<u64, Lsb0>,
}

impl Syndrome {
    pub fn new(z_syndrome: BitVec<u64, Lsb0>, x_syndrome: BitVec<u64, Lsb0>) -> Self {
        Self {
            z_syndrome,
            x_syndrome,
        }
    }

    pub fn len(&self) -> usize {
        self.z_syndrome.len() + self.x_syndrome.len()
    }

    pub fn num_stabilizers(&self) -> usize {
        self.z_syndrome.len() + self.x_syndrome.len()
    }

    pub fn z_syndrome(&self) -> &BitVec<u64, Lsb0> {
        &self.z_syndrome
    }

    pub fn x_syndrome(&self) -> &BitVec<u64, Lsb0> {
        &self.x_syndrome
    }
}
