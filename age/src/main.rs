use rand;
mod data_types;
mod binary;
use binary::binary_search;


fn main() {
    data_types::data_types();


    let rand_num: i32 = rand::random_range(-100..100);
    println!("Random number: {}", rand_num);

    let mut age = rand_num;
    if age > 18 { println!("Adult");} else {println!("Minor");}

    age = 18;
    println!("Age: {}", age);	

    let voting_age = 18;

    match age.cmp(&voting_age) {
        std::cmp::Ordering::Less => println!("Not eligible to vote"),
        std::cmp::Ordering::Equal => println!("Just gained eligibility to vote"),
        std::cmp::Ordering::Greater => println!("Eligible to vote"),
     }

     let array = [1, 2, 3, 4, 5];
        let target = 3;
        match binary_search(&array, target) {
            Some(index) => println!("Element found at index: {}", index),
            None => println!("Element not found"),
        }
}
