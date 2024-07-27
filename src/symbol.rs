use std::fmt::Display;

pub enum SymbolKind {
    Input,
    Output,
    And,
    Nor,
}

impl SymbolKind {
    pub fn name(&self) -> &str {
        match self {
            SymbolKind::Input => "Input",
            SymbolKind::Output => "Output",
            SymbolKind::And => "And",
            SymbolKind::Nor => "Nor",
        }
    }
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
