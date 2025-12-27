use crate::code::error_vector::ErrorVector;

pub trait ErrorChannel {
    fn sample(&self) -> ErrorVector;
    fn sample_batch(&self, num_samples: usize) -> Vec<ErrorVector>;
}