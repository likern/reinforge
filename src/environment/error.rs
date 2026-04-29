#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvironmentError {
    Zero,
    TooSmall { actual: usize, min: usize },
    TooLarge { actual: usize, max: usize },
}
