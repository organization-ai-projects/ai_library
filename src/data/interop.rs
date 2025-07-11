use ndarray::Array2;

/// Conversion d’un dataset tabulaire vers ndarray.
pub fn to_ndarray(data: &[Vec<f32>]) -> Array2<f32> {
    let rows = data.len();
    let cols = data.get(0).map_or(0, |r| r.len());
    let flat: Vec<f32> = data.iter().flat_map(|r| r.iter().cloned()).collect();
    Array2::from_shape_vec((rows, cols), flat).unwrap_or_else(|_| Array2::zeros((rows, cols)))
}

/// Conversion d’un dataset d’images (u8) vers ndarray.
pub fn to_ndarray_u8(images: &[Vec<u8>]) -> ndarray::Array2<u8> {
    let rows = images.len();
    let cols = images.get(0).map_or(0, |r| r.len());
    let flat: Vec<u8> = images.iter().flat_map(|img| img.iter().cloned()).collect();
    ndarray::Array2::from_shape_vec((rows, cols), flat)
        .unwrap_or_else(|_| ndarray::Array2::zeros((rows, cols)))
}
