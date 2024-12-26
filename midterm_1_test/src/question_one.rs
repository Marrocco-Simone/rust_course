pub enum A {
    A2(char, char),
    A1(i32, i32, i32),
}

#[derive(Debug)]
pub enum B {
    B1(i32, i32),
    B2(String),
}

pub fn bfroma(a: A) -> B {
    match a {
        A::A1(int1, int2, int3) => {
            let mut b = "".to_owned();
            b.push_str(&int1.to_string());
            b.push_str("-");
            b.push_str(&int2.to_string());
            b.push_str("-");
            b.push_str(&int3.to_string());
            B::B2(b)
        }
        A::A2(char1, char2) => {
            let int1 = char1 as i32;
            let int2 = char2 as i32;

            B::B1(int1, int2)
        }
    }
}
