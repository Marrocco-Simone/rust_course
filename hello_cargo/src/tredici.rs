// write a `Balance` struct with a field `amount : i32`.
// Add a `maybedeposit` method that takes a n: i32 and adds n from the amount and returns the optional new total, unless n is a negative number.

pub struct Balance {
  pub amt: i32
}

impl Balance {
  pub fn maybedeposit(&self, n: i32) -> Option<i32> {
    if n < 0 {
      return None
    }
    Some(self.amt + n)
  }
}