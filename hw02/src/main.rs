fn main() {
    let number = 2;
    println!("{}", double_int32(number));
    println!("{}", double_int64(number));
    let number = number as f32;
    println!("{}", double_float32(number));
    println!("{}", double_float64(number));

    let a: u32 = 3;
    let b: f32 = 5.0;
    println!("{}", int_plus_float_to_float(a, b));
    println!("{}", int_plus_float_to_int(a, b));

    let tup: (i32, i32) = (2, 6);
    println!("{}", tuple_sum(tup));

    let arr: [i32; 3] = [2, 4, 6];
    println!("{}", array_sum(arr))
}

fn double_int32(number: u32) -> u32 {
    number * 2
}

fn double_int64(number: u32) -> u64 {
    (number * 2) as u64
}

fn double_float32(number: f32) -> f32 {
    number * 2.0
}

fn double_float64(number: f32) -> f64 {
    (number * 2.0) as f64
}

fn int_plus_float_to_float(a: u32, b: f32) -> f64 {
    let a: f64 = a as f64;
    let b: f64 = b as f64;
    a + b
}

fn int_plus_float_to_int(a: u32, b: f32) -> u64 {
    let a: u64 = a as u64;
    let b: u64 = b as u64;
    a + b
}

fn tuple_sum(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn array_sum(a: [i32; 3]) -> i32 {
    a.iter().sum()
}
