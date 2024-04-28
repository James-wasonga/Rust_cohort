fn main(){
    let tup: (f32,i32,u8) = (4.2,500,4);

    //destructuring the tuple 
    let(_x, _y, _z) = tup;
    println!("The value of x is: {_x}");

    //accessing tupple using period[.] and index
    let _ans = tup.1;
    println!("The value of y is : {_y}"); 
}