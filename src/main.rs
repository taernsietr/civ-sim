use clap::Parser;
use nannou::prelude::*;
use lazy_static::lazy_static;
// use rayon::prelude::*;
use crate::{
    image::{generate_image, create_coast, VisualizationMode},
    utils::{cli::Args, helpers::generate_worlds},
    map::world::World
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
        "[MapGen] We are attempting to generate {} map(s) in {} x {}.",
        ARGS.variations,
        ARGS.x,
        ARGS.y
    );

    // let worlds = generate_worlds(&args);

    //worlds.par_iter().for_each(|world| {
    //    // save_image(world, VisualizationMode::Debug, args.file.clone(), args.debug);
    //    save_image(world, VisualizationMode::Altitude, args.file.clone(), args.debug);
    //    // save_image(world, VisualizationMode::Temperature, args.file.clone(), args.debug);
    //    // save_image(world, VisualizationMode::Humidity, args.file.clone(), args.debug);
    //    save_image(world, VisualizationMode::Biome, args.file.clone(), args.debug);
    //});

    nannou::app(model).run();
}

struct Model {
    _window: window::Id,
    worlds: Vec<World>,
    texture: Texture,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().mouse_pressed(mouse_pressed).view(view).build().unwrap();
    let worlds: Vec<World> = generate_worlds(&ARGS);
    let mut image = generate_image(&worlds[0], VisualizationMode::Biome);
    create_coast(&worlds[0], &mut image);
    let converted = nannou::image::DynamicImage::ImageRgb8(image);
    let texture = Texture::from_image(app, &converted);
    Model { _window, worlds, texture }
}

fn mouse_pressed(app: &App, model: &mut Model, _key: MouseButton) {
    model.worlds = generate_worlds(&ARGS);
    let mut image = generate_image(&model.worlds[0], VisualizationMode::Biome);
    create_coast(&model.worlds[0], &mut image);
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
