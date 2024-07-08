use std::fmt::Display;

pub enum SymbolKind {
    And,
}

impl SymbolKind {
    pub fn name(&self) -> &str {
        match self {
            SymbolKind::And => "And",
        }
    }
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
