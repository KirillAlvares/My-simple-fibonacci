use std::io;


fn main() {
    let mut a = vec![0,1];
    println!("How many numbers from the finonacci sequence do you want to see? Enter the number greater than zero");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Can not convert a number. Enter the number greater than zero");
    if guess == 1 {
        println!("The fibinacci sequence for 1 element is [0]");
    } else if guess == 2 {
        println!("The fibinacci sequence for 2 element is [0, 1]");
    } else {
        for i in 1..guess - 1{
            a.push(a[i as usize - (1 as usize)] + a[i as usize]);
        }
        println!("The fibinacci sequence for {guess} element is {:?}", a);
    }
}
