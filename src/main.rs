use ndarray::prelude::*;
use ndarray::*;
use plotters::prelude::*;
use clap::{Parser,command};

/// 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Velocity of wave
    #[arg(short = 'v', long, default_value_t = 0.1)]
    velocity: f64,

    /// Size of temporal step
    #[arg(short='t', long, default_value_t = 0.1)]
    timestep: f64,
    
    #[arg(short, long, default_value_t = false )]
    record: bool,

    #[arg(short='X', long, default_value_t = 100 )]
    x_dim: usize,
    
    #[arg(short='Y', long, default_value_t = 100 )]
    y_dim: usize,
  
    #[arg(short='T', long, default_value_t = 100 )]
    t_dim: usize,

}


const C: f64 = 0.1;
const T: f64 = 0.1;
const H_INV: f64 = 10.0;
const PAR: f64 = C * C * T * T * H_INV * H_INV;
const OUT_FILE_NAME: &str = "./plot.gif";

// gaussian parameters
const A: f64 = 1.0;
const SIGMA_X: f64 = 3.0;
const SIGMA_Y: f64 = 3.0;


fn initial_gaussian( mat: &mut ArrayViewMut3<f64> ){

    let ( _, x_dim, y_dim ) = mat.dim();

    for i in 1..x_dim - 1 {
        for j in 1..y_dim - 1 {
            mat[[0, i, j]] = A
                * (-0.5 * f64::powf((i as f64 - ( x_dim as f64 / 2.0 )) / SIGMA_X, 2.0)
                    - 0.5 * f64::powf((j as f64 - ( y_dim as f64 / 2.0 )) / SIGMA_Y, 2.0))
                .exp();
            mat[[1, i, j]] = mat[[0, i, j]];
            mat[[2, i, j]] = mat[[0, i, j]];
        }
    }
}

fn gaussian_elem( x:usize, y:usize, mu_x:f64, mu_y:f64 ) -> f64 {
  A * (-0.5 * f64::powf((x as f64 - ( mu_x / 2.0 )) / SIGMA_X, 2.0)
    - 0.5 * f64::powf((y as f64 - ( mu_y / 2.0 )) / SIGMA_Y, 2.0))
}

//pub struct WaveSolver {
//  t_dim:usize, 
//  x_dim:usize, 
//  y_dim:usize,
//  spacetime: Array3<f64>
//
//}
//
//impl WaveSolver {
//
//  pub fn new( t_dim:usize, x_dim:usize, y_dim:usize ){
//    let spacetime = Array3::<f64>::zeros( ( t_dim, x_dim, y_dim ) );
//  }
//
//}


//fn update( mat: &mut ArrayViewMut3<f64> ){
//  
//    let (_, x_dim, y_dim ) = mat.dim();
//
//    for i in 1..x_dim - 1 {
//        for j in 1..y_dim - 1 {
//            mat[[2, i, j]] = ATTENUATION
//                * PAR
//                * (mat[[1, i + 1, j]]
//                    + mat[[1, i - 1, j]]
//                    + mat[[1, i, j + 1]]
//                    + mat[[1, i, j - 1]]
//                    - 4.0 * mat[[1, i, j]])
//                - mat[[0, i, j]]
//                + 2.0 * mat[[1, i, j]];
//        }
//    }
//}

fn record_gif(space: Array3<f64>) {
  
    let ( t_dim, x_dim, y_dim ) = space.dim();

    let root = BitMapBackend::gif(OUT_FILE_NAME, (500, 500), 50)
        .unwrap()
        .into_drawing_area();
    
    for t in 1..t_dim- 3 {

        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Explict 2D Wave Equation", ("Inconsolata", 20))
            .build_cartesian_3d(
                (0.0..(x_dim as f64)).step(1.0),
                -1.0..1.0,
                (0.0..(y_dim as f64)).step(1.0),
            )
            .unwrap();

        chart.with_projection(|mut p_mat| {
            p_mat.pitch = 1.57 * (0.5 * ((t as f64) / (100.0)).sin() + 0.5).abs();
            p_mat.scale = 0.7;
            p_mat.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(5)
            .draw()
            .unwrap();

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (0..=x_dim - 1).map(|x| x as f64),
                    (0..=y_dim - 1).map(|y| y as f64),
                    |x: f64, y: f64| space[[t, x as usize, y as usize]],
                )
                .style_func(&|&v| (ViridisRGB::get_color((v + 0.5) / 1.5)).into()),
            )
            .unwrap(); //.label( "Sinusoidal distortion" );

        root.present().unwrap();
    }
}

fn main() {

    let args = Args::parse();
    println!( "timestep {:?}", args.timestep );
    println!( "velocity {:?}", args.velocity );



    let mut space = Array3::<f64>::zeros( ( args.t_dim, args.x_dim, args.y_dim ) );
    initial_gaussian(&mut space.slice_mut(s![..3, .., ..]));

    let shape:(usize,usize,usize) = ( args.t_dim, args.x_dim, args.y_dim );
    let space: Array3<f64> = Array3::from_shape_fn( shape, | ( t, x, y ) |
      match t { 
        0 | 1 => gaussian_elem( x, y, SIGMA_X, SIGMA_Y ),
        _ => 0f64
      }
    );



    //for idx in 1usize..args.t_dim - 3 {
    //  update(&mut space.slice_mut(s![idx..idx + 3, .., ..]));
    //}

    if args.record { record_gif(space) }
}
