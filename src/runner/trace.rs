#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceMode {
    Off,
    On,
}

impl TraceMode {
    pub fn is_enabled(self) -> bool {
        matches!(self, Self::On)
    }
}
