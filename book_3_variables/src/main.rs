fn main() {
    ////////////////
    //Mutable and Immutable values
    ///////////////
    let x = 5;
    println!("x is {}", x);
    // x = 6; //this will give error E0384
    //
    let mut y = 5;
    println!("value of y is {}", y);
    y = 6;

    y = 7;
    println!("value of y is {}", y);
    y = 8;
    println!("value of y is {}. we can keep updating this", y);

    //////////////
    //Overriding Keywords
    //////////
    let r#fn = "this variable is named 'fn' even though that's a keyword";
    // can override identifiers
    println!("{}", r#fn);

    ////////////////
    //Constants
    ///////////////
    const MAXY: u32 = 1_000_000;

    //  MAXY=10; This will fail with E0070

    ////////////
    //Shadowing
    ////////////
    let shad = 10;
    let shad = shad + 1; //Keep in mind that 'let' here is a must
    let shad = shad + 2;
    //shad = shad+1; //This throws E0384
    println!("The value of shad is {}", shad);

    let shad: String = shad.to_string(); //We now made the integer into a string
                                         // We can't do that with a variable; see below:
    let mut mutshad = 5;
    // mutshad = "dd"; //This throws E0308
}
