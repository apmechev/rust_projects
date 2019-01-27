fn main() {
    let number = 3;

    if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is more than 5");
    }

    //if number { // This throws E0308; if expects a bool
    //    println!("this throws an error!")}

    if number != 0{
    println!("Number is not zero")}

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition = true;
    let number = if condition { // Using if inside a let statement
        5                       // This is because if is an expression
    } else {                    // and not a statement
        6                       // but both arms of the if need to have
    };                          // the same type (statically checked)

    println!("The value of number is: {}", number);

    loops();
    iteration();
    countdown();
}

fn loops(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("counter is {}",counter);
        if counter == 10 {
            break counter * 2;  //this executes just before the break and returns it
        }                       //to the 'result' variable. (I.E. it's an expression)
    };
    assert_eq!(result, 20);
}

fn iteration(){
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value a is: {}", element);
    }

}

fn countdown(){

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");}
