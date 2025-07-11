// Tests unitaires pour timeseries.rs

use crate::data::preprocessing::{extract_features, preprocess_timeseries, smooth_timeseries};

mod preprocess_timeseries_tests {
    use super::*;
    #[test]
    fn test_basic() {
        let series = vec![vec![1.0, 2.0, 3.0]];
        let norm = preprocess_timeseries(&series);
        assert_eq!(norm[0][0], 0.0);
        assert_eq!(norm[0][1], 0.5);
        assert_eq!(norm[0][2], 1.0);
    }
    #[test]
    fn test_constant() {
        let series = vec![vec![5.0, 5.0, 5.0]];
        let norm = preprocess_timeseries(&series);
        assert_eq!(norm[0], vec![5.0, 5.0, 5.0]);
    }
    #[test]
    fn test_empty() {
        let series: Vec<Vec<f32>> = vec![vec![]];
        let norm = preprocess_timeseries(&series);
        assert_eq!(norm[0], Vec::<f32>::new());
    }
}

mod smooth_timeseries_tests {
    use super::*;
    #[test]
    fn test_basic() {
        let series = vec![vec![1.0, 2.0, 3.0, 4.0, 5.0]];
        let smoothed = smooth_timeseries(&series);
        let expected = vec![vec![1.0, 1.5, 2.0, 3.0, 4.0]];
        assert_eq!(smoothed, expected);
    }
    #[test]
    fn test_empty() {
        let series: Vec<Vec<f32>> = vec![vec![]];
        let smoothed = smooth_timeseries(&series);
        assert_eq!(smoothed[0], Vec::<f32>::new());
    }
    #[test]
    fn test_short_series() {
        let series = vec![vec![10.0, 20.0]];
        let smoothed = smooth_timeseries(&series);
        let expected = vec![vec![10.0, 15.0]];
        assert_eq!(smoothed, expected);
    }
}

mod extract_features_tests {
    use super::*;
    #[test]
    fn test_basic() {
        let series = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let features = extract_features(&series);
        assert_eq!(features[0], vec![2.0, 1.0, 3.0]);
        assert_eq!(features[1], vec![5.0, 4.0, 6.0]);
    }
    #[test]
    fn test_empty_series() {
        let series: Vec<Vec<f32>> = vec![vec![], vec![]];
        let features = extract_features(&series);
        assert_eq!(features[0], vec![0.0, 0.0, 0.0]);
        assert_eq!(features[1], vec![0.0, 0.0, 0.0]);
    }
    #[test]
    fn test_single_value() {
        let series = vec![vec![42.0]];
        let features = extract_features(&series);
        assert_eq!(features[0], vec![42.0, 42.0, 42.0]);
    }
}
