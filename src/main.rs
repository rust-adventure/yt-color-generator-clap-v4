use clap::Parser;
use gen_color::{gradient, Cli, Commands};
use owo_colors::OwoColorize;
use rand::Rng;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gradient(gradient_options) => {
            gradient::generate(gradient_options)
        }
        Commands::Random => {
            let mut rng = rand::thread_rng();

            let color = owo_colors::Rgb(
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            );
            let debug_str = "    ";
            print!(
                "{} #{:x}{:x}{:x}",
                debug_str.on_color(color),
                color.0,
                color.1,
                color.2
            );
        }
    }
}
