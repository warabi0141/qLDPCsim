use bitvec::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    One,
    I,
    MinusOne,
    MinusI,
}

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
}