// define a const value
const PI: f32 = 3.14159;

fn main() {
    // rust infers the type of x
    let x = 13;
    println!("{}", x);

    // rust can also be explicit about the type
    let x: f64 = 3.14159;
    println!("{}", x);

    // rust can also declare and initialize later, but this is rarely done
    let x;
    x = 0;
    println!("{}", x);

    // mutable values only can rewrite
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);


    let x = 12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32; //add type after value with no space
    let bv = true;
    let t = (13, false); //define tuple
    let s = [13u8, 14]; //define array ** values must be same type**
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence, s[0], s[1]
    );

    //basic type conversation

    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);

    println!("{}", PI);

    // arrays :[type, len]
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}