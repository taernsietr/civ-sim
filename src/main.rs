use nannou::noise::{Perlin, NoiseFn};
use nannou::prelude::*;

// mod position;

const OFFSET: f64 = 0.6553;
const SCALE_FACTOR: f64 = 500.0;

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    
    let x = 4;
    let y = 4;
    let map = Perlin::new();
    let world = World::new(x, y, map);

    for tile in &world.tiles {
        let fx = tile.position.x as f32 * 100.0;
        let fy = tile.position.y as f32 * 100.0;
        let color = ((tile.value + 1.0) * 127.5) as u8;

        dbg!(&tile.value, color);

        draw.rect()
            .w(30.0)
            .h(30.0)
            .rgb8(color, color, color)
            .x_y(fx, fy); 
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::sketch(view).run();
}

#[derive(Debug)]
struct World {
    tiles: Vec<Tile>
}

impl World {
    fn new(width: usize, height: usize, map: Perlin) -> Self {
        let mut tiles: Vec<Tile> = Vec::with_capacity(width * height);
        
        for i in 0..width {
            for j in 0..height {
                tiles.push(Tile::new(i, j, map.get([(i as f64 + OFFSET) / SCALE_FACTOR, (j as f64 + OFFSET) / SCALE_FACTOR]) as f32));
            }
        }

        World {tiles}
    }
}

#[derive(Debug)]
struct Tile {
    position: Position,
    value: f32,
}

impl Tile {
    fn new(x: usize, y: usize, value: f32) -> Self {
        Tile {position: Position {x, y}, value}
    }
}

#[derive(Debug)]
struct Position { x: usize, y: usize }

/*
impl Position {
    #[allow(dead_code)]
    fn get(self) -> (usize, usize) {
        (self.x, self.y)
    }
    
    #[allow(dead_code)]
    fn set(&mut self, nx: usize, ny: usize) {
        self.x = nx;
        self.y = ny;
    }

    #[allow(dead_code)]
    fn distance_to(self, other: &Self) -> f32 {
        /* sqrt of the sum of the squares of the differences */
        ((other.x - self.x).pow(2) as f32 + (other.y - self.y).pow(2) as f32).sqrt()
    }
}
*/
