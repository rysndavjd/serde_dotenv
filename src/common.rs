#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum State {
    #[default]
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}
