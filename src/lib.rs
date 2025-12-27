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
    pub mod error_channel;
    pub mod depolarizing;
    pub mod bit_flip;
}

pub mod prelude {
    pub use crate::code::binary_symplectic::BinarySymplecticVector;
    pub use crate::code::paulis::Paulis;
    pub use crate::code::stabilizer::StabilizerGroup;
    pub use crate::code::stabilizer_code::StabilizerCode;
    pub use bitvec::prelude::*;
}
