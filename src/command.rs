use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Hash, Eq, Clone, EnumIter)]
pub enum Command {
    CountLines,
    CountWords,
    CountChars,
    CountBytes
}
