use crate::code::binary_symplectic::BinarySymplecticVector;
use bitvec::prelude::*;
use std::ops::Mul;

/// Pauli演算子の位相を表す列挙型
/// +1, +i, -1, -i の4つの値を持つ
/// Phase同士の乗算も実装している
///
/// # Examples
/// ```rust
/// use qldpc_sim::code::paulis::Phase;
///
/// let phase1 = Phase::I;
/// let phase2 = Phase::MinusI;
/// let result = phase1 * phase2; // resultはPhase::MinusOne
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    One,
    I,
    MinusOne,
    MinusI,
}

impl Mul for Phase {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Phase::One, p) | (p, Phase::One) => p,
            (Phase::I, Phase::I) => Phase::MinusOne,
            (Phase::I, Phase::MinusI) => Phase::One,
            (Phase::MinusI, Phase::I) => Phase::One,
            (Phase::MinusI, Phase::MinusI) => Phase::MinusOne,
            (Phase::I, Phase::MinusOne) => Phase::MinusI,
            (Phase::MinusOne, Phase::I) => Phase::MinusI,
            (Phase::MinusI, Phase::MinusOne) => Phase::I,
            (Phase::MinusOne, Phase::MinusI) => Phase::I,
            (Phase::MinusOne, Phase::MinusOne) => Phase::One,
        }
    }
}

/// Pauli演算子(Paulis)を表す構造体
/// 量子ビット数、位相、Z部分とX部分のビットベクトルを持つ
/// 位相の情報も持っているという点でBinarySymplecticVectorよりも強力
///
/// # Examples
/// ```rust
/// use qldpc_sim::code::paulis::Paulis;
///
/// let pauli = Paulis::from_stirng("+XZYI");
/// let pauli_minus_i = Paulis::from_stirng("-iXZYI");
/// let pauli_identity = Paulis::identity(3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paulis {
    num_qubits: usize,
    phase: Phase,
    binary_symplectic_vector: BinarySymplecticVector,
}

impl Paulis {
    pub fn new(
        num_qubits: usize,
        phase: Phase,
        z_part: BitVec<u64, Lsb0>,
        x_part: BitVec<u64, Lsb0>,
    ) -> Self {
        assert_eq!(
            num_qubits,
            z_part.len(),
            "num_qubits({})とz_partの長さ({})が一致しません",
            num_qubits,
            z_part.len()
        );
        assert_eq!(
            num_qubits,
            x_part.len(),
            "num_qubits({})とx_partの長さ({})が一致しません",
            num_qubits,
            x_part.len()
        );

        Self {
            num_qubits,
            phase,
            binary_symplectic_vector: BinarySymplecticVector::new(z_part, x_part),
        }
    }

    pub fn from_stirng(s: &str) -> Self {
        fn parse_phase(s: &str) -> Phase {
            match s.chars().next() {
                Some(first_char) => match first_char {
                    '+' => match s.chars().nth(1) {
                        Some(second_char) => match second_char {
                            'I' | 'X' | 'Y' | 'Z' => Phase::One,
                            'i' => Phase::I,
                            _ => panic!("不正な文字が含まれています: {}", second_char),
                        },
                        None => panic!("演算子の情報がありません"),
                    },
                    '-' => match s.chars().nth(1) {
                        Some(second_char) => match second_char {
                            'I' | 'X' | 'Y' | 'Z' => Phase::MinusOne,
                            'i' => Phase::MinusI,
                            _ => panic!("不正な文字が含まれています: {}", second_char),
                        },
                        None => panic!("演算子の情報がありません"),
                    },
                    'i' => Phase::I,
                    'I' | 'X' | 'Y' | 'Z' => Phase::One,
                    _ => panic!("不正な文字が含まれています: {}", first_char),
                },
                None => panic!("空の文字列です"),
            }
        }

        fn parse_pauli_s(s: &str) -> (BitVec<u64, Lsb0>, BitVec<u64, Lsb0>) {
            let mut z_part = BitVec::<u64, Lsb0>::new();
            let mut x_part = BitVec::<u64, Lsb0>::new();

            for c in s.chars() {
                match c {
                    '+' | '-' | 'i' => continue,
                    'I' => {
                        z_part.push(false);
                        x_part.push(false);
                    }
                    'X' => {
                        z_part.push(false);
                        x_part.push(true);
                    }
                    'Y' => {
                        z_part.push(true);
                        x_part.push(true);
                    }
                    'Z' => {
                        z_part.push(true);
                        x_part.push(false);
                    }
                    _ => panic!("不正な文字が含まれています: {}", c),
                }
            }

            (z_part, x_part)
        }

        let phase = parse_phase(s);
        let (z_part, x_part) = parse_pauli_s(s);
        let num_qubits = z_part.len();

        Self::new(num_qubits, phase, z_part, x_part)
    }

    pub fn identity(num_qubits: usize) -> Self {
        let z_part = bitvec![u64, Lsb0; 0; num_qubits];
        let x_part = bitvec![u64, Lsb0; 0; num_qubits];

        Self::new(num_qubits, Phase::One, z_part, x_part)
    }

    pub fn get_num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn get_phase(&self) -> Phase {
        self.phase
    }

    pub fn get_binary_symplectic_vector(&self) -> &BinarySymplecticVector {
        &self.binary_symplectic_vector
    }

    pub fn get_z_part(&self) -> &BitVec<u64, Lsb0> {
        self.binary_symplectic_vector.get_z_part()
    }

    pub fn get_x_part(&self) -> &BitVec<u64, Lsb0> {
        self.binary_symplectic_vector.get_x_part()
    }

    pub fn commutes(&self, other: &Paulis) -> bool {
        assert_eq!(
            self.num_qubits, other.num_qubits,
            "比較するPauli文字列の量子ビット数が一致しません"
        );
        !self
            .binary_symplectic_vector
            .symplectic_product(&other.binary_symplectic_vector)
    }
}

impl Mul<&Paulis> for &Paulis {
    type Output = Paulis;

    fn mul(self, rhs: &Paulis) -> Self::Output {
        assert_eq!(
            self.num_qubits, rhs.num_qubits,
            "乗算するPauli文字列の量子ビット数が一致しません"
        );
        let mut phase = self.phase * rhs.phase;
        let mut z_part = BitVec::<u64, Lsb0>::with_capacity(self.num_qubits);
        let mut x_part = BitVec::<u64, Lsb0>::with_capacity(self.num_qubits);

        for i in 0..self.num_qubits {
            let a_z = self.binary_symplectic_vector.get_z_part()[i];
            let a_x = self.binary_symplectic_vector.get_x_part()[i];
            let b_z = rhs.binary_symplectic_vector.get_z_part()[i];
            let b_x = rhs.binary_symplectic_vector.get_x_part()[i];

            match (a_z, a_x, b_z, b_x) {
                (false, false, _, _) | (_, _, false, false) => {}
                (true, false, true, false)
                | (false, true, false, true)
                | (true, true, true, true) => {}
                (false, true, true, true)
                | (true, true, true, false)
                | (true, false, false, true) => {
                    phase = phase * Phase::I;
                }
                (false, true, true, false)
                | (true, true, false, true)
                | (true, false, true, true) => {
                    phase = phase * Phase::MinusI;
                }
            }

            z_part.push(a_z ^ b_z);
            x_part.push(a_x ^ b_x);
        }

        Paulis::new(self.num_qubits, phase, z_part, x_part)
    }
}

impl Mul<Paulis> for Paulis {
    type Output = Paulis;

    fn mul(self, rhs: Paulis) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Paulis> for Paulis {
    type Output = Paulis;

    fn mul(self, rhs: &Paulis) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Paulis> for &Paulis {
    type Output = Paulis;

    fn mul(self, rhs: Paulis) -> Self::Output {
        self * &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paulis_from_string() {
        let pauli_str = Paulis::from_stirng("+XZIY");
        assert_eq!(pauli_str.num_qubits, 4);
        assert_eq!(pauli_str.phase, Phase::One);
        assert_eq!(
            pauli_str.binary_symplectic_vector.get_z_part().clone(),
            bitvec!(u64, Lsb0; 0, 1, 0, 1)
        );
        assert_eq!(
            pauli_str.binary_symplectic_vector.get_x_part().clone(),
            bitvec!(u64, Lsb0; 1, 0, 0, 1)
        );

        let pauli_str = Paulis::from_stirng("-iYZXI");
        assert_eq!(pauli_str.num_qubits, 4);
        assert_eq!(pauli_str.phase, Phase::MinusI);
        assert_eq!(
            pauli_str.binary_symplectic_vector.get_z_part().clone(),
            bitvec!(u64, Lsb0; 1, 1, 0, 0)
        );
        assert_eq!(
            pauli_str.binary_symplectic_vector.get_x_part().clone(),
            bitvec!(u64, Lsb0; 1, 0, 1, 0)
        );

        let pauli_str = Paulis::from_stirng("IIXI");
        assert_eq!(pauli_str.num_qubits, 4);
        assert_eq!(pauli_str.phase, Phase::One);
        assert_eq!(
            pauli_str.binary_symplectic_vector.get_z_part().clone(),
            bitvec!(u64, Lsb0; 0, 0, 0, 0)
        );
        assert_eq!(
            pauli_str.binary_symplectic_vector.get_x_part().clone(),
            bitvec!(u64, Lsb0; 0, 0, 1, 0)
        );
    }

    #[test]
    fn test_pauli_s_multiplication() {
        let pauli_str1 = Paulis::from_stirng("+XZIY");
        let pauli_str2 = Paulis::from_stirng("-iYZXI");
        let result = &pauli_str1 * &pauli_str2;
        let answer = Paulis::from_stirng("+ZIXY");
        assert_eq!(result, answer);

        let pauli_str3 = Paulis::from_stirng("IIXI");
        let result2 = pauli_str1 * pauli_str3;
        let answer2 = Paulis::from_stirng("+XZXY");
        assert_eq!(result2, answer2);
    }

    #[test]
    fn test_pauli_commutes() {
        let pauli_str1 = Paulis::from_stirng("+XZIY");
        let pauli_str2 = Paulis::from_stirng("-iYZXI");
        assert!(!pauli_str1.commutes(&pauli_str2));

        let pauli_str3 = Paulis::from_stirng("+IZII");
        assert!(pauli_str1.commutes(&pauli_str3));
    }
}
