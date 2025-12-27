/// StabilizerCodeやCssCodeを束ねるためのTrait
/// 今後拡張される可能性が高いため、別ファイルに分離している
pub trait QuantumCode {
    fn code_name(&self) -> &str;
    fn n(&self) -> usize;
    fn k(&self) -> usize;
}
