use std::mem;
struct Point {
    x: f64,
    y:f64
}

fn origin()->Point{
    Point{x:0.0,y:0.0}
}

pub fn heap_and_stack_memory(){
    // stack memory
    let p1:Point = origin();
    let p2 = Box::new(origin());
    println!("p1 use {} bytes while p2 use {} bytes",mem::size_of_val(&p1),mem::size_of_val(&p2));
    let p = *p2;
    println!("p2 {} {}",p.x,p.y);
}