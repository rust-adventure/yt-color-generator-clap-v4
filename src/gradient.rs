use std::{iter::zip, str::FromStr};

use clap::{arg, Args};
use owo_colors::OwoColorize;
use palette::{rgb::Rgb, Gradient, LinSrgb};
use rand::Rng;

#[derive(Args)]
pub struct GradientOptions {
    /// an ordered list of colors
    #[arg(short = 'c', long = "color")]
    colors: Vec<String>,
    /// number of steps to use in gradient
    #[arg(short, long, default_value_t = 10)]
    num_steps: usize,
    /// stop positions
    #[arg(short, long)]
    stops: Vec<f32>,
}

pub fn generate(
    GradientOptions {
        colors,
        num_steps,
        stops,
    }: &GradientOptions,
) {
    if colors.len() > 0 {
        if colors.len() != stops.len() {
            panic!("number of colors and number of steps must match:\nnumber of colors: {}\nnumber of steps: {}", colors.len(), stops.len());
        }
        let color_list = zip(stops, colors)
            .map(|(&stop, color)| {
                (
                    stop,
                    LinSrgb::from_str(color)
                        .expect("hex code")
                        .into_format(),
                )
            })
            .collect::<Vec<(f32, LinSrgb)>>();
        let gradient = Gradient::with_domain(color_list);

        for color in gradient.take(*num_steps).map(
            |Rgb {
                 red,
                 green,
                 blue,
                 standard: _,
             }| {
                owo_colors::Rgb(
                    (red * 255.0) as u8,
                    (green * 255.0) as u8,
                    (blue * 255.0) as u8,
                )
            },
        ) {
            let debug_str = "    ";
            print!("{}", debug_str.on_color(color));
        }
    } else {
        let mut rng = rand::thread_rng();
        let gradient = Gradient::new(vec![
            LinSrgb::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            LinSrgb::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            LinSrgb::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
        ]);

        let taken_colors: Vec<_> = gradient
            .take(*num_steps)
            .map(
                |Rgb {
                     red,
                     green,
                     blue,
                     standard: _,
                 }| {
                    owo_colors::Rgb(
                        (red * 255.0) as u8,
                        (green * 255.0) as u8,
                        (blue * 255.0) as u8,
                    )
                },
            )
            .collect();

        for color in taken_colors.iter() {
            let debug_str = "    ";
            print!("{}", debug_str.on_color(*color));
        }
    }
}
