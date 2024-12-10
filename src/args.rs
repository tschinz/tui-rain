use clap::{Parser, ValueEnum};
use ratatui::style::Color;
use std::fmt;

/// CLI wrapper around tui-rs to create terminal rain effects.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
  /// Type of rain effect [rain|matrix|snow|data|emoji]
  #[clap(short='t', long, default_value_t = RainType::Rain, value_enum)]
  pub rain_type: RainType,

  /// Rain density computes the number of drops based on the frame size. Lower value is denser.
  #[clap(short, long)]
  pub density: Option<usize>,

  /// Rain speed in pixels / second
  #[clap(short, long)]
  pub speed: Option<f64>,

  /// Rain speed variance
  #[clap(short, long)]
  pub variance_speed: Option<f64>,

  /// Tail lifespan in milliseconds
  #[clap(short, long)]
  pub lifespan_tail: Option<u64>,

  /// Color of the rain [black|red|green|yellow|blue|magenta|cyan|gray|darkgray|lightred|lightgreen|lightyellow|lightblue|lightmagenta|lightcyan|white]
  #[clap(short, long)]
  pub color: Option<Color>,

  /// Color of the rain [black|red|green|yellow|blue|magenta|cyan|gray|darkgray|lightred|lightgreen|lightyellow|lightblue|lightmagenta|lightcyan|white]
  #[clap(short = 'k', long)]
  pub head_color: Option<Color>,

  /// Dim effect
  #[clap(short, long)]
  pub effect_dim: Option<bool>,
}

impl Args {
  pub fn sanitize(&mut self) {
    match self.rain_type {
      RainType::Rain => {
        self.density.get_or_insert(30);
        self.speed.get_or_insert(2.0);
        self.variance_speed.get_or_insert(10.0);
        self.lifespan_tail.get_or_insert(500);
        self.color.get_or_insert(Color::LightBlue);
        self.head_color.get_or_insert(Color::White);
        self.effect_dim.get_or_insert(true);
      }
      RainType::Matrix => {
        self.density.get_or_insert(50);
        self.speed.get_or_insert(5.0);
        self.variance_speed.get_or_insert(0.5);
        self.lifespan_tail.get_or_insert(3000);
        self.color.get_or_insert(Color::LightGreen);
        self.head_color.get_or_insert(Color::White);
        self.effect_dim.get_or_insert(true);
      }
      RainType::Snow => {
        self.density.get_or_insert(30);
        self.speed.get_or_insert(2.0);
        self.variance_speed.get_or_insert(0.3);
        self.lifespan_tail.get_or_insert(500);
        self.color.get_or_insert(Color::White);
        self.head_color.get_or_insert(Color::White);
        self.effect_dim.get_or_insert(true);
      }
      RainType::Data => {
        self.density.get_or_insert(70);
        self.speed.get_or_insert(2.0);
        self.variance_speed.get_or_insert(3.0);
        self.lifespan_tail.get_or_insert(1000);
        self.color.get_or_insert(Color::LightBlue);
        self.head_color.get_or_insert(Color::White);
        self.effect_dim.get_or_insert(true);
      }
      RainType::Emoji => {
        self.density.get_or_insert(20);
        self.speed.get_or_insert(10.0);
        self.variance_speed.get_or_insert(0.1);
        self.lifespan_tail.get_or_insert(500);
        self.color.get_or_insert(Color::White);
        self.head_color.get_or_insert(Color::White);
        self.effect_dim.get_or_insert(true);
      }
    }
  }
}

/// Enum for rain effects
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RainType {
  /// Rain effect
  Rain,
  /// Matrix effect
  Matrix,
  /// Snow effect
  Snow,
  /// Data effect
  Data,
  /// Emoji effect
  Emoji,
}
impl fmt::Display for RainType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      RainType::Rain => write!(f, "Rain"),
      RainType::Matrix => write!(f, "Matrix"),
      RainType::Snow => write!(f, "Snow"),
      RainType::Data => write!(f, "Data"),
      RainType::Emoji => write!(f, "Emoji"),
    }
  }
}
