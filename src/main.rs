use qldpc_sim::{math::sparse_matrix::IntoSparseMatrix, prelude::*};

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
    let channel = DepolarizingChannel::new(9, 0.0001);
    let num_samples = 10000;
    let error_batch = channel.sample_batch(num_samples);
    let mut bp_decoder = BpDecoderCss::new(
        &shor_code,
        &channel,
        BpMethod::ProductSum,
        BpSchedule::Parallel,
        20,
        0.75,
        false,
    );
    let mut num_errors = 0;
    for (i, error) in error_batch.iter().enumerate() {
        let syndrome = shor_code.syndrome(error);
        let decoded_error = bp_decoder.decode(&syndrome);
        if decoded_error != *error {
            num_errors += 1;
            println!("Sample {}: Decoding failed.", i);
        }
    }
    let error_rate = num_errors as f64 / num_samples as f64;
    println!("Decoding error rate: {:.4}", error_rate);
}
