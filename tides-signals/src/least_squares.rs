// extern crate linxal;
use linxal::prelude::*;
use ndarray::{Array, Array1, Array2, ShapeError};

pub mod constituents;
use constituents::Constituent;
use crate::measurements::Measurement;

fn make_row(hour: f32) -> Vec<f32> {
    let mut row = vec![1f32, hour];
    for constituent in Constituent::constituents() {
        row.extend(constituent.evaluate(hour).iter());
    }

    row
    // Array1::<f32>::from(row)
}

/// Returns the matrix and the coefficient vector (A, b)
/// for the least square system min |Ax - b|
fn make_matrix(measurements: Vec<Measurement>) -> Result<(Array2<f32>, Array1<f32>), ShapeError> {
    let mut data : Vec<f32> = vec![];
    let n_rows = measurements.len();
    let mut n_cols = 0;
    for mes in &measurements {
        let hour = mes.timestamp.timestamp() as f32 / 3600f32;
        let row = make_row(hour);
        n_cols = row.len();
        data.extend_from_slice(&row);
    }
    
    let a_matrix : Array2<f32> = Array::from_shape_vec(
        (n_rows, n_cols), data)?;

    let b_vec : Vec<f32> = measurements.iter().cloned()
    .map(|mes| mes.value)
    .collect();
    let b : Array1<f32> = Array1::<f32>::from(b_vec);

    Ok((a_matrix, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{self, Duration, Utc};

    #[test]
    fn matrix_test() {
        let measurements = vec![
            Measurement {
                timestamp: Utc::now(),
                value: 0f32
            },
            Measurement {
                timestamp: Utc::now() + Duration::hours(1),
                value: 1f32
            },
            Measurement {
                timestamp: Utc::now() + Duration::hours(2),
                value: 2f32
            },
            Measurement {
                timestamp: Utc::now() + Duration::hours(3),
                value: 3f32
            },
            Measurement {
                timestamp: Utc::now() + Duration::hours(4),
                value: 4f32
            },
        ];

        let base_hour = measurements[0].timestamp.timestamp() as f32 / 3600f32;

        let (a_matrix, b) = make_matrix(measurements).unwrap();
        let constituents = Constituent::constituents();
        for index in 0 .. 4 {
            assert_eq!(index as f32, b[index]);

            assert_eq!(a_matrix[[index, 0]], 1f32);
            assert!((a_matrix[[index, 1]] - base_hour - index as f32).abs() <= 0.1);
            for (cindex, constituent) in constituents.iter().enumerate() {
                let [cos, sin] = constituent.evaluate(base_hour + index as f32);
                assert_eq!(a_matrix[[index, 2 * cindex + 2]], cos);
                assert_eq!(a_matrix[[index, 2 * cindex + 3]], sin);
            }
        }
    }
}