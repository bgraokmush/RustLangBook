fn variables() {
    // Variables are immutable by default
    // To make them mutable, use the mut keyword
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    /*
       Shadowing is different
       because we’re effectively
       creating a new variable when
       we use the let keyword again
    */
    let y = 5;
    println!("The value of y is: {}", y);
    let y = y + 1;
    println!("The value of y is: {}", y);

    // Constants are always immutable
    const MAX_COUNT: u32 = 25_6;
}
