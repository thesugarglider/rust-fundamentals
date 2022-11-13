fn add(num_one:i8, num_two:i8) -> i8{
    return num_one + num_two;
}



fn main() {
    let mut my_name = "Pravin";
    println!("Hello {my_name}");
    my_name = "ranga";
    println!("Hello, world! {}",my_name);

    let num_one:i8= 1;
    let num_two:i8 = 2;

    let total:i8 = add(num_one, num_two);
    println!("Addition is {}",total);

}
