use crate::code::paulis::Paulis;
use crate::code::stabilizer::StabilizerGroup;

struct StabilizerCode {
    stabilizer_group: StabilizerGroup,
}

impl StabilizerCode {
    pub fn new(stabilizer_group: StabilizerGroup) -> Self {
        Self {
            stabilizer_group
        }
    }

    pub fn from_generators(generators: Vec<Paulis>) -> Self {
        let stabilizer_group = StabilizerGroup::new(generators);
        Self::new(stabilizer_group)
    }

    pub fn get_n(&self) -> usize {
        self.stabilizer_group.get_num_qubits()
    }

    pub fn get_k(&self) -> usize {
        let n = self.stabilizer_group.get_num_qubits();
        let r = self.stabilizer_group.get_num_generators();
        n - r
    }

    pub fn get_num_stabilizers(&self) -> usize {
        self.stabilizer_group.get_num_generators()
    }

    pub fn get_stabilizer_group(&self) -> &StabilizerGroup {
        &self.stabilizer_group
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stabilizer_code_new() {
        let generators = vec![
            Paulis::from_stirng("XZZXI"),
            Paulis::from_stirng("IXZZX"),
            Paulis::from_stirng("XIXZZ"),
            Paulis::from_stirng("ZXIXZ"),
        ];
        let stabilizer_code = StabilizerCode::from_generators(generators);
        assert_eq!(stabilizer_code.get_n(), 5);
        assert_eq!(stabilizer_code.get_k(), 1);
        assert_eq!(stabilizer_code.get_num_stabilizers(), 4);
    }
}