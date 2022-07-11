mod questions;

fn main() {
    let result = questions::question_01::Solution::two_sum(
        Vec::from([1, 2, 3, 4]),
        5,
    );

    for r in result
    {
        println!("{}", r)
    }

    let result = questions::question_09::Solution::is_palindrome(121);

    println!("{}", result);

    let result = questions::question_13::Solution::roman_to_int("III".to_string());

    println!("{}", result);

    let result = questions::question_14::Solution::longest_common_prefix(
        vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]
    );

    println!("{}", result);
}
