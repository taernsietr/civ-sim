use noise::NoiseFn;

#[derive(Default, Debug)]
pub struct SamplingParameters {
    pub xoff: f64,
    pub yoff: f64,
    pub zoff: f64,
    pub xscale: f64,
    pub yscale: f64,
    pub zscale: f64,
    pub weight: f64
}

#[derive(Default, Debug)]
pub struct NoiseSampler {
    values: Vec<SamplingParameters>,
    // noise_map: &'a dyn NoiseFn<f64, 3>
    noise_map: noise::OpenSimplex
}

impl NoiseSampler {
    pub fn new(noise_map: noise::OpenSimplex) -> NoiseSampler {
        NoiseSampler {
            noise_map,
            ..NoiseSampler::default()
        }
    }

    pub fn add_values(&mut self, values: [f64; 7]) -> &mut Self {
        self.values.push(
            SamplingParameters {
                xoff: values[0],
                yoff: values[1],
                zoff: values[2],
                xscale: values[3],
                yscale: values[4],
                zscale: values[5],
                weight: values[6]
            }
        );
        self
    }

    // TODO: change into fold?
    pub fn get_point_value(&self, x: u32, y: u32, z: u32) -> f64 {
        let mut result: f64 = 0.0;
        let mut total_wgt: f64 = 0.0;

        for sample in &self.values {
            result += sample.weight * self.noise_map.get([
                (x as f64 + sample.xoff) / sample.xscale,
                (y as f64 + sample.yoff) / sample.yscale,
                (z as f64 + sample.zoff) / sample.zscale
            ]);
            total_wgt += sample.weight;
        }
        result / total_wgt
    }
}

