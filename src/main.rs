extern crate image;
extern crate clap;
extern crate css_color_parser;

mod cell;
mod codec;
mod game;

use clap::{App, Arg, AppSettings};
use codec::Codec;

pub fn main() {
  let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(concat!("v", env!("CARGO_PKG_VERSION")))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(concat!(env!("CARGO_PKG_DESCRIPTION"), " - ", env!("CARGO_PKG_HOMEPAGE")))
    .help_message("Print help information")
    .version_message("Print version information")
    .setting(AppSettings::ColoredHelp)
    .arg(Arg::with_name("INPUT")
         .takes_value(true)
         .required(true)
         .help("Load initial state from <INPUT>"))
    .arg(Arg::with_name("OUTPUT")
         .takes_value(true)
         .required(true)
         .help("Write final state to <OUTPUT>"))
    .arg(Arg::with_name("TICKS")
         .long("ticks")
         .takes_value(true)
         .default_value("1")
         .help("Run simulation for <TICKS> ticks"))
    .arg(Arg::with_name("ALIVE")
         .long("alive")
         .takes_value(true)
         .default_value("#000000FF")
         .help("Use <ALIVE> parsed as CCS color for live cells"))
    .arg(Arg::with_name("DEAD")
         .long("dead")
         .takes_value(true)
         .default_value("#FFFFFFFF")
         .help("Use <DEAD> parsed as CSS color for live cells"))
    .get_matches();

  let input_path = matches.value_of("INPUT")
    .expect("INPUT argument value missing");

  let output_path = matches.value_of("OUTPUT")
    .expect("OUTPUT argument value missing");

  let ticks = matches.value_of("TICKS")
    .expect("TICKS argument value missing")
    .parse::<u64>()
    .expect("Failed to parse TICKS");

  let image = image::open(&input_path)
    .expect("Failed to load input image");

  let alive = matches.value_of("ALIVE")
    .expect("ALIVE argument value missing")
    .parse()
    .expect("Failed to parse ALIVE");

  let dead  = matches.value_of("DEAD")
    .expect("DEAD argument value missing")
    .parse()
    .expect("Failed to parse DEAD");

  let codec = Codec::new(alive, dead);

  let mut game = codec.decode(image);

  for _ in 0..ticks {
    game = game.tick();
  }

  codec.encode(game)
    .save(output_path)
    .expect("failed to save output");
}
