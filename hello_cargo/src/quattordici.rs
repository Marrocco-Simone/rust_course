// write a function `nextchar` that takes a char and returns its precedent in the alphabet, in case of 'z' it returns '{ '
// check out what functions `std` offers for `char`

// write a function `replwithnext` that takes a string and replaces all its characters with the previous one, unless the string contains a 'z'.
// the return should be a result of either the replaced string, or a voir error

pub fn nextchar(c: char) -> char{
  if c == 'z' {
    return '{'
  }
  (c as u8 + 1) as char
}

pub fn replwithnext(s: &mut String) -> Result<String, ()> {
  let mut new_s = "".to_string();
  for c in s.chars() {
    if c == 'z' {
      return Err(())
    }
    new_s.push(nextchar(c));
  }
  Ok(new_s)
}
