use bitvec::prelude::*;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct PauliString {
    num_qubits: usize,
    phase: Phase,
    z_part: BitVec<u64, Lsb0>,
    x_part: BitVec<u64, Lsb0>,
}

impl PauliString {
    pub fn new(num_qubits: usize, phase: Phase, z_part: BitVec<u64, Lsb0>, x_part: BitVec<u64, Lsb0>) -> Self {
        assert_eq!(num_qubits, z_part.len(), "num_qubits({})とz_partの長さ({})が一致しません", num_qubits, z_part.len());
        assert_eq!(num_qubits, x_part.len(), "num_qubits({})とx_partの長さ({})が一致しません", num_qubits, x_part.len());

        Self { num_qubits, phase, z_part, x_part }
    }

    pub fn from_stirng(s: &str) -> Self {

        fn parse_phase(s: &str) -> Phase {
            match s.chars().next() {
                Some(first_char) => {
                    match first_char {
                        '+' => { match s.chars().nth(1) {
                            Some(second_char) => {
                                match second_char {
                                    'I' | 'X' | 'Y' | 'Z' => { Phase::One }
                                    'i' => { Phase::I }
                                    _ => panic!("不正な文字が含まれています: {}", second_char),
                                }
                            }
                            None => panic!("演算子の情報がありません"),
                        } }
                        '-' => { match s.chars().nth(1) {
                            Some(second_char) => {
                                match second_char {
                                    'I' | 'X' | 'Y' | 'Z' => { Phase::MinusOne }
                                    'i' => { Phase::MinusI }
                                    _ => panic!("不正な文字が含まれています: {}", second_char),
                                }
                            }
                            None => panic!("演算子の情報がありません"),
                        } }
                        'i' => { Phase::I }
                        'I' | 'X' | 'Y' | 'Z' => { Phase::One }
                        _ => panic!("不正な文字が含まれています: {}", first_char),
                    }
                }
                None => panic!("空の文字列です"),
            }
        }

        fn parse_pauli_string(s: &str) -> (BitVec<u64, Lsb0>, BitVec<u64, Lsb0>) {
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
        let (z_part, x_part) = parse_pauli_string(s);
        let num_qubits = z_part.len();

        Self::new(num_qubits, phase, z_part, x_part)
    }
}

impl Mul<&PauliString> for &PauliString {
    type Output = PauliString;

    fn mul(self, rhs: &PauliString) -> Self::Output {
        assert_eq!(self.num_qubits, rhs.num_qubits, "乗算するPauli文字列の量子ビット数が一致しません");
        let mut phase = self.phase * rhs.phase;
        let mut z_part = BitVec::<u64, Lsb0>::with_capacity(self.num_qubits);
        let mut x_part = BitVec::<u64, Lsb0>::with_capacity(self.num_qubits);

        for i in 0..self.num_qubits {
            let a_z = self.z_part[i];
            let a_x = self.x_part[i];
            let b_z = rhs.z_part[i];
            let b_x = rhs.x_part[i];

            match (a_z, a_x, b_z, b_x) {
                (false, false, _, _) | (_, _, false, false) => {}
                (true, false, true, false) | (false, true, false, true) | (true, true, true, true) => {}
                (false, true, true, true) | (true, true, true, false) | (true, false, false, true) => { phase = phase * Phase::I; }
                (false, true, true, false) | (true, true, false, true) | (true, false, true, true) => { phase = phase * Phase::MinusI; }
            }

            z_part.push(a_z ^ b_z);
            x_part.push(a_x ^ b_x);
        }

        PauliString::new(self.num_qubits, phase, z_part, x_part)
    }
}

impl Mul<PauliString> for PauliString {
    type Output = PauliString;

    fn mul(self, rhs: PauliString) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&PauliString> for PauliString {
    type Output = PauliString;

    fn mul(self, rhs: &PauliString) -> Self::Output {
        &self * rhs
    }
}

impl Mul<PauliString> for &PauliString {
    type Output = PauliString;

    fn mul(self, rhs: PauliString) -> Self::Output {
        self * &rhs
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pauli_string_from_string() {
        let pauli_str = PauliString::from_stirng("+XZIY");
        assert_eq!(pauli_str.num_qubits, 4);
        assert_eq!(pauli_str.phase, Phase::One);
        assert_eq!(pauli_str.z_part, bitvec!(u64, Lsb0; 0, 1, 0, 1));
        assert_eq!(pauli_str.x_part, bitvec!(u64, Lsb0; 1, 0, 0, 1));

        let pauli_str = PauliString::from_stirng("-iYZXI");
        assert_eq!(pauli_str.num_qubits, 4);
        assert_eq!(pauli_str.phase, Phase::MinusI);
        assert_eq!(pauli_str.z_part, bitvec!(u64, Lsb0; 1, 1, 0, 0));
        assert_eq!(pauli_str.x_part, bitvec!(u64, Lsb0; 1, 0, 1, 0));

        let pauli_str = PauliString::from_stirng("IIXI");
        assert_eq!(pauli_str.num_qubits, 4);
        assert_eq!(pauli_str.phase, Phase::One);
        assert_eq!(pauli_str.z_part, bitvec!(u64, Lsb0; 0, 0, 0, 0));
        assert_eq!(pauli_str.x_part, bitvec!(u64, Lsb0; 0, 0, 1, 0));
    }

    #[test]
    fn test_pauli_string_multiplication() {
        let pauli_str1 = PauliString::from_stirng("+XZIY");
        let pauli_str2 = PauliString::from_stirng("-iYZXI");
        let result = &pauli_str1 * &pauli_str2;
        let answer = PauliString::from_stirng("+ZIXY");
        assert_eq!(result, answer);

        let pauli_str3 = PauliString::from_stirng("IIXI");
        let result2 = pauli_str1 * pauli_str3;
        let answer2 = PauliString::from_stirng("+XZXY");
        assert_eq!(result2, answer2);
    }
}