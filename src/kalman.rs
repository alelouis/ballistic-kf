use linearkalman::KalmanFilter;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;

/// Initialize Kalman filter with problem model
pub fn init_filter() -> KalmanFilter {
    let dt = 1. / 60.;
    KalmanFilter {
        // Process noise covariance
        q: Matrix::new(
            4,
            4,
            vec![
                10.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 10.0,
            ],
        ),
        // Measurement noise matrix
        r: Matrix::new(2, 2, vec![10000.0, 0.0, 0.0, 10000.0]),
        // Observation matrix
        h: Matrix::new(2, 4, vec![1., 0., 0., 0., 0., 1., 0., 0.]),
        // State transition matrix
        f: Matrix::new(
            4,
            4,
            vec![
                1.0, 0.0, dt, 0.0, 0.0, 1.0, 0.0, dt, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        ),
        // Initial guess for state mean at time 1
        x0: Vector::new([0., 1., 10., 100.]),
        // Initial guess for state covariance at time 1
        p0: Matrix::new(
            4,
            4,
            vec![
                10.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 10.0,
            ],
        ),
    }
}
