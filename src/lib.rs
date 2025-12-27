pub mod math {
    pub mod bit_linear_algebra;
    pub mod sparse_matrix;
}

pub mod code {
    pub mod binary_symplectic;
    pub mod css_code;
    pub mod error_vector;
    pub mod paulis;
    pub mod quantum_code;
    pub mod stabilizer;
    pub mod stabilizer_code;
}

pub mod channel {
    pub mod bit_flip;
    pub mod depolarizing;
    pub mod error_channel;
}

pub mod prelude {
    pub use crate::channel::bit_flip::BitFlipChannel;
    pub use crate::channel::depolarizing::DepolarizingChannel;
    pub use crate::code::css_code::CssCode;
    pub use crate::code::stabilizer_code::StabilizerCode;
    pub use crate::math::sparse_matrix::BinarySparseMatrix;
}
