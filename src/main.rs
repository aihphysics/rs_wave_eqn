use clap::{command, Parser};
use ndarray::prelude::*;

use substrate::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Velocity of wave
    #[arg(short = 'v', long = "velocity", default_value_t = 0.1f64)]
    pub velocity: f64,

    /// Amplitude of wave
    #[arg(short = 'a', long = "amplitude", default_value_t = 1.0f64)]
    pub amplitude: f64,

    /// Size of temporal step
    #[arg(short = 't', long = "timestep", default_value_t = 0.1f64)]
    pub timestep: f64,

    /// Record full process.
    #[arg(short, long, default_value_t = false)]
    pub record: bool,

    #[arg(short = 'X', long, default_value_t = 100usize)]
    pub x_dim: usize,

    #[arg(short = 'Y', long, default_value_t = 100usize)]
    pub y_dim: usize,

    #[arg(short = 'T', long, default_value_t = 100usize)]
    pub t_dim: usize,

    #[arg(short, long = "sigma_x", default_value_t = 1.0f64)]
    pub sigma_x: f64,

    #[arg(short, long = "sigma_y", default_value_t = 1.0f64)]
    pub sigma_y: f64,

    #[arg(short='o', long, default_value_t=String::from("./plot.gif") )]
    pub output_file: String,
}

fn main() {
    //let args = Args::parse();

    //let shape: (usize, usize, usize) = (args.t_dim, args.x_dim, args.y_dim);

    //let mut space: Array3<f64> = Array3::from_shape_fn(shape, |(t, x, y)| match t {
    //    0 | 1 => gaussian_elem(x, y),
    //    _ => 0f64,
    //});

    //for idx in 1usize..args.t_dim - 3 {
    //    update(&mut space.slice_mut(s![idx..idx + 3, .., ..]));
    //}

    //if args.record {
    //    record_gif(space)
    //}
}
