use qldpc_sim::{
    channel::error_channel::ErrorChannel, code::error_vector::ErrorVector,
    math::sparse_matrix::IntoSparseMatrix, prelude::*,
};

fn main() {
    let hz = vec![
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1],
    ];
    let hx = vec![
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 1, 1, 1],
    ];
    let shor_code = CssCode::from_parity_check_matrices(
        "ShorCode",
        hz.into_sparse_matrix(),
        hx.into_sparse_matrix(),
    );
    let channel = DepolarizingChannel::new(9, 0.001);
    let num_samples = 10000;
    let error_batch = channel.sample_batch(num_samples);
    let mut num_error = 0;
    for (i, error) in error_batch.iter().enumerate() {
        num_error += error.num_errors();
    }
    let error_rate = num_error as f64 / (9.0 * num_samples as f64);
    println!("Sampled error rate: {}", error_rate);
}
