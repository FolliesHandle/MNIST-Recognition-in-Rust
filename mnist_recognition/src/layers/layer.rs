use rand::{distributions::Uniform, prelude::Distribution};

use ndarray::{prelude::Array2, Axis, Zip};

pub trait ActivationLayer {
    fn activate(&mut self);
    fn deactivate(&mut self, previous_layer: &Layer);
}
pub struct Layer {
    pub preactivation: Array2<f64>,
    pub layer: Array2<f64>,
    pub d_activation: Array2<f64>,
    pub weights: Array2<f64>,
    pub d_weights: Array2<f64>,
    pub biases: Array2<f64>,
    pub d_biases: Array2<f64>,
    alpha: f64,
    samples: usize,
}

impl Layer {
    pub fn new_layer(input: usize, nodes: usize, samples: usize, alpha: f64) -> Layer {
        let preactivation = Array2::zeros((nodes, samples));
        let layer = Array2::zeros((nodes, samples));
        let d_activation = Array2::zeros((nodes, samples));

        let rand_range = Uniform::from(-0.5f64..=0.5f64);
        let mut rng = rand::thread_rng();

        let weights = Array2::from_shape_simple_fn((nodes, input), || rand_range.sample(&mut rng));
        let d_weights =
            Array2::from_shape_simple_fn((nodes, input), || rand_range.sample(&mut rng));
        let biases = Array2::from_shape_simple_fn((nodes, 1), || rand_range.sample(&mut rng));
        let d_biases = Array2::from_shape_simple_fn((nodes, 1), || rand_range.sample(&mut rng));

        Layer {
            preactivation: preactivation,
            layer: layer,
            d_activation: d_activation,
            weights: weights,
            d_weights: d_weights,
            biases: biases,
            d_biases: d_biases,
            alpha: alpha,
            samples: samples,
        }
    }

    fn forward_prop(&mut self, previous_layer: &Layer, activation: &mut dyn FnMut(&Array2<f64>)) {
        self.preactivation = self.weights.dot(&previous_layer.layer) + &self.biases;
        activation(&self.preactivation);
    }

    fn back_prop(&mut self, previous_layer: &Layer, deactivation: &mut dyn FnMut(&Layer)) {
        deactivation(previous_layer);
        self.d_biases = self
            .d_activation
            .sum_axis(Axis(1))
            .map(|x| x * 1. / (self.samples as f64))
            .insert_axis(Axis(1));
    }

    fn update_params(&mut self) {
        Zip::from(&mut self.weights)
            .and(&self.d_weights)
            .apply(|a, b| *a -= self.alpha * b);
        Zip::from(&mut self.biases)
            .and(&self.d_biases)
            .apply(|a, b| *a -= self.alpha * b);
    }
}