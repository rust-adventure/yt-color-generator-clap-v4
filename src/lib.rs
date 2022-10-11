use clap::{Parser, Subcommand};
use miette::Diagnostic;
use thiserror::Error;

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
    /// Optionally provide color stops to control
    /// the gradient's generation
    Gradient(gradient::GradientOptions),
    /// Generate a random color
    Random,
}

#[derive(Error, Diagnostic, Debug)]
pub enum ColorGenerationError {
    #[error(transparent)]
    #[diagnostic(code(gen_color::io_error))]
    IoError(#[from] std::io::Error),

    #[error(
        "number of colors and number of steps must match"
    )]
    #[diagnostic(code(
        gen_color::colors_and_steps_mismatch
    ))]
    ColorsAndStepsMustMatch {
        #[source_code]
        input: String,
        #[help]
        advice: String,
        #[label("colors")]
        color_src: (usize, usize),
        #[label("stops")]
        stops_src: (usize, usize),
        // color_count: usize,
        // stop_count: usize,
    },
}
