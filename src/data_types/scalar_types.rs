pub fn add_five (num: i32 ) -> i32 {
    let ans = num + 5;
    println!("Adding {} + 5 = {}", num, ans);
    return ans;
}

pub fn add_floats (num1:f64, num2:f64) -> f64 {
    let ans: f64 =  num1 + num2;
    println!("Adding {} + {} = {}", num1, num2, ans);
    return ans;
}

pub fn convert_int (num1: i32) -> f64 {
    let float: f64 = num1.into();
    println!("Converted num {} into {}", num1, format!("{:.10}", float));
    return float;
}

pub fn char_code (letter: char) -> u32 {
    let ucode: u32 = letter.into();
    println!("The character code for {} is {}", letter, ucode);
    return ucode;
}