fn main() {
    let  mut s1: String = String::from("Hello");
    append_string(&mut s1);
    println!("The new string is this {}", s1);
    // let len: usize = calculae_leng(&s1); //borrow operation
    // println!("The lrngth of {} is {}", s1, len);
}
fn append_string(s3: &mut String) {
    s3.push_str("world");
}

// fn calculae_leng(s2: &String) -> usize {
//     return s2.len();
// }


// ________________________________________
// avoidig ownership

// fn main() {
//     let s1: String = String::from("Hello");
//     let len: usize = calculae_leng(s1.clone());

//     println!("The lrngth of {} is {}", s1, len);
// }

// fn calculae_leng(s: String) -> usize {
//     let length:usize = s.len();
//     return length;
// }

// fn main() {
//     let s1: String = String::from("Hello");
//     // let len: usize = calculae_leng(s1);
//     let (s2,len) = calculae_leng(s1);

//     println!("The lrngth of {} is {}", s2, len);
// }

// fn calculae_leng(s: String) -> (String,usize) {
//     let length:usize = s.len();
//     return (s,length);
// }

// fn main() {
//     let s1: String = get_str();
//     println!("This is s1:{}", s1);
//     let s2: String = String::from("World");
//     let s3: String = send_get_string(s2);
//     println!("This is s3:{}", s3);
// }

// fn get_str() -> String {
//     let new_string = String::from("hello");
//     return new_string;
// }

// fn send_get_string(received_string: String) -> String {
//     return received_string;
// }

// ____________________________________________________
// ownership

// fn main() {
//     let x: String = String::from("Hello"); //x is the owner of hello
//     process_string(x); //transfer of the ownership
//     // println!("The value of x in main() is {}", x);
// }

// fn process_string(item: String) {
//     //hello -new owner is itme
//     println!("The value of x is in Process_string() id {}", item);
// }

// fn main() {
//     let x: u8 = 5; //memory
//     process_int(x);
//     println!("The value of x in main() is {}", x);
// }

// fn process_int(item: u8) {
//     println!("The value of x is in Process_string() id {}", item);
// }

// fn main() {
//     // let a = 5;
//     // let b = a;
//     // println!("a={}",a);
//     // println!("b={}",b);

//     let str1 = String::from("Hello");
//     let str2 = str1;
//     println!("str1={}", str1);
//     println!("Str2={}", str2);
// }

// _____________________________________________________________________
// Functions
// fn main() {
//     let n1: u8 = 10;
//     let n2: u8 = 20;
//     let result: u8 = add(n1, n2);
//     println!("{}", result);
//     // print_value(5);
// }

// fn add(num1: u8, num2: u8) -> u8 {
//     return num1 + num2;
// }

// fn print_value(item: u8) {
//     println!("value is {} ", item);
//     println!("Hello World!");
// }

// _____________________________________________________________
// Tupple
// fn main() {
//     let emp_info: (&str, u8) = ("Akash", 30);
//     let emp_name = emp_info.0;
//     let emp_age = emp_info.1;
//     println!("Name {emp_name}, Age {emp_age}");

//     // destructring
//     let (employ_name, employ_age) = emp_info;
//     println!("Name {}, Age {}", employ_name, employ_age);
// }

// _________________________________________________________
// String
// fn main() {
//     // String - Dynamic Length String - Heap Allocated
//     // &str - Fixed lenght String  --special memory
//     let mut string_literal: String = String::from("Hi, Akash!!!");
//     string_literal.push_str("What's Up?");
//     println!("This is Strinng litearal: {string_literal}");
// }
