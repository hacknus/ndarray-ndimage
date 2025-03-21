use approx::assert_relative_eq;
use ndarray::{arr3, Array1};

use ndarray_ndimage::{shift, zoom, BorderMode};

#[test] // Results verified with the `shift` function from SciPy. (v1.8.1)
fn test_shift() {
    let data =
        (0..27).collect::<Array1<_>>().into_shape_with_order((3, 3, 3)).unwrap().mapv(f64::from);
    assert_relative_eq!(
        shift(&data, [0.7, 0.9, 1.1], BorderMode::Mirror, true),
        arr3(&[
            [[8.7725, 7.6375, 8.4735], [6.2645, 5.1295, 5.9655], [9.6695, 8.5345, 9.3705]],
            [[4.7945, 3.6595, 4.4955], [2.2865, 1.1515, 1.9875], [5.6915, 4.5565, 5.3925]],
            [[16.6295, 15.4945, 16.3305], [14.1215, 12.9865, 13.8225], [17.5265, 16.3915, 17.2275]]
        ]),
        epsilon = 1e-5
    );
    assert_relative_eq!(
        shift(&data, [0.0, -0.5, 1.75], BorderMode::Mirror, true),
        arr3(&[
            [
                [2.8515625, 1.5703125, 1.0234375],
                [6.9765625, 5.6953125, 5.1484375],
                [6.9765625, 5.6953125, 5.1484375]
            ],
            [
                [11.8515625, 10.5703125, 10.0234375],
                [15.9765625, 14.6953125, 14.1484375],
                [15.9765625, 14.6953125, 14.1484375]
            ],
            [
                [20.8515625, 19.5703125, 19.0234375],
                [24.9765625, 23.6953125, 23.1484375],
                [24.9765625, 23.6953125, 23.1484375]
            ]
        ]),
        epsilon = 1e-5
    );
    assert_relative_eq!(
        shift(&data, [-1.17, -0.38, -0.1], BorderMode::Mirror, false),
        arr3(&[
            [
                [12.236589, 12.99325567, 13.550589],
                [14.943389, 15.70005567, 16.257389],
                [15.479933, 16.23659967, 16.793933]
            ],
            [
                [16.475967, 17.23263367, 17.789967],
                [19.182767, 19.93943367, 20.496767],
                [19.719311, 20.47597767, 21.033311]
            ],
            [
                [9.206067, 9.96273367, 10.520067],
                [11.912867, 12.66953367, 13.226867],
                [12.449411, 13.20607767, 13.763411]
            ]
        ]),
        epsilon = 1e-5
    );
}

#[test] // Results verified with the `shift` function from SciPy. (v1.10.1)
fn shift_modes() {
    let data =
        (0..18).collect::<Array1<_>>().into_shape_with_order((2, 3, 3)).unwrap().mapv(f64::from);
    assert_relative_eq!(
        shift(&data, [0.1, 0.2, 0.3], BorderMode::Constant(1.0), true),
        arr3(&[
            [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0],],
            [[1.0, 1.0, 1.0], [1.0, 11.4235, 12.7385], [1.0, 15.1435, 16.4585]],
        ]),
        epsilon = 1e-5
    );
    assert_relative_eq!(
        shift(&data, [1.1, -1.2, 1.3], BorderMode::Mirror, true),
        arr3(&[
            [[14.0725, 12.7575, 13.1995], [16.0165, 14.7015, 15.1435], [12.2965, 10.9815, 11.4235]],
            [[5.5765, 4.2615, 4.7035], [7.5205, 6.2055, 6.6475], [3.8005, 2.4855, 2.9275]]
        ]),
        epsilon = 1e-5
    );
    assert_relative_eq!(
        shift(&data, [1.1, 1.2, 1.3], BorderMode::Nearest, true),
        arr3(&[
            [
                [0.1988552, 0.10241835, 0.80532556],
                [-0.04224935, -0.1386862, 0.56422101],
                [2.39356508, 2.29712823, 3.00003544]
            ],
            [
                [-0.4045963, -0.50103315, 0.20187406],
                [-0.64570085, -0.7421377, -0.03923048],
                [1.79011358, 1.69367673, 2.39658394]
            ],
        ]),
        epsilon = 1e-5
    );
    assert_relative_eq!(
        shift(&data, [1.1, 1.2, 1.3], BorderMode::Reflect, true),
        arr3(&[
            [
                [1.39757254, 1.0427011, 1.81466135],
                [0.68192475, 0.32711659, 1.09894886],
                [3.25623966, 2.90121202, 3.67348695]
            ],
            [
                [0.05309316, -0.30171616, 0.47012713],
                [-0.66254825, -1.01729429, -0.24557899],
                [1.91177461, 1.55680909, 2.32896706]
            ],
        ]),
        epsilon = 1e-5
    );
    assert_relative_eq!(
        shift(&data, [1.1, 1.2, 1.3], BorderMode::Wrap, true),
        arr3(&[
            [[11.4235, 12.7385, 11.4235], [15.1435, 16.4585, 15.1435], [11.4235, 12.7385, 11.4235]],
            [[11.4235, 12.7385, 11.4235], [15.1435, 16.4585, 15.1435], [11.4235, 12.7385, 11.4235]]
        ]),
        epsilon = 1e-5
    );
}

#[test] // Results verified with the `zoom` function from SciPy. (v1.8.1)
fn test_zoom() {
    let data =
        (0..27).collect::<Array1<_>>().into_shape_with_order((3, 3, 3)).unwrap().mapv(f64::from);
    assert_relative_eq!(
        zoom(&data, [1.5, 1.5, 1.5], BorderMode::Mirror, true),
        arr3(&[
            [
                [0.0, 0.51851852, 1.48148148, 2.0],
                [1.55555556, 2.07407407, 3.03703704, 3.55555556],
                [4.44444444, 4.96296296, 5.92592593, 6.44444444],
                [6.0, 6.51851852, 7.48148148, 8.0]
            ],
            [
                [4.66666667, 5.18518519, 6.14814815, 6.66666667],
                [6.22222222, 6.74074074, 7.7037037, 8.22222222],
                [9.11111111, 9.62962963, 10.59259259, 11.11111111],
                [10.66666667, 11.18518519, 12.14814815, 12.66666667]
            ],
            [
                [13.33333333, 13.85185185, 14.81481481, 15.33333333],
                [14.88888889, 15.40740741, 16.37037037, 16.88888889],
                [17.77777778, 18.2962963, 19.25925926, 19.77777778],
                [19.33333333, 19.85185185, 20.81481481, 21.33333333]
            ],
            [
                [18.0, 18.51851852, 19.48148148, 20.0],
                [19.55555556, 20.07407407, 21.03703704, 21.55555556],
                [22.44444444, 22.96296296, 23.92592593, 24.44444444],
                [24.0, 24.51851852, 25.48148148, 26.0]
            ]
        ]),
        epsilon = 1e-5
    );

    assert_relative_eq!(
        zoom(&data, [0.75, 0.75, 2.0], BorderMode::Mirror, true),
        arr3(&[
            [[0.0, 0.208, 0.704, 1.296, 1.792, 2.0], [6.0, 6.208, 6.704, 7.296, 7.792, 8.0]],
            [
                [18.0, 18.208, 18.704, 19.296, 19.792, 20.0],
                [24.0, 24.208, 24.704, 25.296, 25.792, 26.0]
            ]
        ]),
        epsilon = 1e-5
    );
    assert_relative_eq!(
        zoom(&data, [0.5, 0.65, 1.75], BorderMode::Mirror, false),
        arr3(&[
            [
                [4.33333333, 4.54166667, 5.0, 5.45833333, 5.66666667],
                [8.33333333, 8.54166667, 9.0, 9.45833333, 9.66666667]
            ],
            [
                [16.33333333, 16.54166667, 17.0, 17.45833333, 17.66666667],
                [20.33333333, 20.54166667, 21.0, 21.45833333, 21.66666667]
            ]
        ]),
        epsilon = 1e-5
    );
}

#[test] // Results verified with the `zoom` function from SciPy. (v1.10.1)
fn zoom_modes() {
    let zooms = [1.1, 1.2, 1.3];
    let data =
        (0..18).collect::<Array1<_>>().into_shape_with_order((2, 3, 3)).unwrap().mapv(f64::from);
    let gt = arr3(&[
        [
            [0.0, 0.51851852, 1.48148148, 2.0],
            [1.55555556, 2.07407407, 3.03703704, 3.55555556],
            [4.44444444, 4.96296296, 5.92592593, 6.44444444],
            [6.0, 6.51851852, 7.48148148, 8.0],
        ],
        [
            [9.0, 9.51851852, 10.48148148, 11.0],
            [10.55555556, 11.07407407, 12.03703704, 12.55555556],
            [13.44444444, 13.96296296, 14.92592593, 15.44444444],
            [15.0, 15.51851852, 16.48148148, 17.0],
        ],
    ]);
    assert_relative_eq!(zoom(&data, zooms, BorderMode::Constant(0.0), true), gt, epsilon = 1e-5);
    assert_relative_eq!(zoom(&data, zooms, BorderMode::Mirror, true), gt, epsilon = 1e-5);
    assert_relative_eq!(
        zoom(&data, zooms, BorderMode::Nearest, true),
        arr3(&[
            [
                [0.0, 0.58727431, 1.41272569, 2.0],
                [1.76182294, 2.34909725, 3.17454863, 3.76182294],
                [4.23817706, 4.82545137, 5.65090275, 6.23817706],
                [6.0, 6.58727431, 7.41272569, 8.0]
            ],
            [
                [9.0, 9.58727431, 10.41272569, 11.0],
                [10.76182294, 11.34909725, 12.17454863, 12.76182294],
                [13.23817706, 13.82545137, 14.65090275, 15.23817706],
                [15.0, 15.58727431, 16.41272569, 17.0]
            ],
        ]),
        epsilon = 1e-5
    );
    assert_relative_eq!(
        zoom(&data, zooms, BorderMode::Reflect, true),
        arr3(&[
            [
                [0.00393929, 0.61167589, 1.39720607, 2.00481805],
                [1.8271483, 2.43513141, 3.22084333, 3.82848996],
                [4.18373839, 4.79199451, 5.57788234, 6.18552813],
                [6.00657444, 6.61500642, 7.40098468, 8.00859454]
            ],
            [
                [8.99682451, 9.60516597, 10.39080268, 10.99797464],
                [10.82002419, 11.42861213, 12.21443054, 12.82163712],
                [13.17592696, 13.78478782, 14.57078208, 15.17798782],
                [14.99770892, 15.60674554, 16.39283019, 17.]
            ],
        ]),
        epsilon = 1e-5
    );
    assert_relative_eq!(
        zoom(&data, zooms, BorderMode::Wrap, true),
        arr3(&[
            [
                [0.0, 0.51851852, 1.48148148, 2.0],
                [1.55555556, 2.07407407, 3.03703704, 3.55555556],
                [4.44444444, 4.96296296, 5.92592593, 6.44444444],
                [6.0, 6.51851852, 7.48148148, 8.0]
            ],
            [
                [9.0, 9.51851852, 10.48148148, 11.0],
                [10.55555556, 11.07407407, 12.03703704, 12.55555556],
                [13.44444444, 13.96296296, 14.92592593, 15.44444444],
                [15.0, 15.51851852, 16.48148148, 17.0]
            ],
        ]),
        epsilon = 1e-5
    );
}
