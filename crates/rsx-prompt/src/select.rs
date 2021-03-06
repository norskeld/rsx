use std::io;
use std::io::Write;
use std::fmt::{Debug, Display, Formatter};
use std::clone::Clone;

use crossterm::{cursor, queue};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{style, Attribute, Color, Print, PrintStyledContent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size as terminal_size, Clear, ClearType};

use crate::{Prompt, State, Symbols};
use crate::utils::{calculate_limit_indexes, is_event_abortable, print_state_symbol};

pub struct SelectPrompt<T> {
  message: String,
  state: State,
  items: Vec<T>,
  current: usize,
  limit: usize,
}

impl<T: Debug> Debug for SelectPrompt<T> {
  fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
    fmt
      .debug_struct("SelectPrompt")
      .field("message", &self.message)
      .field("items", &self.items)
      .finish()
  }
}

impl<T: Clone + Display> SelectPrompt<T> {
  pub fn new<S: Into<String>>(message: S, items: Vec<T>, limit: usize) -> SelectPrompt<T> {
    SelectPrompt {
      message: message.into(),
      state: State::default(),
      items,
      current: 0,
      limit,
    }
  }
}

/// TODO: This impl must be refactored:
///   - Replace `queue!` macro w/ function variant.
///   - Decouple rendering and formatting.
impl<T: Clone + Display> Prompt<T> for SelectPrompt<T> {
  fn run(&mut self) -> std::result::Result<Option<T>, crossterm::ErrorKind> {
    enable_raw_mode()?;

    // Initial render.
    self.render()?;

    loop {
      if let Event::Key(event) = read()? {
        self.handle(event)
      }

      // Re-render after handling key press.
      self.render()?;

      match self.state {
        | State::Aborted => {
          disable_raw_mode()?;
          return Ok(None);
        },
        | State::Completed => {
          disable_raw_mode()?;
          return Ok(Some(self.items[self.current].clone()));
        },
        | _ => {},
      }
    }
  }

  fn render(&mut self) -> crossterm::Result<()> {
    let mut stdout = io::stdout();
    let (_, rows) = terminal_size()?;

    let limit = self.items.len();
    let total = std::cmp::min(self.limit, (rows - 1) as usize);

    let (start_index, end_index) = calculate_limit_indexes(self.current, limit, total);

    if self.state == State::Created {
      queue!(stdout, cursor::Hide)?;
      self.state = State::Running;
    } else {
      queue!(
        stdout,
        cursor::MoveUp((end_index - start_index) as u16),
        cursor::MoveToColumn(0),
        Clear(ClearType::FromCursorDown)
      )?;
    }

    queue!(
      stdout,
      print_state_symbol(&self.state),
      Print(" "),
      PrintStyledContent(style(&self.message).attribute(Attribute::Bold))
    )?;

    if !self.state.is_done() {
      for idx in start_index..end_index {
        let choice = self.items[idx].to_string();

        let prefix = if idx == start_index && start_index > 0 {
          Symbols::ArrowUp.as_str()
        } else if idx == end_index - 1 && end_index < self.items.len() {
          Symbols::ArrowDown.as_str()
        } else {
          " "
        };

        queue!(
          stdout,
          Print("\n\r"),
          PrintStyledContent(if idx == self.current {
            style(Symbols::Pointer.as_str()).with(Color::Cyan)
          } else {
            style(" ")
          }),
          Print(format!(" {} ", prefix)),
          PrintStyledContent(if idx == self.current {
            style(choice).attribute(Attribute::Bold).with(Color::Cyan)
          } else {
            style(choice)
          }),
        )?;
      }
    }

    // if self.state == State::Completed {
    //   queue!(
    //     stdout,
    //     Print(" "),
    //     print_input_symbol(&self.state),
    //     Print(self.items[self.current].to_string())
    //   )?;
    // }

    if self.state.is_done() {
      queue!(stdout, Print("\n\r"), cursor::Show)?;
    }

    stdout.flush()?;
    crossterm::Result::Ok(())
  }

  fn handle(&mut self, event: KeyEvent) {
    if is_event_abortable(event) {
      self.state = State::Aborted;
      return;
    }

    if event.modifiers == KeyModifiers::CONTROL {
      match event.code {
        | KeyCode::Char('a') => self.current = 0,
        | KeyCode::Char('e') => self.current = self.items.len() - 1,
        | _ => {},
      }
    }

    if event.modifiers == KeyModifiers::NONE {
      match event.code {
        | KeyCode::Enter => self.state = State::Completed,
        | KeyCode::Home => self.current = 0,
        | KeyCode::End => self.current = self.items.len() - 1,
        | KeyCode::Char('k') | KeyCode::Up => self.current = self.current.saturating_sub(1),
        | KeyCode::Char('j') | KeyCode::Down => self.current = std::cmp::min(self.current + 1, self.items.len() - 1),
        | _ => {},
      }
    }
  }
}
