#[derive(Debug)]
enum Value {
    Int(i32),
    Float(f64),
    // Text(String),
}
const HANAME: &str = "harpreet";
fn main() {
    let   uname = "neha";
    // uname = "hane";

    // print!("Hello, world! {} {} \n",uname ,HANAME);
    // print!("Hello, {}  world!  {} ",uname ,HANAME);
    // print!("Hello, world! {} {}",uname ,HANAME);
    println!("Hello, world! {} {}",uname ,HANAME);

    datatypes();
    oddeven();
    itrations();
    println!("{}",add(2,4));
    let mut word = String::from("Hi");
     word.push('G');
     word.push(' ');
     word.push('H');
     word.push_str(" I am new in rust lang");
     println!("{}", word);




     let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = format!("{} {} {}", s1, s2, s3);
println!("{}", result);



    // A vector with 3 elements
    let mut cars = vec!["Volvo", "BMW", "Ford"];
  
    // Add another element
    cars.push("Mazda");
    cars[3]="TATA";
    println!("{:?}", cars); 
  


}
//   learn data types

fn datatypes() {
    let mut vl: Value = Value::Float(1.20);
    println!("value {:?}", vl);
    vl = Value::Int(10);

    match vl {
        Value::Float(v) => println!("{}", v),
        Value::Int(v) => println!("{}", v)
        // Value::Text(v) => println!("{}", v),
    }
    println!("{:?}", vl);
}

//  learn if  else

fn oddeven() {
    let score = 1;

    if score % 2 == 0 {
        println!("even");
    } else {
        println!("odd")
    }
    let number = 15;
    let result = if number < 10 { "Too small" } else { "100" };
    println!("{}", result);
}
//  learn loop
fn itrations(){
//     let mut count=1;
// let result= loop {
//     println!("neha");
//     if count==3 {
//         break count;
//     }
//     count += 1;
// };

// println!("The loop stopped at: {}", result);


// //  while loop

//  let mut  values =1;
//  while values <= 10 {
//     println!("{}",values) ;
//     values= values+1;
//  };
// //  for loop
 for i in 1..16 {
    println!("i is: {}",i)
 }
}
 

// functions 

fn add (a: i32 , b: i32)->i32 {
   return a +b;

}
