pub struct Wrapper {
    pub v: Vec<i32>,
}

impl Wrapper {
  pub fn iter(self) -> WrapperIter {
    return self.into_iter()
  }
}

impl IntoIterator for Wrapper {
    type Item = i32;
    type IntoIter = WrapperIter;

    fn into_iter(self) -> Self::IntoIter {
        WrapperIter {
            wrapper: self,
            index: 0,
        }
    }
}

pub struct WrapperIter {
    wrapper: Wrapper,
    index: usize,
}

impl Iterator for WrapperIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.wrapper.v.get(self.index);
        self.index += 2;
        return match result {
            Some(x) => Some(x.clone()),
            None => None,
        };
    }
}
