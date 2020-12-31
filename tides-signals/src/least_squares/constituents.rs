use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Constituent {
    name: &'static str,
    symbol: &'static str,
    period: f32 // hr
}

impl Constituent {
    // Return a frequency σ such that the period of
    // exp(-i2π σt) equals self.period.
    pub fn frequency(&self) -> f32 {
        1f32 / self.period
    }

    // Computes (cos(2π σt), sin(2π σt))
    pub fn evaluate(&self, t: f32) -> [f32; 2] {
        let argument = 2f32 * PI * self.frequency() * t;
        [argument.cos(), argument.sin()]
    }

    // source: https://en.wikipedia.org/wiki/Theory_of_tides#Tidal_constituents
    pub fn constituents() -> Vec<Constituent> {
        vec![
            Constituent {
                name: "Principal lunar semidiurnal",
                symbol: "M2",
                period: 12.4206012
            },
            Constituent {
                name: "Principal solar semidiurnal",
                symbol: "S2",
                period: 12f32
            },
            Constituent {
                name: "Larger lunar elliptic semidiurnal",
                symbol: "N2",
                period: 12.65834751
            },
            Constituent {
                name: "Lunar diurnal",
                symbol: "K1",
                period: 23.93447213
            },
            Constituent {
                name: "Lunar diurnal",
                symbol: "O1",
                period: 25.81933871
            },
            Constituent {
                name: "Shallow water overtides of principal lunar",
                symbol: "M4",
                period: 6.210300601
            },
            Constituent {
                name: "Shallow water overtides of principal lunar",
                symbol: "M6",
                period: 4.140200401
            },
            Constituent {
                name: "Shallow water terdiurnal",
                symbol: "MK3",
                period: 8.177140247
            },
            Constituent {
                name: "Shallow water overtides of principal solar",
                symbol: "S4",
                period: 6f32
            },
            Constituent {
                name: "Shallow water quarter diurnal",
                symbol: "MN4",
                period: 6.269173724
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn period_test() {
        for constituent in Constituent::constituents() {
            let [cos, sin] = constituent.evaluate(0.25 * constituent.period);
            assert!((cos - 1f32).abs() > 0.1);
            assert!(sin.abs() > 0.1);

            let [cos, sin] = constituent.evaluate(constituent.period);
            assert!((cos - 1f32).abs() < 1E-5);
            assert!(sin.abs() < 1E-5);
        }
    }
}