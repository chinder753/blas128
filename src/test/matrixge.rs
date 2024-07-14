use crate::matrix::ge::MatrixGe;
use crate::matrix::Matrix;

#[test]
fn matrixge_test() {
    let a = [[0.0, 1.0, 2.0], [4.0, 5.0, 6.0]];
    let a_matrix = MatrixGe::from_array(a);
    let b = [[4.0, 5.0, 6.0], [0.0, 1.0, 2.0]];
    let b_matrix = MatrixGe::from_array(b);

    println!("{:?}", a_matrix);
    println!("{:?}", b_matrix);

    println!("{:?}", a_matrix.add(b_matrix));
}
