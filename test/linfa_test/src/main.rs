use ndarray::Array2; // 2d array
use linfa::prelude::*; // prelude for linfa
use ndarray::prelude::*; // prelude for ndarray


fn main() {
    let original_data: Array32<f32> = array!(
        // create sample array datas based from feature_names
        [1., 1., 1000., 1., 10.],
        [1., 0., 0., 0., 7.],
        [1., 0., 100., 1., 4.],
        [0., 1., 500., 0., 3.],
        [0., 1., 700., 0., 1.],
        [1., 0., 0., 0., 8.],
        [1., 1., 50., 1., 10.],
        [1., 1., 0., 1., 8.],
        [0., 0., 0., 0., 6.],
        [1., 1., 33., 1., 6.] // 10 rows
    );

    let feature_names = vec!["a", "b", "c", "d", "e"]; // feature names

    let num_features = original_data.len_of(Axis(1)); // get the number of features

    let features = original_data.slice(s![.., 0..num_features - 1]); // get the features
}