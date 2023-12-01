// --- Day 1: Trebuchet?! ---
// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

// For example:

// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

// Consider your entire calibration document. What is the sum of all of the calibration values?

pub fn day_one() -> i32 {
    let input = std::fs::read_to_string("src/resources/day_one_input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

//   find first and last digit of each line
    for line in lines {
        // remove non numeric characters
        let line = line.chars().filter(|c| c.is_numeric()).collect::<String>();
        // convert string to vector of chars
        let line: Vec<char> = line.chars().collect();
        // get first digit
        let first_digit = line[0];
        // get last digit
        let last_digit = line[line.len() - 1];
        // convert first digit to string
        let first_digit = first_digit.to_string();
        // convert last digit to string
        let last_digit = last_digit.to_string();
        // combine first and last digit
        let combined = first_digit + &last_digit;
        // convert combined string to i32
        let combined = combined.parse::<i32>().unwrap();
        // add combined to sum
        sum += combined;
    }

    return sum as i32;
}