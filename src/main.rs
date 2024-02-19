use clap::Parser;
use nannou::prelude::*;
use lazy_static::lazy_static;
// use rayon::prelude::*;
use crate::{
    image::{generate_image, VisualizationMode},
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
    static ref WORLDS: Vec<World> = generate_worlds(&ARGS);
}

fn main() {

    //if args.debug { println!("[MapGen] Running with debug on; logs will be generated"); };
    //println!(
    //    "[MapGen] We are attempting to generate {} map(s) in {} x {}.",
    //    args.variations,
    //    args.x,
    //    args.y
    //);

    // let worlds = generate_worlds(&args);

    //worlds.par_iter().for_each(|world| {
    //    // save_image(world, VisualizationMode::Debug, args.file.clone(), args.debug);
    //    save_image(world, VisualizationMode::Altitude, args.file.clone(), args.debug);
    //    // save_image(world, VisualizationMode::Temperature, args.file.clone(), args.debug);
    //    // save_image(world, VisualizationMode::Humidity, args.file.clone(), args.debug);
    //    save_image(world, VisualizationMode::Biome, args.file.clone(), args.debug);
    //});

    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    texture: Texture,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let image = generate_image(&WORLDS[0], VisualizationMode::Biome);
    let converted = nannou::image::DynamicImage::ImageRgb8(image);
    let texture = Texture::from_image(app, &converted);
    Model { _window, texture }
}

fn update(_app: &App, _model: &mut Model, _update: Update) { }

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.texture(&model.texture);
    draw.to_frame(app, &frame).unwrap();
}
