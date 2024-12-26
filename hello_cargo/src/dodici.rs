// write a `maybediv` function that takes a dividend: i32, a divisor: i32 and returns an optional i32 value with the result of the division

pub fn maybediv(dividend: i32, divisor: i32) -> Option<i32> {
  if divisor == 0 {
    return None
  }
  Some(dividend / divisor)
}
