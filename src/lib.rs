pub type Entry = f32;

mod mats;

pub use mats::*;

#[macro_export]
macro_rules! show {
    ($mat:expr) => {
        println!(
            "{name}: {shape:?}\n{mat}",
            name = stringify!($mat),
            shape = $mat.shape(),
            mat = $mat
        )
    };
}

// Matrix Operations:
// add(matrix1, matrix2): Adds two matrices element-wise.
// subtract(matrix1, matrix2): Subtracts one matrix from another element-wise.
// multiply(matrix1, matrix2): Multiplies two matrices.
// transpose(matrix): Transposes the matrix (rows become columns and vice versa).

// scalar_multiply(matrix, scalar): Multiplies each element of the matrix by a scalar.
// is_square(matrix): Checks if the matrix is square (equal number of rows and columns).
// determinant(matrix): Calculates the determinant of a square matrix.
// inverse(matrix): Calculates the inverse of a square matrix.
// apply_function(matrix, func): Applies a given function element-wise to the matrix.

// reshape(matrix, new_shape): Changes the shape of the matrix to the specified dimensions.
