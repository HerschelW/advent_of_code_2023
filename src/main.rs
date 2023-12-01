
pub mod modules {
    pub mod day_one;
}

use modules::day_one::day_one;

fn main() {
    let answer: i32 = day_one();
    println!("The answer is {:#?}", answer);
}
