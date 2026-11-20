use nalgebra::{DVector, dmatrix};

/// Calculate the coefficients of a quintic trajectory.
///
/// The returned coefficients are ordered as `[a0, a1, a2, a3, a4, a5]`,
/// where `theta(t) = a0 + a1*t + a2*t^2 + a3*t^3 + a4*t^4 + a5*t^5`.
pub fn equation_coefficients(
    theta0: f64,
    theta_f: f64,
    v0: f64,
    v_f: f64,
    a0: f64,
    a_f: f64,
    tf: f64,
) -> Result<[f64; 6], &'static str> {
    if !tf.is_finite() || tf <= 0.0 {
        return Err("tf must be finite and greater than zero");
    }

    if [theta0, theta_f, v0, v_f, a0, a_f]
        .iter()
        .any(|value| !value.is_finite())
    {
        return Err("trajectory constraints must be finite");
    }

    let matrix = dmatrix![
        1.0, 0.0, 0.0, 0.0,            0.0,            0.0;
        0.0, 1.0, 0.0, 0.0,            0.0,            0.0;
        0.0, 0.0, 2.0, 0.0,            0.0,            0.0;
        1.0, tf,  tf.powi(2), tf.powi(3), tf.powi(4), tf.powi(5);
        0.0, 1.0, 2.0 * tf, 3.0 * tf.powi(2), 4.0 * tf.powi(3), 5.0 * tf.powi(4);
        0.0, 0.0, 2.0, 6.0 * tf, 12.0 * tf.powi(2), 20.0 * tf.powi(3);
    ];

    let constraints = DVector::from_column_slice(&[theta0, v0, a0, theta_f, v_f, a_f]);

    let coefficients = matrix
        .lu()
        .solve(&constraints)
        .ok_or("trajectory constraint matrix is singular")?;

    Ok([
        coefficients[0],
        coefficients[1],
        coefficients[2],
        coefficients[3],
        coefficients[4],
        coefficients[5],
    ])
}

pub fn equation_solution(
    coeffs: [f64; 6],
    tf: f64,
    num_points: usize,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), &'static str> {
    if !tf.is_finite() || tf < 0.0 {
        return Err("tf must be finite and non-negative");
    }

    let time = match num_points {
        0 => Vec::new(),
        1 => vec![0.0],
        count => (0..count)
            .map(|index| {
                if index == count - 1 {
                    tf
                } else {
                    tf * index as f64 / (count - 1) as f64
                }
            })
            .collect(),
    };

    let mut position = Vec::with_capacity(num_points);
    let mut velocity = Vec::with_capacity(num_points);
    let mut acceleration = Vec::with_capacity(num_points);

    for &t in &time {
        let [a0, a1, a2, a3, a4, a5] = coeffs;
        position.push(((((a5 * t + a4) * t + a3) * t + a2) * t + a1) * t + a0);
        velocity.push((((5.0 * a5 * t + 4.0 * a4) * t + 3.0 * a3) * t + 2.0 * a2) * t + a1);
        acceleration.push(((20.0 * a5 * t + 12.0 * a4) * t + 6.0 * a3) * t + 2.0 * a2);
    }

    Ok((time, position, velocity, acceleration))
}

#[cfg(test)]
mod tests {
    use super::{equation_coefficients, equation_solution};

    #[test]
    fn quintic_trajectory_meets_endpoint_constraints() {
        let start = (0.2, 0.1, -0.3);
        let end = (1.7, -0.2, 0.4);
        let tf = 2.5;
        let coeffs =
            equation_coefficients(start.0, end.0, start.1, end.1, start.2, end.2, tf).unwrap();
        let (time, position, velocity, acceleration) = equation_solution(coeffs, tf, 3).unwrap();

        assert_eq!(time, vec![0.0, tf / 2.0, tf]);
        assert!((position[0] - start.0).abs() < 1e-10);
        assert!((velocity[0] - start.1).abs() < 1e-10);
        assert!((acceleration[0] - start.2).abs() < 1e-10);
        assert!((position[2] - end.0).abs() < 1e-10);
        assert!((velocity[2] - end.1).abs() < 1e-10);
        assert!((acceleration[2] - end.2).abs() < 1e-10);
    }

    #[test]
    fn rejects_non_positive_trajectory_duration() {
        assert!(equation_coefficients(0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0).is_err());
    }
}
