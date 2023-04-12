use nannou::noise::{Perlin, NoiseFn, Seedable};
use nannou::prelude::*;
use rand::Rng;

const WINDOW_WIDTH: u32 = 240;
const WINDOW_HEIGHT: u32 = 160;
const PERLIN_OFFSET: f64 = 0.6553;

struct Model { processed_world: ProcessedWorld }
#[derive(Debug)]
struct ProcessedWorld { processed_tiles: Vec<ProcessedTile>, tile_size: f32 }
#[derive(Debug)]
struct ProcessedTile {
    fx: f32,
    fy: f32,
    rgb: [u8; 3]
}

impl ProcessedWorld {
    fn new(world: World, tile_size: f32, left_offset: f32, top_offset: f32) -> ProcessedWorld {
        let mut processed_tiles = Vec::new();

        for tile in world.tiles {
            let fx = tile_size * tile.position.x as f32 + left_offset + tile_size / 2.0;
            let fy = -tile_size * tile.position.y as f32 + top_offset - tile_size / 2.0;
            let rgb = [tile.values[0] as u8, tile.values[1] as u8, tile.values[2] as u8];

            processed_tiles.push(ProcessedTile { fx, fy, rgb });
            dbg!(&tile.values, rgb);
        }
        ProcessedWorld { processed_tiles, tile_size }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Procedural Map Generator")
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();
    let map = Perlin::new().set_seed(rng.gen());
    // let tile_size = num::integer::gcd(WINDOW_WIDTH / MAP_SIZE, WINDOW_HEIGHT / MAP_SIZE) as f32;
    let tile_size = 1.0;
    let world = World::new(WINDOW_WIDTH, WINDOW_HEIGHT, map);
    let processed_world = ProcessedWorld::new(world, tile_size, app.window_rect().left(), app.window_rect().top());
    let mut image_buffer = ImageBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    
    for (_, _, pixel) in image_buffer.enumerate_pixels_mut() {
        let r = ;
        let g = random_range(0, 255);
        let b = random_range(0, 255);
        *pixel = Rgb([r as u8, g as u8, b as u8]);
    }
        
    dbg!(&processed_world);

    Model { processed_world }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for tile in &model.processed_world.processed_tiles {
        draw.rect()
            .w(model.processed_world.tile_size)
            .h(model.processed_world.tile_size)
            .x_y(tile.fx, tile.fy)
            .rgb8(tile.rgb[0], tile.rgb[1], tile.rgb[2]);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, _model: &mut Model, _update: Update) { }

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

#[derive(Debug)]
struct World {
    tiles: Vec<Tile>
}

impl World {
    fn new(width: u32, height: u32, noise: Perlin) -> Self {
        let mut tiles: Vec<Tile> = Vec::with_capacity((width * height) as usize);
        
        for i in 0..width {
            for j in 0..height {
                tiles.push(
                    Tile::new(
                        i, 
                        j, 
                        [
                            // map_range(noise.get([(i as f64 + PERLIN_OFFSET) / PERLIN_SCALE_FACTOR, (j as f64 + PERLIN_OFFSET) / PERLIN_SCALE_FACTOR]).clamp(0.0, 1.0), 0.0, 1.0, 0.0, 255.0),
                            // map_range(noise.get([(i as f64 + PERLIN_OFFSET + 100.0) / PERLIN_SCALE_FACTOR, (j as f64 + PERLIN_OFFSET + 100.0) / PERLIN_SCALE_FACTOR]).clamp(0.0, 1.0), 0.0, 1.0, 0.0, 255.0),
                            // map_range(noise.get([(i as f64 + PERLIN_OFFSET + 200.0) / PERLIN_SCALE_FACTOR, (j as f64 + PERLIN_OFFSET + 200.0) / PERLIN_SCALE_FACTOR]).clamp(0.0, 1.0), 0.0, 1.0, 0.0, 255.0)
                            // map_range(noise.get([(i as f64 + PERLIN_OFFSET), (j as f64 + PERLIN_OFFSET)]).clamp(0.0, 1.0), 0.0, 1.0, 0.0, 255.0),
                            // map_range(noise.get([(i as f64 + PERLIN_OFFSET + 100.0), (j as f64 + PERLIN_OFFSET + 100.0)]).clamp(0.0, 1.0), 0.0, 1.0, 0.0, 255.0),
                            // map_range(noise.get([(i as f64 + PERLIN_OFFSET + 200.0), (j as f64 + PERLIN_OFFSET + 200.0)]).clamp(0.0, 1.0), 0.0, 1.0, 0.0, 255.0)
                            map_range(noise.get([(i as f64 + PERLIN_OFFSET), (j as f64 + PERLIN_OFFSET)]).clamp(0.0, 1.0), 0.0, 1.0, 0.0, 255.0),
                            map_range(noise.get([(i as f64 + PERLIN_OFFSET), (j as f64 + PERLIN_OFFSET)]).clamp(0.0, 1.0), 0.0, 1.0, 0.0, 255.0),
                            map_range(noise.get([(i as f64 + PERLIN_OFFSET), (j as f64 + PERLIN_OFFSET)]).clamp(0.0, 1.0), 0.0, 1.0, 0.0, 255.0)
                        ]
                    )
                )
            }
        }
        World {tiles}
    }
}

#[derive(Debug)]
struct Tile {
    position: Position,
    values: [f64; 3],
}

impl Tile {
    fn new(x: u32, y: u32, values: [f64; 3]) -> Self {
        Tile {position: Position {x, y}, values}
    }
}

#[derive(Debug)]
struct Position { x: u32, y: u32 }

