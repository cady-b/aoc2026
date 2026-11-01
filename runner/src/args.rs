use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
  #[arg(short, long, help = "The days to run", value_delimiter = ' ', num_args = 0..)]
  pub days:     Vec<u8>,
  #[arg(long, help = "CPU goes vroooommm")]
  pub parallel: bool,
}
