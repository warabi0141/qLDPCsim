use qldpc_sim::{math::sparse_matrix::IntoSparseMatrix, prelude::*};
use rayon::prelude::*;

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
    let num_samples = 1000000;
    let error_batch = channel.sample_batch(num_samples);

    let num_errors: usize = error_batch
        .par_iter()  // ← 並列イテレーター
        .filter(|&error| {
            let syndrome = shor_code.syndrome(error);
            let mut decoder = BpDecoderCss::new(
                &shor_code,
                &channel,
                BpMethod::ProductSum,
                BpSchedule::Parallel,
                20,
                0.75,
                false,
            );
            let decoded_error = decoder.decode(&syndrome);
            decoded_error != *error
        })
        .count();  // ← true の個数をカウント

    let error_rate = num_errors as f64 / num_samples as f64;
    println!("Decoding error rate: {:.4}", error_rate);
}
