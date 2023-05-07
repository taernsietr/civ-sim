use noise::NoiseFn;

pub struct NoiseSampler<'a> {
    xoff: f64,
    yoff: Option<f64>,
    zoff: Option<f64>,
    xscale: f64,
    yscale: Option<f64>,
    zscale: Option<f64>,
    weight: f64,
    noise_map: &'a dyn NoiseFn<f64, 3>
}

impl<'a> NoiseSampler<'a> {
    pub fn build_samplers(values: Vec<(f64, Option<f64>, Option<f64>, f64, Option<f64>, Option<f64>, f64, &'a impl NoiseFn<f64, 3>)>) -> Vec<NoiseSampler<'a>> {
        let mut samplers = Vec::new();
        for i in values { samplers.push(
            NoiseSampler { 
                xoff: i.0, 
                yoff: i.1, 
                zoff: i.2, 
                xscale: i.3, 
                yscale: i.4, 
                zscale: i.5,
                weight: i.6,
                noise_map: i.7
            });
        }
        samplers
    }

    pub fn get_point_value(x: u32, y: Option<u32>, z: Option<u32>, samplers: Vec<NoiseSampler>) -> f64 {
        let mut result: f64 = 0.0;
        let mut total_wgt: f64 = 0.0;

        for sampler in samplers {
            result += sampler.sample(x, y, z);
            total_wgt += sampler.weight;
        }
        result / total_wgt
    }

    fn sample(&self, x: u32, y: Option<u32>, z: Option<u32>) -> f64 {
        let value = self.noise_map.get([
            ((x as f64 + self.xoff) / self.xscale),
            if let Some(y) = y { (y as f64 + self.yoff.unwrap()) / self.yscale.unwrap() } else { 0.0 },
            if let Some(z) = z { (z as f64 + self.zoff.unwrap()) / self.zscale.unwrap() } else { 0.0 }
        ]);
        value * self.weight
    }
}

