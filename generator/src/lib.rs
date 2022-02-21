use image::{Rgb, RgbImage};
use perlin2d::PerlinNoise2D;
use std::error::Error;

pub struct GeneratorSettings {
    octaves: i32,
    amplitude: f64,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    scale: (f64, f64),
    bias: f64,
    seed: i32,
    img_size: (u32, u32),
}

impl GeneratorSettings {
    pub fn new(
        octaves: i32,
        amplitude: f64,
        frequency: f64,
        persistence: f64,
        lacunarity: f64,
        scale: (f64, f64),
        bias: f64,
        seed: i32,
        img_size: (u32, u32),
    ) -> GeneratorSettings {
        GeneratorSettings {
            octaves,
            amplitude,
            frequency,
            persistence,
            lacunarity,
            scale,
            bias,
            seed,
            img_size,
        }
    }
}

pub struct Generator<'a> {
    settings: &'a GeneratorSettings,
}

impl<'a> Generator<'a> {
    pub fn new(settings: &GeneratorSettings) -> Generator {
        Generator { settings }
    }

    pub fn generate(&self) -> Result<(), Box<dyn Error>> {
        let perlin = PerlinNoise2D::new(
            self.settings.octaves,
            self.settings.amplitude,
            self.settings.frequency,
            self.settings.persistence,
            self.settings.lacunarity,
            self.settings.scale,
            self.settings.bias,
            self.settings.seed,
        );

        let width = self.settings.img_size.0;
        let height = self.settings.img_size.1;

        let mut img = RgbImage::new(width, height);

        for x in 0..height {
            for y in 0..width {
                let noise = (perlin.get_noise(x as f64, y as f64) + 1.0) / 2.0;

                if noise > 0.5 {
                    img.put_pixel(y, x, Rgb([0, 0, 255]));
                } else {
                    img.put_pixel(y, x, Rgb([25, 51, 0]));
                }
            }
        }

        img.save("./target/island.png")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::Path;

    #[test]
    fn island_file_generated() {
        let generator_settings =
            GeneratorSettings::new(6, 1.0, 0.5, 1.0, 2.0, (200.0, 200.0), 0.5, 101, (100, 100));

        let generator = Generator::new(&generator_settings);
        generator.generate().unwrap_or_else(|err| {
            eprintln!("Cannot write island image: {}", err);
            panic!();
        });

        assert!(Path::new("./target/island.png").exists());
    }
}
