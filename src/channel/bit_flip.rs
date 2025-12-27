use crate::channel::error_channel::ErrorChannel;
use crate::code::error_vector::ErrorVector;
use bitvec::prelude::*;
use rand::distr::Bernoulli;
use rand::prelude::*;

pub struct BitFlipChannel {
    num_qubits: usize,
    error_rate: f64,
}

impl BitFlipChannel {
    pub fn new(num_qubits: usize, error_rate: f64) -> Self {
        assert!(
            error_rate >= 0.0 && error_rate <= 1.0,
            "Error rate must be between 0 and 1"
        );

        Self {
            num_qubits,
            error_rate,
        }
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn error_rate(&self) -> f64 {
        self.error_rate
    }
}

impl ErrorChannel for BitFlipChannel {
    fn sample(&self) -> ErrorVector {
        let mut rng = rand::rng();
        let mut x_part = bitvec![u64, Lsb0; 0; self.num_qubits];
        let z_part = bitvec![u64, Lsb0; 0; self.num_qubits];

        let dist = Bernoulli::new(self.error_rate).unwrap();

        for qubit_idx in 0..self.num_qubits {
            if dist.sample(&mut rng) {
                x_part.set(qubit_idx, true);
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
    fn test_bit_flip_channel_sample() {
        let channel = BitFlipChannel::new(5, 0.2);
        let error_vector = channel.sample();
        assert_eq!(error_vector.num_qubits(), 5);
    }

    #[test]
    fn test_bit_flip_channel_sample_batch() {
        let channel = BitFlipChannel::new(5, 0.2);
        let error_vectors = channel.sample_batch(10);
        assert_eq!(error_vectors.len(), 10);
    }
}
