use ch06_iterators::streams::count_down;

fn main() {
    let mut stream = count_down(3);
    println!("第一次 next={:?}", stream.next());
    println!("第二次 next={:?}", stream.next());
    println!("第三次 next={:?}", stream.next());
    println!("第四次 next={:?}", stream.next());
    println!("第五次仍是={:?}", stream.next());
}
