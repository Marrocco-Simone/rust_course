// mod nove;
// mod dieci;
// mod undici;
// mod dodici;
// mod tredici;
// mod quattordici;
mod quindici;

fn main() { 
    /* let xx1 = nove::XX::Y1(2,3);
    println!("data {}",nove::data(&xx1));
    let xx1 = nove::XX::Y2(String::from("test"));
    println!("data {}",nove::data(&xx1)); */
    
    /* let z1 = dieci::Z::Y1(1,2);
    let z2 = dieci::Z::Y2(true,String::from("new"));
    println!("len {:?}", dieci::maybelength(&z1));
    println!("len {:?}", dieci::maybelength(&z2)); */

    /* let ex = undici::enumx::X::Y(String::from("test"));
    let sx = undici::structx::X{i:String::from("asd")};
    println!("Longer {}", undici::modfun::longer(&ex,&sx));
    let ex = undici::enumx::X::Y(String::from("asdasd"));
    println!("Longer {}", undici::modfun::longer(&ex,&sx)); */

    /* println!("Maybediv 2/3 {:?} ", dodici::maybediv(2,3));
    println!("Maybediv 2/0 {:?} ", dodici::maybediv(2,0)); */

    /* let b = tredici::Balance{amt:100};
    println!("maybedep {:?}", b.maybedeposit(10));
    println!("maybedep {:?}", b.maybedeposit(-10)); */

    /* println!("char {}, next {}", 'c', nextchar('c'));
    println!("char {}, next {}", 'a', nextchar('a'));
    println!("char {}, next {}", 'z', nextchar('z'));
    let mut s = String::from("egna");
    println!("S {}",s);
    let ret = replwithnext(&mut s);
    println!("Returned: {:?}",ret);
    let mut s = String::from("zegna");
    println!("S {}",s);
    let ret = replwithnext(&mut s);
    println!("Returned: {:?}",ret); */

    let x = quindici::X::new();
    let y = quindici::Y::new();
    println!("X {:?} - Y {:?}", x, y);
    let (x,y) = quindici::swapstr(x,y);
    println!("X {} - Y {}", x, y);
}