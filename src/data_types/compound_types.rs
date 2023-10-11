pub fn tuple_data(num1: i32, letter: char, is_true: bool) -> (i32, char, bool) {
    let tupled = (num1, letter, is_true);
    println!(
        "The values of this tuple are: {}, {}, {}",
        tupled.0, tupled.1, tupled.2
    );
    return tupled;
}

pub fn array_length(data: [i64; 5]) -> usize {
    let length: usize = data.len();
    println!("The size of this array is: {}", length);
    return length;
}

pub fn array_slice(data: &[i64; 5]) -> &[i64] {
    println!("The string slice is from: {:?}", &data[0..1]);
    return &data[0..1];
}

pub fn array_slice_loop(data: [isize; 5]) -> () {
    for ele in data {
        println!("Here is the element: {}", ele)
    }
}
