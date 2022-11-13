
//functions in rust
fn add(num_one:i8, num_two:i8) -> i8{
    return num_one + num_two;
}



fn main() {

    //variables
    let name = "ranga";             //immutable variable
    println!("{}",name);

    let mut my_name = "Pravin";     //mutable variable

    println!("Hello {my_name}");
    my_name = "ranga";
    println!("Hello, world! {}",my_name);

    let num_one:i8= 10;
    let num_two:i8 = 10;

    let mut total:i8 = add(num_one, num_two);
    println!("Addition is {}",total);



    //conditional statements

    let mut free_shipping = false;

    //if..else
    if total > 50 {
        println!("You are qualified");
        free_shipping = true;
    }
    else if total > 30 {
        println!("you are going good");
    }
    else {
        println!("need to work hard");
    }

    //match expressions
    

    match free_shipping {
        true => total += 0,
        false => total += 5

    }
    println!("{}",total);

}
