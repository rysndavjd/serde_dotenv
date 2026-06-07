#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum QuoteState {
    #[default]
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}
