use generator::{self, Generator, GeneratorSettings};
use std::process;

fn main() {
    let generator_settings =
        GeneratorSettings::new(6, 1.0, 0.5, 1.0, 2.0, (200.0, 200.0), 0.5, 101, (1280, 720));

    let generator = Generator::new(&generator_settings);
    generator.generate().unwrap_or_else(|err| {
        eprintln!("Cannot write island image: {}", err);
        process::exit(1);
    });
}
