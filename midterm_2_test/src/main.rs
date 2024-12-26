/* mod MockTrait;
use MockTrait::print_len;
use MockTrait::Lengthable; */

mod MockList;
use MockList::List;


fn main() {
    /* let myvec: Vec<Box<dyn Lengthable>> = vec![
        Box::new(532),
        Box::new(String::from("helloworld")),
        Box::new(vec![1, 2, 3]),
    ];

    for elem in myvec {
        print_len(&*elem);
    } */

    let mut list = List::new();
    list.add(3);
    list.add(5);
    list.add(8);
    
    let mut list_iter = list.start.iter();
    println!("main: {:?}", list_iter.next());
    println!("main: {:?}", list_iter.next());
    println!("main: {:?}", list_iter.next());
    println!("main: {:?}", list_iter.next());

    list.print();
    list.remove(5);
    list.remove(7);
    list.print();
    println!("{}",list.pop());
    println!("{}",list.pop());
    list.print();
}
