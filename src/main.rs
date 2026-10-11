mod num;
fn main() {
    println!("Hello, world!");
  
 
 //Initialized only printed
   
        let x: i32 = 5;
        let _y: i32 = 4;
        assert_eq!(x,5);
        println!("Success!");
    

//Use mut to mark variable as mutable
     let mut x: i32 =1;
     x = x+2;
     assert_eq!(x, 3);
     println!("Success!");


// Scope: A scope is the range within the program for which the item is valid

    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);







//shadowing: you can declare a new varible with same name as previous variable; just like override

    let x: i32 = 5;
    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);

    let x = 50;
    println!("{}", x);




    let mut x: i32 = 1;
    println!("Value is{}", x);
    x = 7;
    x = x+3;
    println!("Value is{}", x);

    let _y: i32 = 4;
   

    println!("success!");


    let x: i32 = 1;
    println!("{}", x);



//Destructuring: we can use a pattern with let to destructure a tuple to separate variables

    let ( mut x, y) = (1, 2);
    x = x + 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");


//Destructuring assignments: introduced in rust 1.59. you can use tuple , slice, and struct patterns as the left hand side of an assignment



    let(x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3,2]);
    println!("Success!");
  }
