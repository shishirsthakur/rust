fn main() {
    // {} is replaced by the data and println prints in a new line
    println!("{} days", 31);

    // integers starting from zero can be used to select the argument to pass
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // this can be used to shift 1 to the right with 4 blank spaces
    println!("{number:>5}", number=1);

    // this adds zeroes before 1
    println!("{number:0>5}", number=1); // 00001
    //this adds the zeroes at the end
    println!("{number:0<5}", number=1); // 10000

    // variables with $ can be used for the same
    println!("{number:0>width$}", number=1, width=5);

    // if the number of arguments passed does not match the index, error is returned
    // println!("My name is {0}, {1} {0}", "Bond");




    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));



    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}