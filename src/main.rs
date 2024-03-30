use plotters::prelude::*;
use ndarray::prelude::*;
use ndarray::*;

const X_DIM: usize = 27;
const Y_DIM: usize = 27;
const T_DIM: usize = 1000;

const C: f64 = 0.1;
const T: f64 = 0.1;
const H_INV: f64 = 10.0;
const ATTENUATION: f64 = 0.5;
const PAR: f64 = C*C*T*T*H_INV*H_INV;
const OUT_FILE_NAME: &str = "./plot.gif";


fn update( t_idx: usize, mat: &mut ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 3]>> ){

  mat[[1, X_DIM/2, Y_DIM/2]]  = 0.5*( ( t_idx as f64 * T )/2.0 ).sin();

  for i in 1..X_DIM-1{
    for j in 1..Y_DIM-1{
      mat[[2, i, j]] = ATTENUATION * PAR * ( mat[ [1, i+1,j   ] ] + mat[ [ 1, i-1, j   ] ] 
                           +   mat[ [1, i,  j+1 ] ] + mat[ [ 1, i,   j-1 ] ] 
                           - 4.0*mat[ [ 1, i, j ] ] ) 
                        - mat[ [ 0, i, j  ] ] + 2.0*mat[ [ 1, i, j ] ];
    }
  }
}


fn main() {

  let mut space = Array3::< f64 >::zeros( ( T_DIM, X_DIM, Y_DIM ) );
  for idx in 1usize..T_DIM-3 {
    update( idx-1, &mut space.slice_mut( s![ idx..idx+3, .., .. ] ) );
  }

  let root = BitMapBackend::gif(OUT_FILE_NAME, (500, 500), 50).unwrap().into_drawing_area();
  
  for t in 1..T_DIM-3 {

    root.fill( &WHITE ).unwrap();

    let mut chart = ChartBuilder::on(&root)
      .caption( "Explict 2D Wave Equation", ("Inconsolata", 20))
      .build_cartesian_3d( (0.0..(X_DIM as f64)).step(1.0), -1.0..1.0, (0.0..(X_DIM as f64)).step(1.0) ).unwrap();

    chart.with_projection(|mut p_mat| {
      p_mat.pitch = 1.57*( 0.5*((t as f64)/(100.0)).sin() + 0.5 ).abs();
        p_mat.scale = 0.7;
        p_mat.into_matrix()
    });


    chart
      .configure_axes()
      .light_grid_style(BLACK.mix(0.15))
      .max_light_lines(5)
      .draw().unwrap();


    chart.draw_series(
      SurfaceSeries::xoz(
        (0..=X_DIM-1).map(|x| x as f64 ),
        (0..=Y_DIM-1).map(|y| y as f64 ),
        | x:f64, y:f64| space[ [t, x as usize, y as usize ] ]
      ).style_func(&|&v| (ViridisRGB::get_color( (v+0.5)/1.5 )).into()),
    ).unwrap().label( "Sinusoidal distortion" );
  

    root.present().unwrap();
  }




}
