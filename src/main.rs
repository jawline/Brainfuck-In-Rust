use std::io;
use std::io::{Write, Read};

pub struct BFState {
  pub pos: usize,
  pub cells: Vec<u8>
}

impl BFState {
  pub fn new() -> BFState {
    BFState { pos: 0, cells: [0; 30000].to_vec() }
  }
}

macro_rules! bf_program {

  ($var:ident; + $($rest:tt)*) => {
    $var.cells[$var.pos] += 1;
    bf_program!($var; $($rest)*); 
  };

  ($var:ident; - $($rest:tt)*) => {
    $var.cells[$var.pos] -= 1;
    bf_program!($var; $($rest)*); 
  };

  ($var:ident; > $($rest:tt)*) => {
    $var.pos += 1;
    bf_program!($var; $($rest)*);
  };

  ($var:ident; < $($rest:tt)*) => {
    $var.pos -= 1;
    bf_program!($var; $($rest)*);
  };

  ($var:ident; >> $($rest:tt)*) => {
    $var.pos += 2;
    bf_program!($var; $($rest)*);
  };

  ($var:ident; << $($rest:tt)*) => {
    $var.pos -= 2;
    bf_program!($var; $($rest)*);
  };

  ($var:ident; . $($rest:tt)*) => {
    print!("{}", $var.cells[$var.pos] as char);
    bf_program!($var; $($rest)*);
  };

  ($var:ident; .. $($rest:tt)*) => {
    bf_program!($var; . .);
    bf_program!($var; $($rest)*);
  };

  ($var:ident; ... $($rest:tt)*) => {
    bf_program!($var; . . .);
    bf_program!($var; $($rest)*);
  };

  ($var:ident; , $($rest:tt)*) => {
    $var.cells[$var.pos] = io::stdin().lock().bytes().next().unwrap().unwrap();
    bf_program!($var; $($rest)*);
  };

  ($var:ident; [ $($body:tt)* ] $($rest:tt)*) => {
    while $var.cells[$var.pos] != 0 {
      bf_program!($var; $($body)*);
    }
    bf_program!($var; $($rest)*);
  };

  ($var:ident;) => {
    io::stdout().flush().unwrap();
  };
}

macro_rules! bf_do {
  ($($body:tt)*) => {
    let mut state = BFState::new();
    bf_program!(state; $($body)*);
  }
}

fn main() {
  bf_do! { ++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>. };
  println!("Finished");
}
