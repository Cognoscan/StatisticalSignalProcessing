use statrs::function::erf::*;

pub fn problem1(percent: f64, range: f64) {
    let c = 3e8;
    // We know t0 = R * 2/c
    // We also know PDF of t₀ ~ N(t₀, σ²)
    let target_dev = erf_inv(percent)*2.0f64.sqrt();
    let std_dev = range/target_dev * 2.0 / c;
    println!("Problem 1");
    println!("---------");
    println!("PDF for ̂R = N(R, σ² * c²/4)");
    println!("Std Dev such that  {:2.0}% of the time range will be within {:4.0}m of true value: {:.5e}",
             100.0*percent, range, std_dev);

}
