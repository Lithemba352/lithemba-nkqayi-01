use burn::tensor::Tensor;
use burn::module::Module;
use burn::nn::Linear;
use burn::tensor::backend::Backend;
use rand::Rng;
use textplots::{Chart, Plot, Shape};

// Defining the linear model
#[derive(Module, Debug)]
struct LinearRegression<B: Backend> {
    linear: Linear<B>,
}

impl<B: Backend> LinearRegression<B> {
    pub fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 2> {
        self.linear.forward(x)
    }
}

// Generating synthetic data
fn generate_data(n: usize) -> (Vec<f32>, Vec<f32>) {
    let mut rng = rand::thread_rng();
    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();

    for _ in 0..n {
        let x = rng.gen_range(-12.0..12.0); // Slightly adjusted range
        let y = 2.5 * x + 0.8 + rng.gen_range(-1.2..1.2); // Adjusted coefficients and noise
        x_vals.push(x);
        y_vals.push(y);
    }

    (x_vals, y_vals)
}

fn main() {
    let (x_vals, y_vals) = generate_data(120); // Increased sample size

    // Print sample data points
    for i in 0..12 { // Printing 12 instead of 10
        println!("x: {:.3}, y: {:.3}", x_vals[i], y_vals[i]); // More decimal places
    }

    // Plot results
    Chart::new(190, 65, -12.0, 12.0) // Adjusted chart size
        .lineplot(&Shape::Points(&x_vals.iter().zip(y_vals.iter()).map(|(&x, &y)| (x, y)).collect::<Vec<_>>() ))
        .display();
}
