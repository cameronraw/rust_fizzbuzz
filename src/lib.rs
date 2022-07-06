
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
}