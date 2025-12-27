/// StabilizerCodeやCssCodeを束ねるためのTrait
/// 今後拡張される可能性が高いため、別ファイルに分離している
pub trait QuantumCode {
    fn get_code_name(&self) -> &str;
    fn get_n(&self) -> usize;
    fn get_k(&self) -> usize;
}
