mod question_one;
mod question_two;
mod question_x;

fn main() {
    // question 1
    let a1 = question_one::A::A1(1, 2, 3);
    println!("B: {:?}", question_one::bfroma(a1));
    let a2 = question_one::A::A2('a', 'b');
    println!("B: {:?}", question_one::bfroma(a2));

    // question 2
    let x = question_two::X {
        s: "xxx".to_string(),
        i: 0,
    };
    let y = question_two::Y {
        b: true,
        c: "yyy".to_string(),
    };
    println!("X {:?} - Y {:?}", x, y);
    let (x, y) = question_two::swapstr(x, y);
    println!("X {:?} - Y {:?}", x, y);
}
