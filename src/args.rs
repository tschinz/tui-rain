use clap::{Parser, ValueEnum};
use ratatui::style::Color;

/// CLI wrapper around tui-rs to create terminal rain effects.
/// Added message functionality for a more festive touch.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
  /// Type of rain effect [rain|matrix|snow|data|emoji]
  #[clap(short='t', long, default_value_t = RainType::Snow, value_enum)]
  pub rain_type: RainType,

  /// Rain density computes the number of drops based on the frame size. Lower value is denser.
  #[clap(short, long)]
  pub density: Option<usize>,

  /// Rain speed in pixels / second
  #[clap(short, long)]
  pub speed: Option<f64>,

  /// Rain speed variance
  #[clap(short = 'S', long)]
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

  /// Message to display
  #[clap(short, long)]
  pub message: Option<String>,

  /// Color of the message [black|red|green|yellow|blue|magenta|cyan|gray|darkgray|lightred|lightgreen|lightyellow|lightblue|lightmagenta|lightcyan|white]
  #[clap(short = 'n', long)]
  pub message_color: Option<Color>,

  /// Message speed in pixels / second
  #[clap(short = 'o', long)]
  pub message_speed: Option<f64>,

  /// Verbose mode
  #[clap(short, long, default_value_t = false)]
  pub verbose: bool,
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
        self.message_color.get_or_insert(Color::Blue);
        self.message_speed.get_or_insert(2.0);
        self.message.get_or_insert(String::from(
          "🎄  Season’s Greetings from SPL! 🎄\n\nThank you for your trust and collaboration this year.\n\nWe wish you a joyful holiday season 🎄\nand a successful start to the new year 🌟.\n\nLooking forward to more innovation together in 2025! 🚀\n\n- The Smart Process Lab Team -",
        ));
      }
      RainType::Matrix => {
        self.density.get_or_insert(50);
        self.speed.get_or_insert(5.0);
        self.variance_speed.get_or_insert(0.5);
        self.lifespan_tail.get_or_insert(3000);
        self.color.get_or_insert(Color::LightGreen);
        self.head_color.get_or_insert(Color::White);
        self.effect_dim.get_or_insert(true);
        self.message_color.get_or_insert(Color::Green);
        self.message_speed.get_or_insert(2.0);
        self.message.get_or_insert(String::from(
          "🎄  Season’s Greetings from SPL! 🎄\n\nThank you for your trust and collaboration this year.\n\nWe wish you a joyful holiday season 🎄\nand a successful start to the new year 🌟.\n\nLooking forward to more innovation together in 2025! 🚀\n\n- The Smart Process Lab Team -",
        ));
      }
      RainType::Snow => {
        self.density.get_or_insert(30);
        self.speed.get_or_insert(2.0);
        self.variance_speed.get_or_insert(0.3);
        self.lifespan_tail.get_or_insert(500);
        self.color.get_or_insert(Color::White);
        self.head_color.get_or_insert(Color::White);
        self.effect_dim.get_or_insert(true);
        self.message_color.get_or_insert(Color::Gray);
        self.message_speed.get_or_insert(2.0);
        self.message.get_or_insert(String::from(
          "🎄  Season’s Greetings from SPL! 🎄\n\nThank you for your trust and collaboration this year.\n\nWe wish you a joyful holiday season 🎄\nand a successful start to the new year 🌟.\n\nLooking forward to more innovation together in 2025! 🚀\n\n- The Smart Process Lab Team -",
        ));
      }
      RainType::Data => {
        self.density.get_or_insert(70);
        self.speed.get_or_insert(2.0);
        self.variance_speed.get_or_insert(3.0);
        self.lifespan_tail.get_or_insert(1000);
        self.color.get_or_insert(Color::LightBlue);
        self.head_color.get_or_insert(Color::White);
        self.effect_dim.get_or_insert(true);
        self.message_color.get_or_insert(Color::Blue);
        self.message_speed.get_or_insert(2.0);
        self.message.get_or_insert(String::from(
          "🎄  Season’s Greetings from SPL! 🎄\n\nThank you for your trust and collaboration this year.\n\nWe wish you a joyful holiday season 🎄\nand a successful start to the new year 🌟.\n\nLooking forward to more innovation together in 2025! 🚀\n\n- The Smart Process Lab Team -",
        ));
      }
      RainType::Emoji => {
        self.density.get_or_insert(20);
        self.speed.get_or_insert(10.0);
        self.variance_speed.get_or_insert(0.1);
        self.lifespan_tail.get_or_insert(500);
        self.color.get_or_insert(Color::White);
        self.head_color.get_or_insert(Color::White);
        self.effect_dim.get_or_insert(true);
        self.message_color.get_or_insert(Color::Yellow);
        self.message_speed.get_or_insert(2.0);
        self.message.get_or_insert(String::from(
          "🎄  Season’s Greetings from SPL! 🎄\n\nThank you for your trust and collaboration this year.\n\nWe wish you a joyful holiday season 🎄\nand a successful start to the new year 🌟.\n\nLooking forward to more innovation together in 2025! 🚀\n\n- The Smart Process Lab Team -",
        ));
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
