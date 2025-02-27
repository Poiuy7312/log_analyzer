use std::f64::consts::E;

use crate::LogData;

#[derive(Clone)]
pub(crate) enum MODELTYPES {
    SCWIND,
    GO,
}

#[derive(Clone)]
pub(crate) struct Models {
    pub(crate) model: MODELTYPES,
    pub(crate) total_errors: f64,
    pub(crate) log_data: Vec<LogData>,
    pub(crate) data_point: usize,
}

impl Models {
    pub(crate) fn get_curve(self) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) /* , Vec<(f64, f64)>)*/ {
        let mut data_point: Vec<(f64, f64)> = Vec::new();
        let mut data_prime: Vec<(f64, f64)> = Vec::new();
        //let mut data_rate: Vec<(f64, f64)> = Vec::new();
        let rate = match self.model {
            MODELTYPES::SCWIND => {
                let mut temp = 0.0;
                let mut count = 1.0;
                for log in &self.log_data {
                    let current_errors = log.errors as f64;
                    temp += count * current_errors / self.total_errors;
                    count += 1.0;
                }
                temp
            }
            MODELTYPES::GO => {
                let mut temp = 0.0;
                for log in &self.log_data {
                    temp += log.errors as f64;
                }
                temp
            }
        };
        let mut starting_value = 0.1;
        let mut b = f64::NAN;
        let t = self.data_point as f64;
        while b.is_nan() {
            b = self.clone().newton_raphson(starting_value, t, rate);
            starting_value = f64::powi(starting_value, 10);
        }
        let a = match self.model {
            MODELTYPES::SCWIND => {
                b * self.total_errors / (1.0 - f64::powf(E, -b * self.data_point as f64))
            }
            MODELTYPES::GO => {
                //self.data_point as f64 / (1.0 - f64::powf(E, -(b * self.total_errors)))
                self.total_errors
            }
        };
        println!("{},{}", a, b);

        for i in 0..self.data_point {
            let (y, y_prime) = match self.model {
                MODELTYPES::SCWIND => (
                    a / b * (1.0 - f64::powf(E, -b * (i + 1) as f64)),
                    a * f64::powf(E, -b * (i + 1) as f64),
                ),
                MODELTYPES::GO => (
                    a * (1.0 - f64::powf(E, -b * (i + 1) as f64)),
                    a * b * f64::powf(E, -b * (i + 1) as f64),
                ),
            };
            //let y2 = a * f64::powf(E, -b * log_data_by_time[i].time);

            data_point.push((i as f64, y));
            data_prime.push((i as f64, y_prime))
            //data_rate.push((log_data_by_time[i].time as f64, y2));
        }
        return (data_point, data_prime); //, data_rate);
    }

    fn model(self, b: f64, t: f64) -> f64 {
        match self.model {
            MODELTYPES::SCWIND => {
                let one = 1.0 / (f64::powf(E, b) - 1.0);
                let two = t / (f64::powf(E, b * t) - 1.0);
                return one - two;
            }
            MODELTYPES::GO => {
                let n = self.total_errors;

                let one = t / b;
                let two = t * n;
                let three = f64::powf(E, -b * n) / (1.0 - f64::powf(E, -b * n));
                return one - (two * three);
            }
        };
    }

    /// Derivative of different reliability model
    fn model_prime(self, b: f64, t: f64) -> f64 {
        match self.model {
            MODELTYPES::SCWIND => {
                let one = f64::powf(E, b) / f64::powi(f64::powf(E, b) - 1.0, 2);
                let two =
                    f64::powi(t, 2) * f64::powf(E, b * t) / f64::powi(f64::powf(E, b * t) - 1.0, 2);
                return two - one;
            }
            MODELTYPES::GO => {
                let t = self.data_point as f64;
                let n = self.total_errors;
                let one = -t / f64::powi(b, 2);
                let two = t * f64::powi(n, 2);
                let three = 1.0 / (1.0 - f64::powf(E, -b * n));
                let four = f64::powf(E, -b * n) / (1.0 - f64::powf(E, (-b * n) * 2.0));
                let five = f64::powf(E, b * n);
                return one + ((two * (three + four)) * five);
            }
        };
    }
    fn newton_raphson(self, b: f64, t: f64, c: f64) -> f64 {
        let mut b = b;
        println!("{}", c);
        for _ in 0..1000 {
            let f_val = self.clone().model(b, t) - c;
            let f_deriv = self.clone().model_prime(b, t);
            let new_b = b - f_val / f_deriv;

            if (new_b - b).abs() < 1e-6 {
                return new_b;
            }
            b = new_b;
        }
        return b;
    }
}
