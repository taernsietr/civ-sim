use rand::Rng;
use noise::Perlin;

// mod position;

fn main() {
}

fn generate_world() {
    let mut rng = rand::thread_rng();
    Perlin::new(rng.gen::<u32>());
}

