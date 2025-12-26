use crate::code::paulis::Paulis;
use crate::math::bit_linear_algebra::is_linearly_independent;

use bitvec::prelude::*;

/// スタビライザー群を表す構造体
/// 量子ビット数と生成子のベクトルを持つ
/// 生成子は互いに可換で独立である必要があり、コンストラクタでチェックする
///
/// # Examples
/// ```rust
/// use qldpc_sim::code::paulis::Paulis;
/// use qldpc_sim::code::stabilizer::StabilizerGroup;
/// 
/// let s1 = Paulis::from_stirng("XZZXI");
/// let s2 = Paulis::from_stirng("IXZZX");
/// let s3 = Paulis::from_stirng("XIXZZ");
/// let s4 = Paulis::from_stirng("ZXIXZ");
/// let stabilizer_group = StabilizerGroup::new(vec![s1, s2, s3, s4]);
/// 
/// // スタビライザ群の要素を全列挙
/// for stabilizer in stabilizer_group.iter() {
///   println!("{:?}", stabilizer);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct StabilizerGroup {
    generators: Vec<Paulis>,
}

impl StabilizerGroup {
    pub fn new(generators: Vec<Paulis>) -> Self {
        let mut z_part_vecs = Vec::<BitVec<u64, Lsb0>>::new();
        let mut x_part_vecs = Vec::<BitVec<u64, Lsb0>>::new();
        for generator in &generators {
            z_part_vecs.push(generator.get_z_part().clone());
            x_part_vecs.push(generator.get_x_part().clone());
        }
        assert!(
            is_linearly_independent(&z_part_vecs) && is_linearly_independent(&x_part_vecs),
            "演算子が独立ではありません"
        );

        for i in 0..generators.len() {
            for j in (i + 1)..generators.len() {
                assert!(
                    generators[i].get_num_qubits() == generators[j].get_num_qubits(),
                    "生成子の量子ビット数が一致しません"
                );
                assert!(
                    generators[i].commutes(&generators[j]),
                    "生成子が互いに可換ではありません"
                );
            }
        }

        Self { generators }
    }

    pub fn get_num_qubits(&self) -> usize {
        self.generators[0].get_num_qubits()
    }

    pub fn get_generators(&self) -> &Vec<Paulis> {
        &self.generators
    }

    pub fn get_num_generators(&self) -> usize {
        self.generators.len()
    }

    pub fn size(&self) -> usize {
        1 << self.generators.len()
    }

    pub fn iter(&self) -> StabilizerGroupIterator {
        StabilizerGroupIterator {
            stabilizer_group: self.clone(),
            index: 0,
            total: self.size(),
        }
    }

    pub fn include(&self, paulis: &Paulis) -> bool {
        let mut z_part_vecs = Vec::<BitVec<u64, Lsb0>>::new();
        z_part_vecs.push(paulis.get_z_part().clone());
        let mut x_part_vecs = Vec::<BitVec<u64, Lsb0>>::new();
        x_part_vecs.push(paulis.get_x_part().clone());
        for generator in &self.generators {
            z_part_vecs.push(generator.get_z_part().clone());
            x_part_vecs.push(generator.get_x_part().clone());
        }
        !is_linearly_independent(&z_part_vecs) && !is_linearly_independent(&x_part_vecs)
    }
}

pub struct StabilizerGroupIterator {
    stabilizer_group: StabilizerGroup,
    index: usize,
    total: usize,
}

impl Iterator for StabilizerGroupIterator {
    type Item = Paulis;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.total {
            return None;
        }

        let mut result = Paulis::identity(self.stabilizer_group.get_num_qubits());

        for (gen_idx, generator) in self.stabilizer_group.get_generators().iter().enumerate() {
            if (self.index >> gen_idx) & 1 == 1 {
                result = &result * generator;
            }
        }

        self.index += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.total - self.index, Some(self.total - self.index))
    }
}

impl ExactSizeIterator for StabilizerGroupIterator {
    fn len(&self) -> usize {
        self.total - self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stabilizer_new() {
        let s1 = Paulis::from_stirng("XZZXI");
        let s2 = Paulis::from_stirng("IXZZX");
        let s3 = Paulis::from_stirng("XIXZZ");
        let s4 = Paulis::from_stirng("ZXIXZ");
        let stabilizer_group = StabilizerGroup::new(vec![s1, s2, s3, s4]);
        assert_eq!(stabilizer_group.get_num_qubits(), 5);
        assert_eq!(stabilizer_group.get_num_generators(), 4);
    }

    #[test]
    #[should_panic(expected = "演算子が独立ではありません")]
    fn test_stabilizer_new_dependent() {
        let s1 = Paulis::from_stirng("XZZXI");
        let s2 = Paulis::from_stirng("IXZZX");
        let s3 = Paulis::from_stirng("XIXZZ");
        let s4 = Paulis::from_stirng("XIXZZ"); // 重複
        let _stabilizer_group = StabilizerGroup::new(vec![s1, s2, s3, s4]);
    }

    #[test]
    #[should_panic(expected = "生成子が互いに可換ではありません")]
    fn test_stabilizer_new_non_commuting() {
        let s1 = Paulis::from_stirng("XZZXI");
        let s3 = Paulis::from_stirng("XIXZZ");
        let s4 = Paulis::from_stirng("XXXZX"); // 非可換
        let _stabilizer_group = StabilizerGroup::new(vec![s1, s3, s4]);
    }

    #[test]
    fn test_stabilizer_size() {
        let s1 = Paulis::from_stirng("XZZXI");
        let s2 = Paulis::from_stirng("IXZZX");
        let s3 = Paulis::from_stirng("XIXZZ");
        let s4 = Paulis::from_stirng("ZXIXZ");
        let stabilizer_group = StabilizerGroup::new(vec![s1, s2, s3, s4]);
        assert_eq!(stabilizer_group.size(), 16);
    }

    #[test]
    fn test_stabilizer_iterator() {
        let s1 = Paulis::from_stirng("XZZXI");
        let s2 = Paulis::from_stirng("IXZZX");
        let s3 = Paulis::from_stirng("XIXZZ");
        let s4 = Paulis::from_stirng("ZXIXZ");
        let stabilizer_group = StabilizerGroup::new(vec![s1, s2, s3, s4]);
        let mut iter = stabilizer_group.iter();
        let mut count = 0;
        while let Some(_pauli_string) = iter.next() {
            count += 1;
        }
        assert_eq!(count, 16);
    }

    #[test]
    fn test_stabilizer_include() {
        let s1 = Paulis::from_stirng("XZZXI");
        let s2 = Paulis::from_stirng("IXZZX");
        let s3 = Paulis::from_stirng("XIXZZ");
        let s4 = Paulis::from_stirng("ZXIXZ");
        let stabilizer_group = StabilizerGroup::new(vec![s1, s2, s3, s4]);
        let included_pauli = Paulis::from_stirng("YXXYI");
        let not_included_pauli = Paulis::from_stirng("XXXXX");
        assert!(stabilizer_group.include(&included_pauli));
        assert!(!stabilizer_group.include(&not_included_pauli));
    }
}
