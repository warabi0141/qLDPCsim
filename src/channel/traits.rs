use crate::code::error_vector::ErrorVector;

pub trait ErrorChannel {
    fn sample(&self) -> ErrorVector;
    fn sample_batch(&self, num_samples: usize) -> Vec<ErrorVector>;
    fn x_error_rate(&self) -> f64;
    fn y_error_rate(&self) -> f64;
    fn z_error_rate(&self) -> f64;
}
