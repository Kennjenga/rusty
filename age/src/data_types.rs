pub fn data_types() {
    // // unsigned integers
    // println!("size of usize {}", usize::MAX);
    // println!("size of u64 {}", u64::MAX);
    // println!("size of u32 {}", u32::MAX);
    // println!("size of u128 {}", u128::MAX);

    // // floating point numbers
    // println!("size of f32 {}", f32::MAX);
    // println!("size of f64 {}", f64::MAX);

    // // signed integers
    // println!("size of i32 {}", i32::MAX);
    // println!("size of i64 {}", i64::MAX);
    // println!("size of i128 {}", i128::MAX);
    // println!("size of isize {}", isize::MAX);

    // // boolean
    // let a: bool = true;

    // // with strings ":" with char `"
    // let _b: &str = "hello";
    // let _c: char = 'k';
    // let d: String = String::from("hello");

    // println!("{} : {}",d , a);

    let arr1: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // println!("arr1[0] first = {}", arr1[0]);
    // println!("length of arr1 = {}", arr1.len());

    loop {
        let mut i: usize = 0;
        while i < arr1.len() {
            println!("arr1[{}] = {}", i + 1, arr1[i].pow(2));
            i += 1;
        }
        break;
    }

    for (i, value) in arr1.iter().enumerate() {
        println!("{} = {}", i, value.pow(2));
    }
    

    let my_tuple: (u32, f32, char, String) = (1, 2.0, 'a', "name".to_string());
    println!("my_tuple = {:?}", my_tuple);

    let (a, b, c, d) = my_tuple;
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
}