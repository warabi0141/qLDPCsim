pub mod math {
    pub mod bit_linear_algebra;
    pub mod sparse_matrix;
}

pub mod code {
    pub mod binary_symplectic;
    pub mod css_code;
    pub mod error_vector;
    pub mod paulis;
    pub mod stabilizer;
    pub mod stabilizer_code;
    pub mod traits;
}

pub mod channel {
    pub mod bit_flip;
    pub mod depolarizing;
    pub mod traits;
}

pub mod decoder {
    pub mod bp;
    pub mod bp_css;
    pub mod traits;
}

pub mod prelude {
    pub use crate::channel::bit_flip::BitFlipChannel;
    pub use crate::channel::depolarizing::DepolarizingChannel;
    pub use crate::channel::traits::ErrorChannel;
    pub use crate::code::css_code::CssCode;
    pub use crate::code::stabilizer_code::StabilizerCode;
    pub use crate::code::traits::QuantumCode;
    pub use crate::decoder::traits::Decoder;
    pub use crate::math::sparse_matrix::BinarySparseMatrix;
}
