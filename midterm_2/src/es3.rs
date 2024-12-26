pub fn basicbox_inc(vs: Vec<String>) -> Vec<Box<usize>> {
  let mut return_v: Vec<Box<usize>> = vec![];

  for elem in vs.iter() {
    let lenght = elem.len();
    return_v.push(Box::new(lenght + 1));
  }

  return return_v;
}