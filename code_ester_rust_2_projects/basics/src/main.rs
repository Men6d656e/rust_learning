use std::io;
fn main() {
    let mut input = String::new();
    println!("Please input your name:");
    io::stdin().read_line(&mut input).expect("Input Failed");
    println!("User input:{}", input);
}

// ------------------------------------------------------------------

//match

// fn main() {
//     fn is_even(num: i8) -> bool {
//         if num % 2 == 0 {
//             return true;
//         }
//         return false;
//     }

//     let number = 5;
//     match number {
//         x if is_even(x) => println!("Even"),
//         x if !is_even(x) => println!("odd"),
//         _ => println!("error"),
//     }

// let number = 5;
// match number {
//     1 | 3 => println!("number is one"),
//     2 | 4 => println!("number is two"),
//     5 => println!("number is five"),
//     _ => println!("number is not recogniziable"),
// }
// }

// -------------------------------------------------
// for loop
// fn main() {
//     let arr = [1, 2, 3];
//     println!("arr[0]={}", arr[0]);
//     println!("arr[1]={}", arr[1]);
//     println!("arr[2]={}", arr[2]);

//     for element in &arr {
//         println!("{}", element)
//     }
// }

// ---------------------------------------------
//shadowing
// fn main() {
//     let x = 5;
//     let x = "hello";
//     let x = x.len();

// }

// --------------------------------------------------------------------
// vectors

// fn main() {
//     let vrr: Vec<&str> = vec!["hello", "world", "coders"];
//     write_vrr(&vrr);
//     println!("vrr={:?}", vrr);
// }

// fn write_vrr(vrr2: &Vec<&str>) {
//     println!("vrr2={:?}", vrr2);
// }

// fn main() {
//     // let mut v: Vec<i32> = Vec::new(); //declaration
//     // another way to declare this vector
//     // let mut v = Vec::<i32>::new();
//     // v.push(1);
//     // v.push(2);
//     // v.push(3);

//     let mut v = vec![1, 2, 3, 4, 5, 6];
//     v.push(3);
//     v.pop();
//     println!("Vector v={:?}", v)
// }

// ------------------------------------------------------------
//array

// fn main() {
//     let arr: [&str; 3] = ["hello", "world", "coders"];
//     read_arry(&arr);
//     println!("arr={:?}", arr);
// }

// fn read_arry(arr1: &[&str; 3]) {
//     println!("arr1={:?}", arr1);
// }

// pass referernce
// fn main() {
//     let mut arr: [&str; 3] = ["hello", "world", "coders"];
//     write_arr(&mut arr);
//     println!("arr={:?}", arr);
// }

// fn write_arr(arr1: &mut [&str; 3]) {
//     arr1[0] = "fellow";
//     println!("arr1={:?}", arr1);
// }

// --
// cocpy entire array
// fn main() {
//     let arr: [&str; 3] = ["hello", "world", "coders"];
//     write_arr(arr);
//     println!("arr={:?}", arr);
// }

// fn write_arr(mut arr1: [&str; 3]) {
//     //it makes the copy
//     arr1[0] = "fellow";
//     println!("arr1={:?}", arr1);
// }

// fn main() {
//     // let arr1:[u8;5];//array declaration
//     let mut arr1;
//     arr1 = [1, 2, 3, 4, 5];
//     println!("arr[0]={}", arr1[0]);
//     println!("arr={:?}", arr1);

//     arr1[2] = 30;
//     println!("arr={:?}", arr1);
//     println!("Array length is {}", arr1.len());
// }
// ______________________________________________
// Dangling refrence
// fn main() {
//     let refrence_to_nothing = create_string_ref();
// }
// fn create_string_ref() -> &String {
//     let s: String = String::from("hello");
//     return &s;
// }

// __________________________________________________
// refrencing auto refrencing
// fn main() {
//     let mut x = 5;
//     println!("x={:p}", &x);
//     let y = &mut x;
//     *y = *y + 1;
//     println!("y={:p}", y);
//     println!("y={}", y);
// }

// fn main() {
//     let x = 5;
//     println!("x={:p}", &x);
//     let y = &x;
//     println!("y={:p}", y);
// }

// _____________________________________________________________________________________
// refrence rule can't makr muttliple write and read at a time it occcurs conflicts

// fn main() {
//     let mut s1: String = String::from("hello");

//     let w1 = &mut s1;
//     w1.push_str(" world");
//     // println!("r1={}", w1);

//     let w2 = &mut s1;
//     w2.push_str(" code");
//     // println!("r2={}", w2);

//     println!("r1={} r2={}", w1, w2);

//     // let r1 = &s1;
//     // let r2 = &s1;
//     // println!("r1={} r2={}", r1, r2);
// }

// ___________________________________________________________
// borrow
// fn main() {
//     let  mut s1: String = String::from("Hello");
//     append_string(&mut s1);
//     println!("The new string is this {}", s1);
//     // let len: usize = calculae_leng(&s1); //borrow operation
//     // println!("The lrngth of {} is {}", s1, len);
// }
// fn append_string(s3: &mut String) {
//     s3.push_str("world");
// }

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
