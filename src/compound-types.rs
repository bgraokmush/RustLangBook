fn compound_types() {
    //  tuple
    /*
       you can use a tuple to return multiple values from a function
       or to pass multiple values to a function
    */
    let tup = (("Hello"), 0.1);
    let (x, y) = tup;

    print!("{} {}", x, y);

    //  array
    /*
       arrays in rust have a fixed length
       once declared, they cannot grow or shrink in size
    */
    let error_codes = [404, 500, 200, 401, 403];
    let not_found = error_codes[0];

    //  create an array of 8 bytes (all 0)
    let byte = [0; 8];
}
