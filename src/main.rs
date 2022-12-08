fn main() {
    // Printing to the Screen 
    println!("hello world");
    // Variables & Mutability 
    let mut x:i128=1;

    x=x+1;

    println!("x is {x}");
    

    // Arrays, Vector, Ranges, and Loops

    let a= 3;
    let b:i32 = 4;
    let arr=[3,4,5];

    // Control Flow

    // enums and structs
    
    // pub enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let x: Option<i32>=Some(4);
 
// match x {
//     None=> println!("x is none"),
//     Some(some_value)=>println!("the value is {some_value}")
// }

// let answer = if let Some(some_value)=x{
//     println!(" the value is {some_value}")
//     } else { 
//     println!("lol")
//     };
    

let answer=x.unwrap();
println!("the value is {answer}");

    // Functions

    fn my_calculation()->i32{
        8+1*4/5
        }
    let val=my_calculation();
    println!("{val}");


    fn say_hello(name: &String){
        println!("hello {name}");
    }

    let name="Dorian".to_string();


    
    say_hello(&name);
    say_hello(&name);

    








}
