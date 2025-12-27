use crate::math::bit_linear_algebra::*;
use bitvec::prelude::*;

/// binary symplectic表現を表す構造体
/// Z部分とX部分のビットベクトルを持つ
/// 位相の情報を持っていないのでPaulisよりも弱いが、符号を扱う上では便利
///
/// TODO: ndarrayやSparseMatrixから生成するメソッドも追加する
///
/// # Examples
/// ```rust
/// use bitvec::prelude::*;
/// use qldpc_sim::code::binary_symplectic::BinarySymplecticVector;
///
/// let bsv = BinarySymplecticVector::new(
///    bitvec![u64, Lsb0; 1, 0, 1],
///   bitvec![u64, Lsb0; 0, 1, 1],
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinarySymplecticVector {
    x_part: BitVec<u64, Lsb0>,
    z_part: BitVec<u64, Lsb0>,
}

impl BinarySymplecticVector {
    pub fn new(x_part: BitVec<u64, Lsb0>, z_part: BitVec<u64, Lsb0>) -> Self {
        assert_eq!(
            z_part.len(),
            x_part.len(),
            "Z部分とX部分の長さが一致しません: z_part.len() = {}, x_part.len() = {}",
            z_part.len(),
            x_part.len()
        );
        Self { z_part, x_part }
    }

    pub fn x_part(&self) -> &BitVec<u64, Lsb0> {
        &self.x_part
    }

    pub fn z_part(&self) -> &BitVec<u64, Lsb0> {
        &self.z_part
    }

    pub fn num_qubits(&self) -> usize {
        self.z_part.len()
    }

    /// このベクトルと他のベクトルのシンプレクティック積を計算する
    ///
    /// # Examples
    /// ```
    /// use bitvec::prelude::*;
    /// use qldpc_sim::code::binary_symplectic::BinarySymplecticVector;
    ///
    /// let v1 = BinarySymplecticVector::new(
    ///     bitvec![u64, Lsb0; 1, 0, 1],
    ///     bitvec![u64, Lsb0; 0, 1, 1],
    /// );
    /// let v2 = BinarySymplecticVector::new(
    ///     bitvec![u64, Lsb0; 0, 1, 1],
    ///     bitvec![u64, Lsb0; 1, 0, 1],
    /// );
    /// let result = v1.symplectic_product(&v2);
    /// assert_eq!(result, false);
    /// ```
    pub fn symplectic_product(&self, other: &BinarySymplecticVector) -> bool {
        assert_eq!(
            self.z_part.len(),
            other.z_part.len(),
            "ベクトルの長さが一致しません"
        );
        let z1 = &self.z_part;
        let x1 = &self.x_part;
        let z2 = &other.z_part;
        let x2 = &other.x_part;

        let term1 = inner_product(z1, x2);
        let term2 = inner_product(x1, z2);

        term1 ^ term2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symplectic_product() {
        let v1 =
            BinarySymplecticVector::new(bitvec![u64, Lsb0; 1, 0, 1], bitvec![u64, Lsb0; 0, 1, 1]);
        let v2 =
            BinarySymplecticVector::new(bitvec![u64, Lsb0; 0, 1, 1], bitvec![u64, Lsb0; 1, 0, 1]);
        assert!(!v1.symplectic_product(&v2));

        let v3 =
            BinarySymplecticVector::new(bitvec![u64, Lsb0; 0, 1, 1], bitvec![u64, Lsb0; 1, 1, 0]);
        assert!(v1.symplectic_product(&v3));
    }
}
