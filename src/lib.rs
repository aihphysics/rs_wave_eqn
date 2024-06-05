use ndarray::prelude::*;
use plotters::prelude::*;

const H_INV: f64 = 10.0;

pub struct Wave {
    amplitude: f64,
    mu_x: f64,
    mu_y: f64,
    sigma_x: f64,
    sigma_y: f64,
}

impl Wave {
    //pub fn new_from( &args: Args ) -> Self {
    //  Wave{ amplitude=0,
    //        0,0,0,0 }
    //}

    pub fn gaussian_elem(&self, x: usize, y: usize) -> f64 {
        self.amplitude
            * (-0.5 * f64::powf((x as f64 - (self.mu_x / 2.0)) / self.sigma_x, 2.0)
                - 0.5 * f64::powf((y as f64 - (self.mu_y / 2.0)) / self.sigma_y, 2.0))
    }
}

//velocity: f64,
//timestep: f64
//hinv: f64

pub struct Medium {
    attenuation: f64,
    parameter: f64,
    shape: (usize, usize, usize),
    medium: Array3<f64>,
}

impl Medium {
    pub fn new_cube( attenuation: f64, parameter: f64, dim: usize) -> Self{
      Self { 
        attenuation, 
        parameter,
        shape: ( dim, dim, dim ),
        medium: Array3::zeros( ( dim, dim, dim ) ),
      }
    }
    pub fn new( attenuation: f64, parameter: f64, xy_dim: usize, t_dim: usize ) -> Self{
      Self { 
        attenuation, 
        parameter,
        shape: ( xy_dim, xy_dim, t_dim ),
        medium: Array3::zeros( ( xy_dim, xy_dim, t_dim ) ),
      }
    }
    pub fn update(&self, mat: &mut ArrayViewMut3<f64>) {
        let (_, x_dim, y_dim) = mat.dim();

        for i in 1..x_dim - 1 {
            for j in 1..y_dim - 1 {
                mat[[2, i, j]] = self.attenuation
                    * self.parameter
                    * (mat[[1, i + 1, j]]
                        + mat[[1, i - 1, j]]
                        + mat[[1, i, j + 1]]
                        + mat[[1, i, j - 1]]
                        - 4.0 * mat[[1, i, j]])
                    - mat[[0, i, j]]
                    + 2.0 * mat[[1, i, j]];
            }
        }
    }
}


pub fn record_gif(space: Array3<f64>) {
    let (t_dim, x_dim, y_dim) = space.dim();

    let root = BitMapBackend::gif("./plot.gif", (500, 500), 50)
        .unwrap()
        .into_drawing_area();

    for t in 1..t_dim - 3 {
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
