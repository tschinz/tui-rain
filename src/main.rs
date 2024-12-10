mod args;
mod utils;
use std::time::Duration;

use clap::Parser;
use std::error::Error;
use tui_rain_cli::{CharacterSet, Rain, RainDensity, RainSpeed};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut args = args::Args::parse();
  args.sanitize();
  let rain_density = RainDensity::Relative {
    sparseness: args.density.unwrap(),
  };
  let rain_speed = RainSpeed::Absolute { speed: args.speed.unwrap() };

  let rain_speed_variance = args.variance_speed.unwrap_or(0.5);
  let tail_lifespan = Duration::from_millis(args.lifespan_tail.unwrap_or(250));
  match args.rain_type {
    args::RainType::Rain => {
      utils::render_rain(Box::new(move |elapsed| {
        Rain::new_rain(elapsed)
          .with_rain_density(rain_density)
          .with_rain_speed(rain_speed)
          .with_rain_speed_variance(rain_speed_variance)
          .with_tail_lifespan(tail_lifespan)
          .with_color(args.color.unwrap())
          .with_head_color(args.head_color.unwrap())
          .with_bold_dim_effect(args.effect_dim.unwrap())
      }))
      .await
    }
    args::RainType::Matrix => {
      utils::render_rain(Box::new(move |elapsed| {
        Rain::new_matrix(elapsed)
          .with_rain_density(rain_density)
          .with_rain_speed(rain_speed)
          .with_rain_speed_variance(rain_speed_variance)
          .with_tail_lifespan(tail_lifespan)
          .with_color(args.color.unwrap())
          .with_head_color(args.head_color.unwrap())
          .with_bold_dim_effect(args.effect_dim.unwrap())
      }))
      .await
    }
    args::RainType::Snow => {
      utils::render_rain(Box::new(move |elapsed| {
        Rain::new_snow(elapsed)
          .with_rain_density(rain_density)
          .with_rain_speed(rain_speed)
          .with_rain_speed_variance(rain_speed_variance)
          .with_tail_lifespan(tail_lifespan)
          .with_color(args.color.unwrap())
          .with_head_color(args.head_color.unwrap())
          .with_bold_dim_effect(args.effect_dim.unwrap())
      }))
      .await
    }
    args::RainType::Data => {
      utils::render_rain(Box::new(move |elapsed| {
        Rain::new_matrix(elapsed)
          .with_rain_density(rain_density)
          .with_rain_speed(rain_speed)
          .with_rain_speed_variance(rain_speed_variance)
          .with_tail_lifespan(tail_lifespan)
          .with_color(args.color.unwrap())
          .with_head_color(args.head_color.unwrap())
          .with_bold_dim_effect(args.effect_dim.unwrap())
          .with_character_set(CharacterSet::Explicit { options: vec!['0', '1'] })
      }))
      .await
    }
    args::RainType::Emoji => {
      utils::render_rain(Box::new(move |elapsed| {
        Rain::new_emoji_soup(elapsed)
          .with_rain_density(rain_density)
          .with_rain_speed(rain_speed)
          .with_rain_speed_variance(rain_speed_variance)
          .with_tail_lifespan(tail_lifespan)
          .with_color(args.color.unwrap())
          .with_head_color(args.head_color.unwrap())
          .with_bold_dim_effect(args.effect_dim.unwrap())
      }))
      .await
    }
  }
}
