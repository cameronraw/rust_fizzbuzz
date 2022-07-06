use fizzbuzz::CanBeFizzBuzzed;

fn main() {
    for x in 1..=500 {
        println!("{}", x.get_fizzbuzz());
    }
}
