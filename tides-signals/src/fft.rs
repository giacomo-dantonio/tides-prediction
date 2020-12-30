use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

fn compute_fft(input: &mut Vec<Complex<f32>>, inverse: bool) -> Vec<Complex<f32>>
{
    let len = input.len();

    let mut output : Vec<Complex<f32>> = vec![Complex::zero(); len];

    let mut planner = FFTplanner::new(inverse);
    let fft = planner.plan_fft(len);
    fft.process(input, &mut output);

    output
}

pub fn reconstruct(samples : &Vec<f32>) -> impl Fn(f32) -> f32 {
    let mut input : Vec<Complex<f32>> = samples
        .iter()
        .map(|val| Complex::from(val))
        .collect();
    let fft = compute_fft(&mut input, false);

    // set lower amplitudes to zero to smooth the signal
    let n = input.len() as f32;
    let mut smoothed = fft.iter().cloned().map(|z|
        if z.norm() > 0.1 * n { z } else { Complex::from(0f32) })
        .collect();

    let inverse = compute_fft(&mut smoothed, true);
    let len = inverse.len();

    move |t : f32| inverse[(t.round() as usize) % len].re / (len as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_fft() {
        let fun = |x: f32| (PI * x).cos();
        let mut samples : Vec<_> = (0 .. 20).map(|i| Complex::from(fun(i as f32))).collect();

        // Just test that it doesn't panic
        compute_fft(&mut samples, false);
    }

    #[test]
    fn test_reconstruction() {
        let fun = |t: f32|
            (PI * t / 20f32).cos()
            + 2f32 * (0.5 * PI * t / 20f32).cos()
            + 3f32 * (1.5 * PI * t / 20f32).cos()
            + 3.14;
        let samples : Vec<f32> = (0 .. 40).map(|i| fun(i as f32)).collect();

        let reconstructed = reconstruct(&samples);
        let predictions : Vec<f32> = (0 .. 40)
            .map(|i| reconstructed(i as f32))
            .collect();

        for (expected, actual) in samples.into_iter().zip(predictions) {
            assert!((expected - actual).abs() < 1e-3)
        }
    }
}