use crate::code::paulis::Paulis;
use crate::code::quantum_code::QuantumCode;
use crate::code::stabilizer::StabilizerGroup;

/// スタビライザー符号を表す構造体
/// スタビライザー群を持ち、符号のパラメータ(n, k)を計算するメソッドを提供する
///
/// # Examples
/// ```rust
/// use qldpc_sim::code::paulis::Paulis;
/// use qldpc_sim::code::stabilizer::StabilizerGroup;
/// use qldpc_sim::code::stabilizer_code::StabilizerCode;
/// use qldpc_sim::code::quantum_code::QuantumCode;
///
/// let s1 = Paulis::from_stirng("XZZXI");
/// let s2 = Paulis::from_stirng("IXZZX");
/// let s3 = Paulis::from_stirng("XIXZZ");
/// let s4 = Paulis::from_stirng("ZXIXZ");
/// let stabilizer_group = StabilizerGroup::new(vec![s1, s2, s3, s4]);
/// let stabilizer_code = StabilizerCode::new("test code".to_string(), stabilizer_group);
/// assert_eq!(stabilizer_code.get_n(), 5);
/// assert_eq!(stabilizer_code.get_k(), 1);
/// assert_eq!(stabilizer_code.get_num_stabilizers(), 4);
/// ```
#[derive(Debug, Clone)]
pub struct StabilizerCode {
    code_name: String,
    stabilizer_group: StabilizerGroup,
}

impl StabilizerCode {
    pub fn new(code_name: String, stabilizer_group: StabilizerGroup) -> Self {
        Self {
            code_name,
            stabilizer_group,
        }
    }

    pub fn from_generators(code_name: &str, generators: Vec<Paulis>) -> Self {
        let stabilizer_group = StabilizerGroup::new(generators);
        Self::new(code_name.to_string(), stabilizer_group)
    }

    pub fn get_num_stabilizers(&self) -> usize {
        self.stabilizer_group.get_num_generators()
    }

    pub fn get_stabilizer_group(&self) -> &StabilizerGroup {
        &self.stabilizer_group
    }
}

impl QuantumCode for StabilizerCode {
    fn get_code_name(&self) -> &str {
        &self.code_name
    }

    fn get_n(&self) -> usize {
        self.stabilizer_group.get_num_qubits()
    }

    fn get_k(&self) -> usize {
        let n = self.get_n();
        let r = self.get_num_stabilizers();
        n - r
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
        let stabilizer_code = StabilizerCode::from_generators("TestCode", generators);
        assert_eq!(stabilizer_code.get_n(), 5);
        assert_eq!(stabilizer_code.get_k(), 1);
        assert_eq!(stabilizer_code.get_num_stabilizers(), 4);
    }
}
