use std::io;

fn main(){
    let mut x= String::new();
   match io::stdin().read_line(&mut x){
    Ok(_)=>print!("{}",x),
    Err(error)=>print!("{}",error)
   }
}