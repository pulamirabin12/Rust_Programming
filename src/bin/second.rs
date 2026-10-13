 //function
  fn main() {
    print_value(5);
    add_numbers(5, 7);
    show_message("Rust is fun");

    let num1: u8 = 10;
    let num2: u8 = 20;
    let result:u8 = add(num1, num2);
    println!("The sum of two numbers are:{} ", result);
  }

  fn print_value(item:u8){
    println!("Item value is {}", item );
  }

  fn add_numbers(a: i32, b: i32) {
    println!("Sum is: {}", a + b);
}


    fn show_message(msg: &str) {
    println!("The messages is :{}", msg);
 }

 fn add(item1:u8, item2:u8) ->u8{
    return item1+item2;
 }
