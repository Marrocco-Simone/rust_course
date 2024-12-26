// write an enum X with 1 variant Y with a String
// write a struct X with 1 field `i` with a String
// use the module system to give the two separate namespaces
// the enum's module is called `enumx`, the structs' module is called `structx`

// write a function `longer` in another module `modfun` that takes 2 arguments, enum X and struct X
// and it returns the longer length of the content of the arguments

pub mod enumx {
  pub enum X {
    Y(String)
  }
}

pub mod structx {
  pub struct X {
    pub i: String
  }
}

pub mod modfun {
  use std::cmp::max;
  use super::structx;
  use super::enumx;

  pub fn longer(x_enum: &enumx::X, x_struct: &structx::X) -> usize {
    let enumx::X::Y(value) = x_enum;
    
    return max(x_struct.i.len(), value.len());
  }
}