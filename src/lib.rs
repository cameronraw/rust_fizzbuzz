pub trait CanBeFizzBuzzed {
    fn get_fizzbuzz(&self) -> String;
}

impl CanBeFizzBuzzed for i32 {
    fn get_fizzbuzz(&self) -> String {
        match (self % 3 == 0, self % 5 == 0) {
            (true, true) => String::from("fizzbuzz"),
            (true, false) => String::from("fizz"),
            (false, true) => String::from("buzz"),
            (false, false) => self.to_string(),
        }
    }
}

pub fn fizzbuzz(number: i32) -> String {
    let mut output = String::from("");
    if number % 3 == 0 {
        output.push_str("fizz");
    }
    if number % 5 == 0 {
        output.push_str("buzz");
    }
    if output == "" {
        output = number.to_string();
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(4)]
    #[case(7)]
    #[case(8)]
    fn should_return_number_if_not_divisible_by_3_or_5(#[case] input: i32){
        assert_eq!(fizzbuzz(input), input.to_string());
    }

    #[rstest]
    #[case(3)]
    #[case(6)]
    #[case(9)]
    #[case(12)]
    #[case(15)]
    fn should_return_fizz_if_divisible_by_3(#[case] input: i32){
        assert!(fizzbuzz(input).contains("fizz"));
    }

    #[rstest]
    #[case(5)]
    #[case(10)]
    #[case(15)]
    #[case(20)]
    #[case(25)]
    fn should_return_buzz_if_divisible_by_5(#[case] input: i32){
        assert!(fizzbuzz(input).contains("buzz"));
    }

    #[rstest]
    #[case(15)]
    #[case(30)]
    #[case(45)]
    #[case(60)]
    #[case(75)]
    fn should_return_fizzbuzz_if_divisible_by_3_and_5(#[case] input: i32){
        assert!(fizzbuzz(input).contains("fizzbuzz"));
    }
}
