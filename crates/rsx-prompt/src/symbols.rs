#[allow(dead_code)]
pub enum Symbols {
  ArrowUp,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  Tick,
  Cross,
  MiddleDot,
  Pointer,
  PointerSmall,
  QuestionMark,
}

impl Symbols {
  pub fn as_str(&self) -> &'static str {
    match self {
      | Symbols::ArrowUp => "↑",
      | Symbols::ArrowDown => "↓",
      | Symbols::ArrowLeft => "←",
      | Symbols::ArrowRight => "→",
      | Symbols::Tick => "✔",
      | Symbols::Cross => "𐄂",
      | Symbols::MiddleDot => "·",
      | Symbols::Pointer => "❯",
      | Symbols::PointerSmall => "›",
      | Symbols::QuestionMark => "?",
    }
  }
}
