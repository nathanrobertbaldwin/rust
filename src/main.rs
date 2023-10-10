mod data_types;
use data_types::scalar_types;

fn main() {
    scalar_types::add_five(2);
    scalar_types::add_floats(2.991, -8.222);
    scalar_types::convert_int(3);
    scalar_types::char_code('A');
}
