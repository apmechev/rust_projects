fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); //parses '42' and errors out if it's not parsable
                                                           // without a type hint it will error out
                                                           //let guess = "42".parse().expect("Not a number!"); // throws E0282
                                                           // integer types: signed: i8 i16 i32 i64 i128 isize
                                                           //              unsigned: u8 u16 u31 u64 u128 usize
    let mut small: i8 = 111;
    //small = 1111; //will error out if not specifically ok

    let heart_eyed_cat = '😻';
    println!("Emoji are cool! {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (1000, 1000.0, 10);
    let (a, b, c) = tup;
    println!(
        "The value of the middle of the tuple {0} is {1} and the last is {2}",
        "(1000,1000.0,10)", b, tup.2
    );

    /////////
    //Arrays
    ////////

    //typing arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let five = add_three(2);
}

fn add_three(x: u32) -> u32 {
    println!("x is {}", x);
    let y = {
        let three = 3;
        x + three
    };
    println!("y is {}", y);
    y
}
