use clap::{Parser, Subcommand};

pub mod gradient;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a gradient
    ///
    /// Optionally provide color stops to control the
    /// gradient's generation
    Gradient(gradient::GradientOptions),
    /// Generate a random color
    Random,
}
