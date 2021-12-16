use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    let mut current = -1;
    let mut input = String::new();
    File::open(Path::new("input")).unwrap().read_to_string(&mut input);
    let result = input
                .lines()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .fold(0, |acc, x| if x>current {current = x; acc+1}else{current = x; acc}) - 1;
    println!("{}",result);
}
