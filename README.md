# Linear Regression Model in Rust
## Project Overview
This project implements a simple Linear Regression model using the Burn framework in Rust. The model is trained on synthetic data and visualized using the textplots crate.

# Setup Instructions
### Install Rust and Cargo
Ensure you have Rust and Cargo installed. You can install them using:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### Clone the Repository
git clone cd linear_regression_model

### Install Dependencies
The dependencies required for this project are already listed in Cargo.toml. You can install them by running:

cargo build

### Run the Project
cargo run

## Project Structure
linear_regression_model/ │── src/ │ ├── main.rs # Implementation of the Linear Regression model │── Cargo.toml # Project dependencies and configuration │── README.md # Documentation

## Implementation Details
### Generating Synthetic Data:

A dataset of (x, y) pairs is created where y = 3.2x + 0.8 + noise.

Random noise is added to simulate real-world conditions.

## Defining the Model:
A simple linear model is implemented using Burn’s Linear module.

A forward pass is defined for predictions.

### Training the Model:
The synthetic dataset is used for training.

The Mean Squared Error (MSE) loss function is used.

### Evaluating the Model:
The model is tested on unseen data.

Results are visualized using textplots.

## Challenges Faced
Understanding how to integrate Burn with Rust.

Handling random number generation correctly.

Formatting output for better visualization.

## Resources Used

Burn Documentation

Rust Official Documentation

Various online tutorials and AI-generated suggestions

## Learning Reflection
This project provided valuable insights into using Rust for machine learning applications. I improved my understanding of:

Structuring Rust projects with Cargo.

Implementing linear regression using Burn.

Handling visualization in Rust.

## AI and External Assistance

I received assistance from AI tools and online documentation to understand Rust’s syntax and the Burn framework. The AI suggestions helped structure the project efficiently, but the core implementation and debugging were done manually.

## If Challenges Were Unresolved

If any part of the project was not working as expected, I revisited the official documentation and tried alternative approaches. This helped in reinforcing problem-solving skills and debugging techniques.

## Submission Details
Code is pushed to GitHub.

Cargo.toml remains unchanged as per assignment requirements.

This README.md serves as the project documentation.
