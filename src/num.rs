
fn main() {
    let x: i32 = 5;
    
    let  mut y: i32 = 8;
    println!("Initial y = {}", y);
    y = x;
  
    println!(" Final value of Y is {}", y);


     let v: u16 = 38_u8 as u16;
    println!("Value of V is {}", v);
    //scalars
    let age: i32 = 18;
    let num: i32 = -100;
    let temperature: f64 = 36.6;
    let is_active: bool = true;
    let grade: char = 'A';
    println!("age: {}, num: {}, temp: {}, active: {}, grade: {}", 
    age, num, temperature, is_active, grade);
    //compound type : tuple, array, slices and slice strings
    
    //tuple
    let person : (&str, u32, u32) = ("Maya", 29, 50000);
    println!("{} is {} years old and has the salary {}",person.0, person.1, person.2);

    let human : (&str, u32, u32) = ("Rabin", 24, 10);
    println!("{} is {} years old and study in grade {}", human.0, human.1, human.2);

    //array
    let scores: [i32; 4] = [90, 85, 100, 59];
    println!("first score is : {}", scores[0]);
    println!("second score is : {}", scores[1]);
    println!("Third score is : {}", scores[2]);

    let fruits: [&str; 4] = ["Apple", "Banana", "Mango", "Orange"];
    println!("First fruit is : {}", fruits[0]);
    println!("Third fruit is : {}", fruits[2]);


    //slices
    let number:&[i32] = &[1,2,3,4,5];
    println!("Slices number: {:?}", number);

    let animals: [&str; 5] = ["dog", "cat", "cow", "bear", "elephant"];

    let slice1: &[&str] = &animals[1..3];
    println!("Slices animals are: {:?}", slice1);

    let slice2: &[&str] = &animals[2..4];
    println!("Remaining animals are: {:?}", slice2);

    let slice3: &[&str] = &animals[..4];
    println!("Remaining animals are: {:?}", slice3);


    let slice4: &[&str] = &animals[1..];
    println!("Remaining animals are: {:?}", slice4);
     
    //string vs string slices
    // strings [growable, mutable, owned string type]

    let mut stone: String = String::from("Hello, ");
    stone.push_str("Yeah!");
    println!("Stone says: {}Rabin ", stone);


    //B- &str (string Slice)
    let string: String = String::from("Hello, Rabin!");
    let slice: &str = &string;
    println!("Slice Value: {}", slice);


    let string: String = String::from("Hello, Rabin!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);


}