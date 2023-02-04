//! Slide puzzle frontend and solvers.
//!
pub mod board;
pub mod expander;
pub mod settings;
pub mod slide_puzzle;
pub mod solver;

pub type Error = Box<dyn std::error::Error>;
