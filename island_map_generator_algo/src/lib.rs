use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::{Clamp, Fbm, MultiFractal, Perlin, ScaleBias};

pub struct Generator {
    noise_map: NoiseMap,
    width: usize,
    height: usize,
}

impl Default for Generator {
    fn default() -> Self {
        Generator::new((512, 512), 6, 4.2, 1.5, 1.2, 1.0, 0.0, 9000)
    }
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
    pub fn new(
        dimensions: (usize, usize),
        octaves: usize,
        frequency: f64,
        persistence: f64,
        lacunarity: f64,
        scale: f64,
        bias: f64,
        seed: u32,
    ) -> Generator {
        let fbm = Fbm::<Perlin>::new(seed)
            .set_octaves(octaves)
            .set_frequency(frequency)
            .set_persistence(persistence)
            .set_lacunarity(lacunarity);

        let scale_bias: ScaleBias<_, Fbm<Perlin>, 2> =
            ScaleBias::new(fbm).set_scale(scale).set_bias(bias);

        let clamp: Clamp<_, ScaleBias<f64, Fbm<Perlin>, 2>, 2> = Clamp::new(scale_bias)
            .set_lower_bound(0.0)
            .set_upper_bound(1.0);

        let (w, h) = dimensions;

        let noise_map = PlaneMapBuilder::new(clamp)
            .set_size(w, h)
            .set_x_bounds(0.0, 1.0)
            .set_y_bounds(0.0, 1.0)
            .build();

        Self {
            noise_map,
            width: w,
            height: h,
        }
    }

    fn get_noise_value(&self, pixel: (usize, usize)) -> f64 {
        self.noise_map.get_value(pixel.0, pixel.1)
    }

    fn get_pixel_color(&self, pixel: (usize, usize)) -> (u8, u8, u8) {
        match self.get_noise_value(pixel) {
            x if x >= 0.95 => get_biome(Biome::HighMoutain),
            x if x >= 0.9 => get_biome(Biome::Moutain),
            x if x >= 0.8 => get_biome(Biome::Forest),
            x if x >= 0.7 => get_biome(Biome::Grass),
            x if x >= 0.5 => get_biome(Biome::Sand),
            x if x >= 0.2 => get_biome(Biome::Shore),
            x if x >= 0.1 => get_biome(Biome::Water),
            _ => get_biome(Biome::DeepWater),
        }
    }

    pub fn get_noise_map_vec_rgba_8_u_norm(&self) -> Vec<u8> {
        let mut noise = Vec::new();

        for i in 0..self.width {
            for j in 0..self.height {
                let (r, g, b) = self.get_pixel_color((i, j));
                noise.push(r);
                noise.push(g);
                noise.push(b);
                noise.push(1); // transparency always equals 1
            }
        }

        noise
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn noise_betwen_zero_and_one() {
        let generator = Generator::default();
        let noise = generator.get_noise_value((10, 15));

        assert!(noise >= 0.0 && noise <= 1.0);
    }
}
