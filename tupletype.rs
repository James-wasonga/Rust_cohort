fn main(){
    tupletype();
}

fn tupletype(){
    let tup (i32,f64) =(-400,6.4);

    let (_x,_y)= tup;

    println!("The value of x is {_x}");
    println!("The value of y is {_y}");

}
