use rand::Rng;

mod count;
mod normal;

// pub type ArmID = usize;
pub type ActionID = usize;
pub type Reward = f64;

pub trait ArmLike<R: Rng> {
    // Generate one random value
    // from distribution, which is behind arm
    fn pull(&mut self) -> f64;
    // Generate multiple random values
    // from distribution, which is behind arm
    fn pull_miltiple(&mut self, len: usize) -> Vec<f64>;

    fn with_rng(mean: f64, std_dev: f64, rng: R) -> Self;
}

pub use count::ArmCount;
pub use count::ArmCountError;
pub use normal::ArmNorm;
