fn main(){

    let x = 5;
    // reassigning a value to x variable 
    let x = x + 1;
    {
        let x = x * 2;
        println!("the vslue of the inner x is: {x}");
    }
    println!("the vslue of x is : {x}");
    
}