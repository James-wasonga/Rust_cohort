fn main(){
  let sum = add_two(2,3);
  let product = multiply_two(2,3);
  
  println!("{sum}");

}

fn add_two(x: u32, y: u32) -> u32{
    x + y 
}

fn multiply_two(x: u32, y: u32) ->u32{
    return x * y;
}