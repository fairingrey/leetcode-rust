//---
//Wednesday, April 17th, 2019
//---
//Good morning! Here's your coding interview problem for today.
//
//This problem was asked by Google.
//
//The area of a circle is defined as πr^2. Estimate π to 3 decimal places using a Monte Carlo method.
//
//Hint: The basic equation of a circle is x2 + y2 = r2.
use rand::Rng;

pub struct Solution {}
impl Solution {
    pub fn estimate_pi(runs: usize) -> f64 {
        let mut rng = rand::thread_rng();

        f64::from((0..runs).fold(0, |acc, _| {
            if rng.gen::<f64>().powi(2) + rng.gen::<f64>().powi(2) <= 1. {
                acc + 1
            } else {
                acc
            }
        })) * 4.
            / (runs as f64)
    }
}
