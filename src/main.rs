//use plotters::prelude::*;
//use grid::*;
//use lending_iterator::prelude::*;
use ndarray::prelude::*;
use ndarray::*;


const X_DIM: usize = 200;
const Y_DIM: usize = 200;
const T_DIM: usize = 500;

const C: f64 = 0.1;
const T: f64 = 0.1;
const H_INV: f64 = 10.0;
const PAR:f64 = C*C*T*T*H_INV*H_INV;

//fn add_disturbance( t_idx: usize, mat: &mut Grid<f64> ){
//  mat[( X_DIM/2, Y_DIM/2)] = ( t_idx as f64 * T ).sin();
//}

//fn update( window: &mut [( usize, &mut Grid<f64> ) ] ){
//  let now = &mut window[1].1;
//  let idx= window[1].0;
//  add_disturbance( idx, now );
//  for i in 1..X_DIM-1{
//    for j in 1..Y_DIM-1{
//      window[2].1[(i,j)] = PAR * ( window[1].1[(i+1,j)] + window[1].1[(i-1,j)] 
//                           + window[1].1[(i,j+1)] + window[1].1[(i,j-1)] 
//                           - 4.0*window[1].1[(i,j)] ) 
//                  - window[0].1[(i,j)] + 2.0*window[1].1[(i,j)];
//    }
//  }
//
//}

fn update( mat: &mut ArrayBase<ViewRepr<&mut f64>, Dim<[usize; 3]>> ){


  for i in 1..X_DIM-1{
    for j in 1..Y_DIM-1{
      mat[[2, i, j]] = PAR * ( mat[ [1, i+1,j   ] ] + mat[ [ 1, i-1, j   ] ] 
                           +   mat[ [1, i,  j+1 ] ] + mat[ [ 1, i,   j-1 ] ] 
                           - 4.0*mat[ [ 1, i, j ] ] ) 
                  - mat[ [ 0, i, j ] ] + 2.0*mat[ [ 1, i, j ] ];
    }
  }

}

fn main() {

  // create our grid and its initial state, empty
  //let mut space: Vec<Grid<f64>> = vec![Grid::new(X_DIM,Y_DIM); T_DIM];
  //let mut thing: Vec<(usize,&mut Grid<f64>)> = space.iter_mut().enumerate().collect();
  //thing.windows_mut::<3>().for_each( |window| update( window ) ); 
  
  let mut space = Array3::< f64 >::zeros( ( T_DIM, X_DIM, Y_DIM ) );
  for idx in 1..T_DIM-3 {
    update( &mut space.slice_mut( s![ idx..idx+3, .., .. ] ) );
  }

}
