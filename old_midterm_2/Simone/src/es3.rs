pub fn basicbox_sum(vs: Vec<String>) -> Vec<Box<usize>> {
  let mut return_v: Vec<Box<usize>> = vec![];
  let mut sum = 0;
  
  for elem in vs.iter() {
    let lenght = elem.len();
    sum = sum + lenght;
    return_v.push(Box::new(lenght));
  }

  return_v.push(Box::new(sum));
  return return_v;
}