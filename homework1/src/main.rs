const freeze:f64 = 32.0;
fn fahrenheit_to_celsius(f:f64) -> f64 {
    let newTemp = (f - freeze) / 1.8;
    return newTemp;
}
fn celsius_to_fahrenheit(mut c:f64) -> f64 {
    c = (c * 1.8) + freeze;
    return c;
}

fn is_even(n:i32) -> bool {
    if n%2 == 0 {
        return true;
    }
    else {
        return false;
    }
}

fn check_guess(guess: i32, secret:i32) -> i32 {
    if guess > secret {
        return 1;
    }
    else if guess < secret {
        return -1;
    }
    else {
        return 0
    }
}

fn main() {
    let fahrTemp = 70.0;
    let FtoC = fahrenheit_to_celsius(fahrTemp);
    println!("Temperature conversion F to C: {}", FtoC);

    let tempList = [32.0,33.0,34.0,35.0,36.0,37.0];

    for idx in 0..tempList.len(){
        let res = fahrenheit_to_celsius(tempList[idx]);
        println!("{:?}", res);
    }

    let nums = [1,2,4,8,10,12,15,17,20,50];
    let mut max = 0;

    for idy in 0..nums.len(){
        if max < nums[idy] {
            max = nums[idy];
        } 
        let isEven = is_even(nums[idy]);
        if nums[idy]%3 == 0 && nums[idy]%5 == 0 {
            println!("FizzBuzz")
        }
        else if nums[idy]%3 == 0 {
            println!("Fizz")
        }
        else if nums[idy]%5 == 0 {
            println!("Buzz")
        }
        else {
            println!("{:?}", isEven);
        }
    }

    let mut i = 0;
    let mut sum = 0;
    while i<nums.len() {
        sum += nums[i];
        i = i + 1;
    }

    println!("The biggest number is: {}", max);
    println!("The sum of all numbers is: {}", sum);

    let mut secret = 3;
    let mut guess = 0;
    let mut attemps = 0;
    loop {
        if check_guess(guess, secret) == 1 {
            println!("The guess was too high");
        }
        else if check_guess(guess, secret) == -1 {
            println!("The guess was too low");
        }
        else if check_guess(guess, secret) == 0 {
            println!("The guess was correct");
            break;
        }
        guess = guess + 1;
    }
}
