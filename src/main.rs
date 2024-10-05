use itertools::join;
use std::io;

fn insert_sort(vector: &mut Vec<i16>) {
    for i in 1..vector.len() {
        let mut j = i;
        let cur_value = vector[j];
        while j != 0 && vector[j - 1] > cur_value {
            vector[j] = vector[j - 1];
            j -= 1;
        }
        vector[j] = cur_value;
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    let input = input_str.trim().split(' ');

    let mut vector: Vec<i16> = Vec::new();
    for number in input {
        vector.push(number.parse().expect("Not an integer"));
    }
    insert_sort(&mut vector);

    println!("{}", join(&vector, " "));
}
