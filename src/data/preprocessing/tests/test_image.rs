// Tests unitaires pour image.rs

use crate::data::preprocessing::image::*;

mod preprocess_images_tests {
    use super::*;
    #[test]
    fn test_basic() {
        let images = vec![vec![0, 128, 255]];
        let norm = preprocess_images(&images);
        assert_eq!(norm[0][0], 0.0);
        assert_eq!(norm[0][2], 1.0);
    }
    #[test]
    fn test_empty() {
        let images: Vec<Vec<u8>> = vec![vec![]];
        let norm = preprocess_images(&images);
        assert_eq!(norm[0], Vec::<f32>::new());
    }
}

mod resize_images_tests {
    use super::*;
    #[test]
    fn test_resize_crop() {
        let images = vec![vec![1, 2, 3, 4, 5]];
        let resized = resize_images(&images, 2, 2); // target_size = 4
        assert_eq!(resized[0], vec![1, 2, 3, 4]);
    }
    #[test]
    fn test_resize_pad() {
        let images = vec![vec![1, 2]];
        let resized = resize_images(&images, 2, 2); // target_size = 4
        assert_eq!(resized[0], vec![1, 2, 0, 0]);
    }
}

mod augment_images_tests {
    use super::*;
    #[test]
    fn test_flip_horizontal() {
        let images = vec![vec![1, 2, 3]];
        let augmented = augment_images(&images);
        assert_eq!(augmented[0], vec![3, 2, 1]);
    }
    #[test]
    fn test_flip_empty() {
        let images: Vec<Vec<u8>> = vec![vec![]];
        let augmented = augment_images(&images);
        assert_eq!(augmented[0], Vec::<u8>::new());
    }
}