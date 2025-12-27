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

    pub fn x_part(&self) -> &BitVec<u64, Lsb0> {
        &self.x_part
    }

    pub fn z_part(&self) -> &BitVec<u64, Lsb0> {
        &self.z_part
    }

    pub fn num_qubits(&self) -> usize {
        self.x_part.len()
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
