use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{style, Color, PrintStyledContent};

use super::{State, Symbols};

pub fn is_event_abortable(event: KeyEvent) -> bool {
  let KeyEvent { modifiers, code } = event;

  match (modifiers, code) {
    | (KeyModifiers::CONTROL, KeyCode::Char('c')) => true,
    | (KeyModifiers::CONTROL, KeyCode::Char('d')) => true,
    | (KeyModifiers::NONE, KeyCode::Esc) => true,
    | _ => false,
  }
}

pub fn calculate_limit_indexes(current: usize, total: usize, limit: usize) -> (usize, usize) {
  let start_index = std::cmp::min(
    total.checked_sub(limit).unwrap_or(0),
    current.checked_sub(limit / 2).unwrap_or(0),
  );

  let end_index = std::cmp::min(start_index + limit, total);

  (start_index, end_index)
}

pub fn print_state_symbol(state: &State) -> PrintStyledContent<&'static str> {
  let content = match state {
    | State::Aborted => style(Symbols::Cross.as_str()).with(Color::Red),
    | State::Completed => style(Symbols::Tick.as_str()).with(Color::Green),
    | _ => style(Symbols::QuestionMark.as_str()).with(Color::Magenta),
  };

  PrintStyledContent(content)
}

#[allow(dead_code)]
pub fn print_input_symbol(state: &State) -> PrintStyledContent<String> {
  let content = match state {
    | State::Aborted => "".to_string(),
    | State::Completed => format!("{} ", Symbols::MiddleDot.as_str()),
    | _ => format!("{} ", Symbols::PointerSmall.as_str()),
  };

  PrintStyledContent(style(content).with(Color::Grey))
}
