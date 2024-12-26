#[derive(Debug)]
pub struct X {
    pub s: String,
    pub i: i32,
}

#[derive(Debug)]
pub struct Y {
    pub b: bool,
    pub c: String,
}

impl X {
    pub fn get_string(self) -> String {
        self.s.to_string()
    }
}

impl Y {
    pub fn get_string(self) -> String {
        self.c.to_string()
    }
}

pub fn swapstr(x: X, y: Y) -> (X, Y) {
    let x_i = x.i;
    let y_b = y.b;

    let x_str = x.get_string();
    let y_str = y.get_string();

    let new_x = X { s: y_str, i: x_i };

    let new_y = Y { b: y_b, c: x_str };

    (new_x, new_y)
}
