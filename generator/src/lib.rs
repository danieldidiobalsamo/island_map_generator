use perlin2d::PerlinNoise2D;

#[derive(Clone)]
pub struct GeneratorSettings {
    octaves: i32,
    amplitude: f64,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    scale: (f64, f64),
    bias: f64,
    seed: i32,
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
        }
    }
}

pub struct Generator {
    settings: GeneratorSettings,
    noise_map: PerlinNoise2D,
}

impl Generator {
    pub fn new(settings: GeneratorSettings) -> Generator {
        Self {
            settings: settings.clone(),
            noise_map: PerlinNoise2D::new(
                settings.octaves,
                settings.amplitude,
                settings.frequency,
                settings.persistence,
                settings.lacunarity,
                settings.scale,
                settings.bias,
                settings.seed,
            ),
        }
    }

    fn get_noise_value(&self, pixel: (u64, u64)) -> f64 {
        let x = pixel.0;
        let y = pixel.1;

        let raw_noise = self.noise_map.get_noise(x as f64, y as f64);

        (raw_noise + 1.0) / 2.0
    }

    pub fn get_pixel_color(&self, pixel: (u64, u64)) -> (u8, u8, u8) {
        let noise = self.get_noise_value(pixel);
        let color;

        if noise > 0.5 {
            color = (0, 0, 255);
        } else {
            color = (25, 51, 0);
        }

        color
    }

    pub fn set_octaves(&mut self, octave: i32) {
        self.settings.octaves = octave;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn noise_betwen_zero_and_one() {
        let generator_settings =
            GeneratorSettings::new(6, 1.0, 0.5, 1.0, 2.0, (200.0, 200.0), 0.5, 101);

        let generator = Generator::new(generator_settings);
        let noise = generator.get_noise_value((10, 25));

        assert!(noise >= 0.0 && noise <= 1.0);
    }
}
