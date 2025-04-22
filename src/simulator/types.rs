#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cycle(pub u32);

impl Cycle {
    pub fn new(n: u32) -> Self {
        Self(n)
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn diff(&self, other: &Cycle) -> u32 {
        self.0.saturating_sub(other.0)
    }
}

impl std::fmt::Display for Cycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cycle {}", self.0)
    }
}
