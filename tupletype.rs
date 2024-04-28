fn main(){
    let tup: (f32,i32,u8) = (4.2,500,4);

    //destructuring the tuple 
    let(_x, _y, _z) = tup;
    println!("The value of x is: {_x}");

    
}