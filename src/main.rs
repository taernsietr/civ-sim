use clap::Parser;
use lazy_static::lazy_static;
use nannou::{
    prelude::*,
    wgpu::Texture
};
use crate::{
    image::VisualizationMode, 
    map::world::{WorldBuilder, World, WorldParameters},
    utils::{cli::Args, helpers::load_parameters}
};

pub mod utils;
pub mod map;
pub mod image;
pub mod noise_sampler;

lazy_static! {
    static ref ARGS: Args = Args::parse();
}

struct Model {
    _window: window::Id,
    world: World,
    texture: Texture,
    parameters: WorldParameters,
    visual_mode: VisualizationMode,
}

fn main() {
    if ARGS.debug { println!("[MapGen] Running with debug on; logs will be generated"); };
    println!(
        "[MapGen] We are attempting to generate a map in {} x {}.",
        ARGS.x,
        ARGS.y
    );
    nannou::app(model).loop_mode(LoopMode::Wait).run();
}

fn model(app: &App) -> Model {
    let parameters = load_parameters();
    let _window = app.new_window()
        .key_pressed(handle_keys)
        .view(view)
        .build()
        .unwrap();
    let visual_mode = VisualizationMode::Biome;
    let world = WorldBuilder::new(&ARGS, &parameters).build();
    let texture = Texture::from_image(app, &world.generate_image(&visual_mode));
    Model { _window, world, texture, parameters, visual_mode }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let rect = app.window_rect();
    draw.texture(&model.texture).w_h(rect.w(), rect.h());
    draw.to_frame(app, &frame).unwrap();
}

fn handle_keys(app: &App, model: &mut Model, key: Key) {
    // SPACE: switch visualization mode
    if matches!(key, Key::Space) {
        match model.visual_mode {
            VisualizationMode::Biome => model.visual_mode = VisualizationMode::Altitude,
            VisualizationMode::Altitude => model.visual_mode = VisualizationMode::Rainfall,
            VisualizationMode::Rainfall => model.visual_mode = VisualizationMode::Temperature,
            VisualizationMode::Temperature => model.visual_mode = VisualizationMode::EquatorDistance,
            VisualizationMode::EquatorDistance => model.visual_mode = VisualizationMode::Biome,
            _ => unreachable!()
        };
        println!("[MapGen] Mode switched to {}.", model.visual_mode);
        model.texture = Texture::from_image(app, &model.world.generate_image(&model.visual_mode));
    };

    // S: save current map
    if matches!(key, Key::S) {
        model.world.save_image(&model.visual_mode, ARGS.debug);
    }

    // N: generate new map
    if matches!(key, Key::N) {
        model.world = WorldBuilder::new(&ARGS, &model.parameters).build();
        model.texture = Texture::from_image(app, &model.world.generate_image(&model.visual_mode));
    }
}
