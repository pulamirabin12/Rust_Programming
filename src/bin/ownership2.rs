fn main(){
    let s1:String = get_string(); //s1 is the owner of the hello
    println!("The is s1: {}", s1);

    let s2:String = String::from("World");//s2 is the owner of the world
    let s3: String = send_get_string(s2); //1.transfer of ownership to received string
    println!("This is s3:{}", s3);//s3 is the new owner


    let s4:String = String::from("Hello");
    let (s5,len) = calculate_length(s4);
    println!("The length of {} is {}", s5, len);
}

fn get_string()->String{
    let new_string: String = String::from ("Hello"); //new_string ownership
    return new_string;
}


//received_string onwer of the world
fn send_get_string(received_string:String)->String{
    return received_string;//transfer of ownership from received to s3
}

fn calculate_length(s:String)->(String,usize) {
    let length = s.len();
    return (s,length);
}