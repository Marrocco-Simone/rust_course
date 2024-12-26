pub mod enumx {
    pub enum X {
        Field(i32),
    }
}

pub mod structx {
    pub struct X {
        pub field: i32,
    }
}

pub mod modfun {
    use crate::question_x;
    use std::cmp::max;

    pub fn larger(x_enum: question_x::enumx::X, x_struct: question_x::structx::X) -> i32 {
        let question_x::enumx::X::Field(value) = x_enum;
        return max(value, x_struct.field);
    }
}
