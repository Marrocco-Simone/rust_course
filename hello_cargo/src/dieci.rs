// write enum Z with variants Y1 Y2, the former takes 2 i32, the latter a bool and a string
// write a function `maybelength` that returns the optional length of the string

pub enum Z {
  Y1(i32, i32),
  Y2(bool, String)
}

pub fn maybelength(z: &Z) -> Option<i32>{
  match z {
    Z::Y1(_, _) => None,
    Z::Y2(_b, s) => Some(s.len() as i32)
  }
}