use std::{path::PathBuf, str::FromStr};
use clap::Parser;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use nannou::image::DynamicImage::ImageRgba8;
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
    rivers: Vec<usize>,
    parameters: WorldParameters,
    visual_mode: VisualizationMode,
}

fn model(app: &App) -> Model {
    let parameters = {
        let file = PathBuf::from_str("/home/tsrodr/Run/civ-sim/src/parameters.json").expect("[MapGen] Failed to load json file.");
        let data = std::fs::read_to_string(file).expect("[MapGen] Failed to load parameters.");
        serde_json::from_str::<WorldParameters>(&data).expect("[MapGen] Failed to parse parameters.")
    };
    let _window = app.new_window()
        .key_pressed(handle_keys)
        .view(view)
        .build()
        .unwrap();
    let visual_mode = VisualizationMode::Biome;
    let world = World::new(&ARGS, &parameters);
    //let rivers = world.generate_rivers(&parameters);
    let rivers = vec!();
    let texture = Texture::from_image(app, &ImageRgba8(generate_image(&world, &rivers, &visual_mode)));
    Model { _window, rivers, world, texture, parameters, visual_mode }
}

fn handle_keys(app: &App, model: &mut Model, key: Key) {
    // SPACE: switch visualization mode
    if matches!(key, Key::Space) {
        match model.visual_mode {
            VisualizationMode::Biome => model.visual_mode = VisualizationMode::Altitude,
            VisualizationMode::Altitude => model.visual_mode = VisualizationMode::Rainfall,
            VisualizationMode::Rainfall => model.visual_mode = VisualizationMode::Temperature,
            VisualizationMode::Temperature => model.visual_mode = VisualizationMode::Debug,
            VisualizationMode::Debug => model.visual_mode = VisualizationMode::EquatorDistance,
            VisualizationMode::EquatorDistance => model.visual_mode = VisualizationMode::Biome,
            _ => unreachable!()
        };
        println!("[MapGen] Mode switched to {}.", model.visual_mode);
        model.texture = Texture::from_image(app, &ImageRgba8(generate_image(&model.world, &model.rivers, &model.visual_mode)));
    };

    // S: save current map
    if matches!(key, Key::S) {
        save_image(&generate_image(&model.world, &model.rivers, &model.visual_mode), &model.world, &model.visual_mode, ARGS.debug);
    }

    // N: generate new map
    if matches!(key, Key::N) {
        model.world = World::new(&ARGS, &model.parameters);
        //model.rivers = model.world.generate_rivers(&model.parameters);
        model.texture = Texture::from_image(app, &ImageRgba8(generate_image(&model.world, &model.rivers, &model.visual_mode)));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let rect = app.window_rect();
    draw.texture(&model.texture).w_h(rect.w(), rect.h());
    draw.to_frame(app, &frame).unwrap();
}
