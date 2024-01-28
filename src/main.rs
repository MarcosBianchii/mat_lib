use mat_lib::show;
use mat_lib::sparse::Sparse;

fn main() {
    let mut mat = Sparse::<i32, 25, 10>::zeros();
    mat.set((0, 0), 1);
    mat.set((1, 1), 2);
    mat.set((2, 2), 3);
    show!(mat);
}
