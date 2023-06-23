use ndarray::{Array, Array2, Axis};
use ndarray_rand::rand::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_stats::QuantileExt;

// Define the sigmoid activation function
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

// Define the derivative of the sigmoid function
fn sigmoid_derivative(x: f64) -> f64 {
    let sig = sigmoid(x);
    sig * (1.0 - sig)
}

// Define the neural network struct
struct NeuralNetwork {
    weights_hidden: Array2<f64>,
    bias_hidden: Array2<f64>,
    weights_output: Array2<f64>,
    bias_output: Array2<f64>,
}

impl NeuralNetwork {
    // Initialize the neural network with random weights and biases
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let weights_hidden = Array::random((input_size, hidden_size), Uniform::new(-1.0, 1.0));
        let bias_hidden = Array::random(hidden_size, Uniform::new(-1.0, 1.0));
        let weights_output = Array::random((hidden_size, output_size), Uniform::new(-1.0, 1.0));
        let bias_output = Array::random(output_size, Uniform::new(-1.0, 1.0));

        NeuralNetwork {
            weights_hidden,
            bias_hidden,
            weights_output,
            bias_output,
        }
    }

    // Forward propagation through the network
    fn forward(&self, input: &Array2<f64>) -> Array2<f64> {
        let hidden = sigmoid(input.dot(&self.weights_hidden) + &self.bias_hidden);
        sigmoid(hidden.dot(&self.weights_output) + &self.bias_output)
    }

    // Backpropagation to update weights and biases
    fn train(&mut self, input: &Array2<f64>, target: &Array2<f64>, learning_rate: f64) {
        // Forward propagation
        let hidden = sigmoid(input.dot(&self.weights_hidden) + &self.bias_hidden);
        let output = sigmoid(hidden.dot(&self.weights_output) + &self.bias_output);

        // Backward propagation
        let output_error = output - target;
        let output_delta = output_error * output.mapv(sigmoid_derivative);
        let hidden_error = output_delta.dot(&self.weights_output.t());
        let hidden_delta = hidden_error * hidden.mapv(sigmoid_derivative);

        // Update weights and biases
        self.weights_output -= learning_rate * hidden.t().dot(&output_delta);
        self.bias_output -= learning_rate * output_delta.sum_axis(Axis(0));
        self.weights_hidden -= learning_rate * input.t().dot(&hidden_delta);
        self.bias_hidden -= learning_rate * hidden_delta.sum_axis(Axis(0));
    }
}

fn main() {
    // Generate random input data and targets
    let input_data = Array::random((1000, 2), Uniform::new(-1.0, 1.0));
    let targets = (input_data.sum_axis(Axis(1)) > 0.0).mapv(|x| if x { 1.0 } else { 0.0 }).insert_axis(Axis(1));

    // Create a neural network with 2 input neurons, 4 hidden neurons, and 1 output neuron
    let mut neural_network = NeuralNetwork::new(2, 4, 1);

    // Train the network
    let learning_rate = 0.1;
    let epochs = 1000;
    for _ in 0..epochs {
        neural_network.train(&input_data, &targets, learning_rate);
    }

    // Test the trained network
    let test_input = array![[0.5, 0.5], [-0.5, -0.5], [0.5, -0.5], [-0.5, 0.5]];
    let predictions = neural_network.forward(&test_input);
    for (input, prediction) in test_input.outer_iter().zip(predictions.outer_iter()) {
        println!("Input: {:?} => Prediction: {:?}", input, prediction);
    }
}
