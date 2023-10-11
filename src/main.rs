mod data_types;
use data_types::compound_types;
use data_types::scalar_types;

fn main() {
    // Scalar Types
    scalar_types::add_five(2);
    scalar_types::add_floats(2.991, -8.222);
    scalar_types::convert_int(3);
    scalar_types::char_code('A');
    let char_type: &str = scalar_types::char_methods('A');
    println!("Char is of type: {}", char_type);

    // Compound Types
    compound_types::tuple_data(5, 'Z', true);
    compound_types::array_length([1, 2, 3, 4, 5]);
    let slice = compound_types::array_slice(&[1, 2, 3, 4, 5]);
    println!("Here is the slice from main: {:?}", slice);
    compound_types::array_slice_loop([1, 2, 3, 4, 5]);
}
