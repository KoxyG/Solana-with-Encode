fn main() {
    println!("Hello, Welcome to the bootcamp!");

    fizz_buzz();
}

fn fizz_buzz() {
    let mut fizz_buzz_count = 0;

    for i in 1..=301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
            fizz_buzz_count += 1;
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }

    println!("Number off times 'FizzBuzz' occurred: {}", fizz_buzz_count)

}


