use std::f32::consts::PI;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

fn compute_fft(values : &Vec<f32>) -> Vec<Complex<f32>>
{
    let mut input : Vec<Complex<f32>> = values
        .iter()
        .map(|val| Complex::from(val))
        .collect();

    let len = input.len();

    let mut output : Vec<Complex<f32>> = vec![Complex::zero(); len];

    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(len);
    fft.process(&mut input, &mut output);

    output
}

pub fn reconstruct(samples : &Vec<f32>) -> impl Fn(f32) -> f32 {
    let len = samples.len() as f32;
    let fft = compute_fft(&samples);
    move |t : f32| fft.iter()
        .enumerate()
        .map(|(k, amplitude)| {
            let phase = 2f32 * PI * k as f32 / len;
            amplitude * (Complex::from(phase * t) * Complex::i()).exp()
        })
        .sum::<Complex<f32>>()
        .re / len
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_fft() {
        let fun = |x: f32| (PI * x).cos();
        let samples : Vec<f32> = (0 .. 20).map(|i| fun(i as f32)).collect();

        // Just test that it doesn't panic
        compute_fft(&samples);
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