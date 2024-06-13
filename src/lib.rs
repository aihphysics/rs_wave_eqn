use ndarray::prelude::*;
use plotters::prelude::*;
use serde::{Serialize, Deserialize};

const H_INV: f64 = 10.0;

#[derive(Serialize, Deserialize)]
enum WaveType {
    Gaussian,
    Sin,
    Cos,
}

#[derive(Serialize, Deserialize)]
pub struct Wave {
    amplitude: f64,
    mu_x: f64,
    mu_y: f64,
    sigma_x: f64,
    sigma_y: f64,
    function: WaveType,
}

impl Wave {
    pub fn elem(&self, x: usize, y: usize) -> f64 {
        match self.function {
            WaveType::Gaussian => {
                self.amplitude
                    * (-0.5
                        * f64::powf((x as f64 - (self.mu_x / 2.0)) / self.sigma_x, 2.0)
                        * -0.5
                        * f64::powf((y as f64 - (self.mu_y / 2.0)) / self.sigma_y, 2.0))
            }
            WaveType::Sin => (x as f64).sin() * (y as f64).sin(),
            WaveType::Cos => (x as f64).cos() * (y as f64).cos(),
            _ => 0f64,
        }
    }
}

pub struct Medium {
    attenuation: f64,
    parameter: f64,
    shape: (usize, usize, usize),
    medium: Array3<f64>,
}

impl Medium {
    pub fn new(
        attenuation: f64,
        velocity: f64,
        timestep: f64,
        x_dim: usize,
        y_dim: usize,
        t_dim: usize,
    ) -> Self {
        Self {
            attenuation,
            parameter: velocity * velocity * timestep * timestep * H_INV * H_INV,
            shape: (t_dim, x_dim, y_dim),
            medium: Array3::zeros((t_dim, x_dim, y_dim)),
        }
    }

    pub fn inital_add(&mut self, wave: Wave) {
        let rhs = Array3::from_shape_fn(self.shape, |(t, x, y)| match t {
            0 | 1 => wave.elem(x, y),
            _ => 0f64,
        });
        //self.medium = self.medium + rhs;
        //self.medium = &self.medium + rhs;
        self.medium.scaled_add(1.0, &rhs);
    }

    pub fn new_square(
        attenuation: f64,
        velocity: f64,
        timestep: f64,
        x_dim: usize,
        t_dim: usize,
    ) -> Self {
        Self {
            attenuation,
            parameter: velocity * velocity * timestep * timestep * H_INV * H_INV,
            shape: (t_dim, x_dim, x_dim),
            medium: Array3::zeros((t_dim, x_dim, x_dim)),
        }
    }

    pub fn process(&mut self) {
        for idx in 1usize..self.shape.0 - 3 {
            Medium::update(self, idx);
        }
    }

    fn update(&mut self, step: usize) {
        let mut slice: ArrayViewMut3<f64> = self.medium.slice_mut(s![step..step + 3, .., ..]);
        let (_, x_dim, y_dim) = slice.dim();

        for i in 1..x_dim - 1 {
            for j in 1..y_dim - 1 {
                slice[[2, i, j]] = self.attenuation
                    * self.parameter
                    * (slice[[1, i + 1, j]]
                        + slice[[1, i - 1, j]]
                        + slice[[1, i, j + 1]]
                        + slice[[1, i, j - 1]]
                        - 4.0 * slice[[1, i, j]])
                    - slice[[0, i, j]]
                    + 2.0 * slice[[1, i, j]];
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
