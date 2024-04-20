


fn main() 
{
    let message: &str = "Hello, World!";
    println!("{}", message);
    let tup: (i32, i32, i32) = (1,2,3);
    let arr: [i32; 3] = [1,2,3];
    println!("{} {} {}", tup.0, tup.1, tup.2);
    println!("{} {} {}", arr[0], arr[1], arr[2]);
    msg(message);
}
fn msg(message: &str) -> &str
{
    println!("{}", message);
    return message;
}