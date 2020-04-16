fn main() {
    /* Author: Shravankumar Shetty
       Organization: Dotmole
    */

    // Print Vishala
    let mut x = "Vishala";
    println!("Hello, {}!", x);
    
    // Print Shravan
    x = "Shravan";
    println!("Hello, {}!", x);

    // Using variable types
    let _y: u64 = 45;
    let _f: f32 = 6.7;
    let _b: bool = false;

    let z = 45;

    if (z == 45) {
        println!("45 found!");
    } else if z < 45 {
        println!("Less than 45!");
    } else {
        println!("More than 45");
    }
}
