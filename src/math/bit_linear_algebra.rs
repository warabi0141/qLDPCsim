use bitvec::prelude::*;

pub fn inner_product(a: &BitVec<u64, Lsb0>, b: &BitVec<u64, Lsb0>) -> bool {
    assert_eq!(a.len(), b.len(), "ベクトルの長さが一致しません: a.len() = {}, b.len() = {}", a.len(), b.len());

    let mut parity = false;

    for i in 0..a.len() {
        if a[i] & b[i] {
            parity = !parity;
        }
    }

    parity
}

pub fn rank(bit_matrix: &[BitVec<u64, Lsb0>]) -> usize {
    let n = bit_matrix.len();
    if n == 0 {
        return 0;
    }
    let m = bit_matrix[0].len();
    for vec in bit_matrix {
        assert_eq!(vec.len(), m, "ベクトルの長さが一致しません");
    }

    let mut mat: Vec<BitVec<u64, Lsb0>> = bit_matrix.to_vec();

    let mut rank = 0;

    for col in 0..m {
        let mut pivot_row = None;
        for row in rank..n {
            if mat[row][col] {
                pivot_row = Some(row);
                break;
            }
        }

        if let Some(pivot) = pivot_row {
            mat.swap(rank, pivot);

            for row in 0..n {
                if row != rank && mat[row][col] {
                    let rank_vec = mat[rank].clone();
                    mat[row] ^=  rank_vec;
                }
            }

            rank += 1;
        }
    }

    rank
}

pub fn is_linearly_independent(vectors: &[BitVec<u64, Lsb0>]) -> bool {
    rank(vectors) == vectors.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_product() {
        let a = bitvec![u64, Lsb0; 1, 0, 1, 1];
        let b = bitvec![u64, Lsb0; 1, 1, 0, 1];
        assert_eq!(inner_product(&a, &b), false);

        let c = bitvec![u64, Lsb0; 1, 1, 0, 0];
        assert_eq!(inner_product(&a, &c), true);
    }

    #[test]
    fn test_rank() {
        let vectors = vec![
            bitvec![u64, Lsb0; 1, 0, 0, 1],
            bitvec![u64, Lsb0; 0, 1, 1, 0],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
        ];
        assert_eq!(rank(&vectors), 2);
        let dependent_vectors = vec![
            bitvec![u64, Lsb0; 1, 0, 0, 1],
            bitvec![u64, Lsb0; 0, 1, 1, 0],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
        ];
        assert_eq!(rank(&dependent_vectors), 2);
    }

    #[test]
    fn test_is_linearly_independent() {
        let independent_vectors = vec![
            bitvec![u64, Lsb0; 1, 0, 0, 1],
            bitvec![u64, Lsb0; 0, 1, 1, 0],
            bitvec![u64, Lsb0; 1, 1, 0, 1],
        ];
        assert!(is_linearly_independent(&independent_vectors));
        let dependent_vectors = vec![
            bitvec![u64, Lsb0; 1, 0, 0, 1],
            bitvec![u64, Lsb0; 0, 1, 1, 0],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
            bitvec![u64, Lsb0; 1, 1, 1, 1],
        ];
        assert!(!is_linearly_independent(&dependent_vectors));
    }
}