const PI: u8 = 3;

fn main() {
    // let mut age = 3;

    // println!("Hello, world!{age}");
    // age = 34;
    // println!("Hello, world!{age}");
    // // const PI: u8 = 3;

    // const THREE_HORS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println(THREE_HORS_IN_SECONDS)

    // let apple = 10;
    // let apple = 20;
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value x is: {x}");
    }
    println!("The value x is: {x}");
}
