use std::io;

pub fn input(){

  let mut x= String::new();
  match io::stdin().read_line(&mut x){
   Ok(_)=>print!("{}",x),
   Err(error)=>print!("{}",error)
  }
}
//rustc ./src/input1.rs
//./input1