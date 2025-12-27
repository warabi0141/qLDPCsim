use crate::code::css_code::CssCode;
use crate::code::error_vector::{ErrorVector, Syndrome};
use crate::decoder::bp::BpDecoder;
use crate::decoder::bp::BpMethod;
use crate::decoder::bp::BpSchedule;
use crate::decoder::traits::Decoder;
use crate::prelude::ErrorChannel;

pub struct BpDecoderCss {
    decoder_x: BpDecoder,
    decoder_z: BpDecoder,
}

impl BpDecoderCss {
    pub fn new<C: ErrorChannel>(
        code: &CssCode,
        error_channel: &C,
        bp_method: BpMethod,
        schedule: BpSchedule,
        max_iterations: usize,
        ms_scaling_factor: f64,
        random_serial_schedule: bool,
    ) -> Self {
        let hz = code.hz().clone();
        let hx = code.hx().clone();

        let error_rate_x = error_channel.x_error_rate() + error_channel.y_error_rate();
        let error_rate_z = error_channel.z_error_rate() + error_channel.y_error_rate();

        let channel_probabilities_x = vec![error_rate_x; code.num_qubits()];
        let channel_probabilities_z = vec![error_rate_z; code.num_qubits()];

        let decoder_x = BpDecoder::from_pcm(
            hx,
            bp_method,
            schedule,
            max_iterations,
            ms_scaling_factor,
            random_serial_schedule,
            channel_probabilities_z,
        );

        let decoder_z = BpDecoder::from_pcm(
            hz,
            bp_method,
            schedule,
            max_iterations,
            ms_scaling_factor,
            random_serial_schedule,
            channel_probabilities_x,
        );

        BpDecoderCss {
            decoder_x,
            decoder_z,
        }
    }
}

impl Decoder for BpDecoderCss {
    fn name(&self) -> &str {
        "BP Decoder for CSS Codes"
    }

    fn decode(&mut self, syndrome: &Syndrome) -> ErrorVector {
        let syndrome_x = syndrome
            .x_syndrome()
            .as_bitslice()
            .iter()
            .map(|bit| if *bit { 1 } else { 0 })
            .collect::<Vec<u8>>();
        let syndrome_z = syndrome
            .z_syndrome()
            .as_bitslice()
            .iter()
            .map(|bit| if *bit { 1 } else { 0 })
            .collect::<Vec<u8>>();

        let error_z = self.decoder_x.decode(&syndrome_x);
        let error_x = self.decoder_z.decode(&syndrome_z);

        ErrorVector::from_u8vec(error_x, error_z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::sparse_matrix::BinarySparseMatrix;
    use bitvec::prelude::*;

    #[test]
    fn test_bp_decoder_css_no_error() {
        let hz_row_adj = vec![
            vec![0, 1],
            vec![1, 2],
            vec![3, 4],
            vec![4, 5],
            vec![6, 7],
            vec![7, 8],
        ];
        let hx_row_adj = vec![vec![0, 1, 2, 3, 4, 5], vec![3, 4, 5, 6, 7, 8]];
        let hz = BinarySparseMatrix::from_row_adj(6, 9, hz_row_adj);
        let hx = BinarySparseMatrix::from_row_adj(2, 9, hx_row_adj);
        let css_code = CssCode::from_parity_check_matrices("TestCSS", hz, hx);

        let channel = crate::channel::bit_flip::BitFlipChannel::new(9, 0.1);
        let mut decoder = BpDecoderCss::new(
            &css_code,
            &channel,
            BpMethod::ProductSum,
            BpSchedule::Parallel,
            10,
            0.75,
            false,
        );
        let zero_syndrome = Syndrome::new(
            bitvec![u64, Lsb0; 0; css_code.num_stabilizers()],
            bitvec![u64, Lsb0; 0; css_code.num_stabilizers()],
        );

        let decoded_error = decoder.decode(&zero_syndrome);
        assert_eq!(decoded_error.num_errors(), 0);
    }

    #[test]
    fn test_bp_decoder_css_one_error() {
        let hz_row_adj = vec![
            vec![0, 1],
            vec![1, 2],
            vec![3, 4],
            vec![4, 5],
            vec![6, 7],
            vec![7, 8],
        ];
        let hx_row_adj = vec![vec![0, 1, 2, 3, 4, 5], vec![3, 4, 5, 6, 7, 8]];
        let hz = BinarySparseMatrix::from_row_adj(6, 9, hz_row_adj);
        let hx = BinarySparseMatrix::from_row_adj(2, 9, hx_row_adj);
        let css_code = CssCode::from_parity_check_matrices("TestCSS", hz, hx);

        let channel = crate::channel::bit_flip::BitFlipChannel::new(9, 0.1);
        let mut decoder = BpDecoderCss::new(
            &css_code,
            &channel,
            BpMethod::ProductSum,
            BpSchedule::Parallel,
            10,
            0.75,
            false,
        );

        // Introduce an X error on qubit 0
        let error_vector = ErrorVector::from_u8vec(vec![1, 0, 0, 0, 0, 0, 0, 0, 0], vec![0; 9]);
        let syndrome = css_code.syndrome(&error_vector);

        let decoded_error = decoder.decode(&syndrome);
        assert_eq!(
            decoded_error.x_part(),
            &bitvec![u64, Lsb0; 1, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(decoded_error.z_part(), &bitvec![u64, Lsb0; 0; 9]);
    }
}
