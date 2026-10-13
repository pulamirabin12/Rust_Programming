//ownership memory management without needing garbage collector
//

const GLOBAL_CONST:u8 = 100;
fn main() {
    let outside_variable = 5;
    {
        let inside_variable = 10;
        println!("Inside variable is {}", inside_variable);
    }


    println!("Outside variable is {}", outside_variable);
    println!("{}", GLOBAL_CONST);
    print_value();

    let a = 5;
    let b = 10;
    println!("a = {}", a);
    println!("b = {}", b);

    let str1 = String::from("Hello");
    let str2 = str1;
    println!("str2={}", str2);
    
    let x:String = String ::from("Hello"); //x is the owner of Hello
    process_string(x);//transfer of ownership
   

}

fn process_string(item:String){
    println!("The value of x in process_string() is {}", item);
}

fn print_value() {
    println!("{}", GLOBAL_CONST);
}