use crate::code::error_vector::ErrorVector;
use crate::code::error_vector::Syndrome;

pub trait Decoder {
    fn name(&self) -> &str;
    fn decode(&self, syndrome: &Syndrome) -> ErrorVector;
}
