use crate::channel::error_channel::ErrorChannel;
use crate::code::error_vector::ErrorVector;
use bitvec::prelude::*;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

pub struct DepolarizingChannel {
    num_qubits: usize,
    error_rate: f64,
    distribution: WeightedIndex<f64>,
}

impl DepolarizingChannel {
    pub fn new(num_qubits: usize, error_rate: f64) -> Self {
        assert!(
            error_rate >= 0.0 && error_rate <= 1.0,
            "Error rate must be between 0 and 1"
        );
        let weights = [
            1.0 - error_rate,
            error_rate / 3.0,
            error_rate / 3.0,
            error_rate / 3.0,
        ];
        let distribution = WeightedIndex::new(&weights).unwrap();

        Self {
            num_qubits,
            error_rate,
            distribution,
        }
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn error_rate(&self) -> f64 {
        self.error_rate
    }
}

impl ErrorChannel for DepolarizingChannel {
    fn sample(&self) -> ErrorVector {
        let mut rng = rand::rng();
        let mut x_part = bitvec![u64, Lsb0; 0; self.num_qubits];
        let mut z_part = bitvec![u64, Lsb0; 0; self.num_qubits];

        for qubit_idx in 0..self.num_qubits {
            let error_type = self.distribution.sample(&mut rng);
            match error_type {
                0 => {
                    // No error
                }
                1 => {
                    // X error
                    x_part.set(qubit_idx, true);
                }
                2 => {
                    // Y error
                    x_part.set(qubit_idx, true);
                    z_part.set(qubit_idx, true);
                }
                3 => {
                    // Z error
                    z_part.set(qubit_idx, true);
                }
                _ => unreachable!(),
            }
        }

        ErrorVector::new(x_part, z_part)
    }

    fn sample_batch(&self, num_samples: usize) -> Vec<ErrorVector> {
        (0..num_samples).map(|_| self.sample()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depolarizing_channel_sample() {
        let channel = DepolarizingChannel::new(5, 0.1);
        let error_vector = channel.sample();
        assert_eq!(error_vector.num_qubits(), 5);
    }

    #[test]
    fn test_depolarizing_channel_sample_batch() {
        let channel = DepolarizingChannel::new(5, 0.1);
        let error_vectors = channel.sample_batch(10);
        assert_eq!(error_vectors.len(), 10);
        for ev in error_vectors {
            assert_eq!(ev.num_qubits(), 5);
        }
    }
}
