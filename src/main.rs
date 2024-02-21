use clap::Parser;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use nannou::image::DynamicImage::ImageRgb8;
use lazy_static::lazy_static;
use crate::{
    image::{generate_image, VisualizationMode},
    utils::cli::Args,
    map::world::{World, WorldParameters}
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
        .key_pressed(switch_mode)
        .view(view)
        .build()
        .unwrap();
    let parameters = WorldParameters {
        sea_level: 0.0,
        swamp_humidity: 0.5,
        desert_humidity: -0.5,
        hill_altitude: 0.5,
        mountain_altitude: 0.75,
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

fn switch_mode(app: &App, model: &mut Model, key: Key) {
    if matches!(key, Key::Space) {
        match model.visual_mode {
            VisualizationMode::Biome => model.visual_mode = VisualizationMode::Altitude,
            VisualizationMode::Altitude => model.visual_mode = VisualizationMode::Humidity,
            VisualizationMode::Humidity => model.visual_mode = VisualizationMode::Temperature,
            VisualizationMode::Temperature => model.visual_mode = VisualizationMode::Debug,
            VisualizationMode::Debug => model.visual_mode = VisualizationMode::Biome,
            _ => unreachable!()
        };
        println!("[MapGen] Mode switched to {}.", model.visual_mode);
    };
    model.texture = Texture::from_image(app, &ImageRgb8(generate_image(&model.world, &model.visual_mode)));
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let rect = app.window_rect();
    draw.texture(&model.texture).w_h(rect.w(), rect.h());
    draw.to_frame(app, &frame).unwrap();
}
