use clap::Parser;
use nannou::prelude::*;
use lazy_static::lazy_static;
use crate::{
    image::{generate_image, shape_continent, create_coast, VisualizationMode},
    utils::cli::Args,
    map::world::{World, WorldParameters}
};
use nannou::wgpu::Texture;

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
    nannou::app(model).run();
}

struct Model {
    _window: window::Id,
    world: World,
    texture: Texture,
    parameters: WorldParameters,
}

fn model(app: &App) -> Model {
    let parameters = WorldParameters {
        sea_level: 0.0,
        swamp_humidity: 0.5,
        desert_humidity: -0.5,
        hill_altitude: 0.5,
        mountain_altitude: 0.75
    };
    let _window = app.new_window().mouse_pressed(mouse_pressed).view(view).build().unwrap();
    let mut world = World::new(&ARGS, &parameters);
    shape_continent(&mut world);
    let mut image = generate_image(&world, VisualizationMode::Biome);
    create_coast(&world, &mut image);
    let converted = nannou::image::DynamicImage::ImageRgb8(image);
    let texture = Texture::from_image(app, &converted);
    Model { _window, world, texture, parameters }
}

fn mouse_pressed(app: &App, model: &mut Model, _key: MouseButton) {
    model.world = World::new(&ARGS, &model.parameters);
    shape_continent(&mut model.world);
    let mut image = generate_image(&model.world, VisualizationMode::Biome);
    create_coast(&model.world, &mut image);
    let converted = nannou::image::DynamicImage::ImageRgb8(image);
    model.texture = Texture::from_image(app, &converted);
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let rect = app.window_rect();
    draw.texture(&model.texture).w_h(rect.w(), rect.h());
    draw.to_frame(app, &frame).unwrap();
}
