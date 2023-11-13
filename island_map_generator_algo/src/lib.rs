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

enum Biome {
    DeepWater,
    Water,
    Shore,
    Sand,
    Grass,
    Forest,
    Moutain,
    HighMoutain,
}

fn get_biome(biome: Biome) -> (u8, u8, u8) {
    match biome {
        Biome::DeepWater => (28, 2, 198),
        Biome::Water => (14, 76, 156),
        Biome::Shore => (13, 108, 181),
        Biome::Sand => (229, 232, 135),
        Biome::Grass => (36, 159, 61),
        Biome::Forest => (19, 112, 38),
        Biome::Moutain => (128, 144, 143),
        Biome::HighMoutain => (146, 179, 177),
    }
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
        let (x, y) = (pixel.0 as f64, pixel.1 as f64);
        let raw_noise = self.noise_map.get_noise(x, y);

        (raw_noise + 1.0) / 2.0
    }

    pub fn get_pixel_color(&self, pixel: (u64, u64)) -> (u8, u8, u8) {
        match self.get_noise_value(pixel) {
            x if x >= 0.99 => get_biome(Biome::DeepWater),
            x if x >= 0.95 => get_biome(Biome::Water),
            x if x >= 0.9 => get_biome(Biome::Shore),
            x if x >= 0.85 => get_biome(Biome::Sand),
            x if x >= 0.8 => get_biome(Biome::Grass),
            x if x >= 0.7 => get_biome(Biome::Forest),
            x if x >= 0.65 => get_biome(Biome::Moutain),
            _ => get_biome(Biome::HighMoutain),
        }
    }

    pub fn set_octaves(&mut self, octaves: i32) {
        self.settings.octaves = octaves;
    }

    pub fn set_frequency(&mut self, frequency: f64) {
        self.settings.frequency = frequency;
    }

    pub fn set_persistence(&mut self, persistence: f64) {
        self.settings.persistence = persistence;
    }

    pub fn set_lacunarity(&mut self, lacunarity: f64) {
        self.settings.lacunarity = lacunarity;
    }

    pub fn set_scale(&mut self, scale: (f64, f64)) {
        self.settings.scale = scale;
    }

    pub fn set_bias(&mut self, bias: f64) {
        self.settings.bias = bias;
    }

    pub fn set_seed(&mut self, seed: i32) {
        self.settings.seed = seed;
    }

    pub fn update_generator(&mut self) {
        self.noise_map = PerlinNoise2D::new(
            self.settings.octaves,
            self.settings.amplitude,
            self.settings.frequency,
            self.settings.persistence,
            self.settings.lacunarity,
            self.settings.scale,
            self.settings.bias,
            self.settings.seed,
        )
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
