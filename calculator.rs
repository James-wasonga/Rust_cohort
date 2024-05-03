fn main(){

  sum();
  subt();
  mult();
  div();
    // let resu = subt();
    // let resul = mult();
    // let result = div();


}

    

fn sum(){
    let a: u32 = 6;
    let b: u32 = 4;

    let sum_res: u32 = a + b;
    println!("The sum is: {sum_res}");

}

fn subt(){
    let a: u32 = 6;
    let b: u32 = 4;
     let sub_res: u32 = a - b;

     println!("The subtraction is: {sub_res}");

}
fn mult(){
    let a: u32 = 6;
    let b: u32 = 4;
    let mul_res: u32 = a*b;

    println!("The multiplication is: {mul_res}");

}
fn div(){
    let a: u32 = 6;
    let b: u32 = 4;
    let div_res: u32 = a/b;

    println!("The multiplication is: {div_res}")

}
