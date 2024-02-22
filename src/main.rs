use clap::Parser;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use nannou::image::DynamicImage::ImageRgb8;
use lazy_static::lazy_static;
use crate::{
    image::{generate_image, save_image, VisualizationMode}, map::world::{World, WorldParameters}, utils::cli::Args
};

pub mod utils;
pub mod map;
pub mod image;
pub mod noise_sampler;

lazy_static! {
    static ref ARGS: Args = Args::parse();
}

fn main() {
    if ARGS.debug { println!("[MapGen] Running with debug on; logs will be generated"); };
    println!(
        "[MapGen] We are attempting to generate a map in {} x {}.",
        ARGS.x,
        ARGS.y
    );
    nannou::app(model)
        .loop_mode(LoopMode::Wait)
        .run();
}

struct Model {
    _window: window::Id,
    world: World,
    texture: Texture,
    parameters: WorldParameters,
    visual_mode: VisualizationMode,
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .mouse_pressed(new_map)
        .key_pressed(handle_keys)
        .view(view)
        .build()
        .unwrap();
    let parameters = WorldParameters {
        sea_level: 0.0,
        peak_height: 1.0,
        mountain_height: 0.8,
        hills_height: 0.6,
        glacier_temp: -1.0,
        grassland_threshold: 0.5,
        forest_threshold: 0.6,
        swamp_threshold: 0.4,
        tundra_low_t: -0.4,
        tundra_high_t: -0.4,
        altitude_scale: 500.0,
        temperature_scale: 500.0,
        humidity_scale: 500.0,
    };
    let visual_mode = VisualizationMode::Biome;
    let world = World::new(&ARGS, &parameters);
    let texture = Texture::from_image(app, &ImageRgb8(generate_image(&world, &visual_mode)));
    Model { _window, world, texture, parameters, visual_mode }
}

fn new_map(app: &App, model: &mut Model, _key: MouseButton) {
    model.world = World::new(&ARGS, &model.parameters);
    model.texture = Texture::from_image(app, &ImageRgb8(generate_image(&model.world, &model.visual_mode)));
}

fn handle_keys(app: &App, model: &mut Model, key: Key) {
    if matches!(key, Key::Space) {
        match model.visual_mode {
            VisualizationMode::Biome => model.visual_mode = VisualizationMode::Altitude,
            VisualizationMode::Altitude => model.visual_mode = VisualizationMode::Humidity,
            VisualizationMode::Humidity => model.visual_mode = VisualizationMode::Temperature,
            VisualizationMode::Temperature => model.visual_mode = VisualizationMode::Debug,
            VisualizationMode::Debug => model.visual_mode = VisualizationMode::EquatorDistance,
            VisualizationMode::EquatorDistance => model.visual_mode = VisualizationMode::Biome,
            _ => unreachable!()
        };
        println!("[MapGen] Mode switched to {}.", model.visual_mode);
        model.texture = Texture::from_image(app, &ImageRgb8(generate_image(&model.world, &model.visual_mode)));
    };

    if matches!(key, Key::S) {
        save_image(&generate_image(&model.world, &model.visual_mode), &model.world, &model.visual_mode, ARGS.debug);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let rect = app.window_rect();
    draw.texture(&model.texture).w_h(rect.w(), rect.h());
    draw.to_frame(app, &frame).unwrap();
}
