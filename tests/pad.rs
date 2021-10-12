use approx::assert_relative_eq;
use ndarray::{arr3, Array1, Array3};

use ndarray_image::{pad, PadMode};

fn simple_data() -> Array3<f64> {
    (0..24).map(|v| v as f64).collect::<Array1<_>>().into_shape((2, 3, 4)).unwrap()
}

#[test] // Results verified with the `pad(mode='symmetric')` function from NumPy. (v1.21.1)
fn test_pad_symmetric() {
    let data = simple_data();
    assert_relative_eq!(
        pad(data.view(), (1, 1, 1), PadMode::Symmetric),
        arr3(&[
            [
                [0.0, 0.0, 1.0, 2.0, 3.0, 3.0],
                [0.0, 0.0, 1.0, 2.0, 3.0, 3.0],
                [4.0, 4.0, 5.0, 6.0, 7.0, 7.0],
                [8.0, 8.0, 9.0, 10.0, 11.0, 11.0],
                [8.0, 8.0, 9.0, 10.0, 11.0, 11.0]
            ],
            [
                [0.0, 0.0, 1.0, 2.0, 3.0, 3.0],
                [0.0, 0.0, 1.0, 2.0, 3.0, 3.0],
                [4.0, 4.0, 5.0, 6.0, 7.0, 7.0],
                [8.0, 8.0, 9.0, 10.0, 11.0, 11.0],
                [8.0, 8.0, 9.0, 10.0, 11.0, 11.0]
            ],
            [
                [12.0, 12.0, 13.0, 14.0, 15.0, 15.0],
                [12.0, 12.0, 13.0, 14.0, 15.0, 15.0],
                [16.0, 16.0, 17.0, 18.0, 19.0, 19.0],
                [20.0, 20.0, 21.0, 22.0, 23.0, 23.0],
                [20.0, 20.0, 21.0, 22.0, 23.0, 23.0]
            ],
            [
                [12.0, 12.0, 13.0, 14.0, 15.0, 15.0],
                [12.0, 12.0, 13.0, 14.0, 15.0, 15.0],
                [16.0, 16.0, 17.0, 18.0, 19.0, 19.0],
                [20.0, 20.0, 21.0, 22.0, 23.0, 23.0],
                [20.0, 20.0, 21.0, 22.0, 23.0, 23.0]
            ]
        ])
    );
    assert_relative_eq!(
        pad(data.view(), (0, 1, 2), PadMode::Symmetric),
        arr3(&[
            [
                [1.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 2.0],
                [1.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 2.0],
                [5.0, 4.0, 4.0, 5.0, 6.0, 7.0, 7.0, 6.0],
                [9.0, 8.0, 8.0, 9.0, 10.0, 11.0, 11.0, 10.0],
                [9.0, 8.0, 8.0, 9.0, 10.0, 11.0, 11.0, 10.0]
            ],
            [
                [13.0, 12.0, 12.0, 13.0, 14.0, 15.0, 15.0, 14.0],
                [13.0, 12.0, 12.0, 13.0, 14.0, 15.0, 15.0, 14.0],
                [17.0, 16.0, 16.0, 17.0, 18.0, 19.0, 19.0, 18.0],
                [21.0, 20.0, 20.0, 21.0, 22.0, 23.0, 23.0, 22.0],
                [21.0, 20.0, 20.0, 21.0, 22.0, 23.0, 23.0, 22.0]
            ]
        ])
    );
}

#[test] // Results verified with the `pad(mode='wrap')` function from NumPy. (v1.21.1)
fn test_pad_wrap() {
    let data = simple_data();
    assert_relative_eq!(
        pad(data.view(), (1, 1, 1), PadMode::Wrap),
        arr3(&[
            [
                [23.0, 20.0, 21.0, 22.0, 23.0, 20.0],
                [15.0, 12.0, 13.0, 14.0, 15.0, 12.0],
                [19.0, 16.0, 17.0, 18.0, 19.0, 16.0],
                [23.0, 20.0, 21.0, 22.0, 23.0, 20.0],
                [15.0, 12.0, 13.0, 14.0, 15.0, 12.0]
            ],
            [
                [11.0, 8.0, 9.0, 10.0, 11.0, 8.0],
                [3.0, 0.0, 1.0, 2.0, 3.0, 0.0],
                [7.0, 4.0, 5.0, 6.0, 7.0, 4.0],
                [11.0, 8.0, 9.0, 10.0, 11.0, 8.0],
                [3.0, 0.0, 1.0, 2.0, 3.0, 0.0]
            ],
            [
                [23.0, 20.0, 21.0, 22.0, 23.0, 20.0],
                [15.0, 12.0, 13.0, 14.0, 15.0, 12.0],
                [19.0, 16.0, 17.0, 18.0, 19.0, 16.0],
                [23.0, 20.0, 21.0, 22.0, 23.0, 20.0],
                [15.0, 12.0, 13.0, 14.0, 15.0, 12.0]
            ],
            [
                [11.0, 8.0, 9.0, 10.0, 11.0, 8.0],
                [3.0, 0.0, 1.0, 2.0, 3.0, 0.0],
                [7.0, 4.0, 5.0, 6.0, 7.0, 4.0],
                [11.0, 8.0, 9.0, 10.0, 11.0, 8.0],
                [3.0, 0.0, 1.0, 2.0, 3.0, 0.0]
            ]
        ])
    );
    assert_relative_eq!(
        pad(data.view(), (0, 1, 2), PadMode::Wrap),
        arr3(&[
            [
                [10.0, 11.0, 8.0, 9.0, 10.0, 11.0, 8.0, 9.0],
                [2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0],
                [6.0, 7.0, 4.0, 5.0, 6.0, 7.0, 4.0, 5.0],
                [10.0, 11.0, 8.0, 9.0, 10.0, 11.0, 8.0, 9.0],
                [2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0]
            ],
            [
                [22.0, 23.0, 20.0, 21.0, 22.0, 23.0, 20.0, 21.0],
                [14.0, 15.0, 12.0, 13.0, 14.0, 15.0, 12.0, 13.0],
                [18.0, 19.0, 16.0, 17.0, 18.0, 19.0, 16.0, 17.0],
                [22.0, 23.0, 20.0, 21.0, 22.0, 23.0, 20.0, 21.0],
                [14.0, 15.0, 12.0, 13.0, 14.0, 15.0, 12.0, 13.0]
            ]
        ])
    );
}

#[test] // Results verified with the `pad(mode='reflect')` function from NumPy. (v1.21.1)
fn test_pad_reflect() {
    let data = simple_data();
    assert_relative_eq!(
        pad(data.view(), (1, 1, 1), PadMode::Reflect),
        arr3(&[
            [
                [17.0, 16.0, 17.0, 18.0, 19.0, 18.0],
                [13.0, 12.0, 13.0, 14.0, 15.0, 14.0],
                [17.0, 16.0, 17.0, 18.0, 19.0, 18.0],
                [21.0, 20.0, 21.0, 22.0, 23.0, 22.0],
                [17.0, 16.0, 17.0, 18.0, 19.0, 18.0]
            ],
            [
                [5.0, 4.0, 5.0, 6.0, 7.0, 6.0],
                [1.0, 0.0, 1.0, 2.0, 3.0, 2.0],
                [5.0, 4.0, 5.0, 6.0, 7.0, 6.0],
                [9.0, 8.0, 9.0, 10.0, 11.0, 10.0],
                [5.0, 4.0, 5.0, 6.0, 7.0, 6.0]
            ],
            [
                [17.0, 16.0, 17.0, 18.0, 19.0, 18.0],
                [13.0, 12.0, 13.0, 14.0, 15.0, 14.0],
                [17.0, 16.0, 17.0, 18.0, 19.0, 18.0],
                [21.0, 20.0, 21.0, 22.0, 23.0, 22.0],
                [17.0, 16.0, 17.0, 18.0, 19.0, 18.0]
            ],
            [
                [5.0, 4.0, 5.0, 6.0, 7.0, 6.0],
                [1.0, 0.0, 1.0, 2.0, 3.0, 2.0],
                [5.0, 4.0, 5.0, 6.0, 7.0, 6.0],
                [9.0, 8.0, 9.0, 10.0, 11.0, 10.0],
                [5.0, 4.0, 5.0, 6.0, 7.0, 6.0]
            ]
        ])
    );
    assert_relative_eq!(
        pad(data.view(), (0, 1, 2), PadMode::Reflect),
        arr3(&[
            [
                [6.0, 5.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0],
                [2.0, 1.0, 0.0, 1.0, 2.0, 3.0, 2.0, 1.0],
                [6.0, 5.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0],
                [10.0, 9.0, 8.0, 9.0, 10.0, 11.0, 10.0, 9.0],
                [6.0, 5.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0]
            ],
            [
                [18.0, 17.0, 16.0, 17.0, 18.0, 19.0, 18.0, 17.0],
                [14.0, 13.0, 12.0, 13.0, 14.0, 15.0, 14.0, 13.0],
                [18.0, 17.0, 16.0, 17.0, 18.0, 19.0, 18.0, 17.0],
                [22.0, 21.0, 20.0, 21.0, 22.0, 23.0, 22.0, 21.0],
                [18.0, 17.0, 16.0, 17.0, 18.0, 19.0, 18.0, 17.0]
            ]
        ])
    );
}
