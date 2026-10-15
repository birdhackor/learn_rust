use ch14_api::options::Options;

fn main() {
    let options = Options::default();
    println!("預設上限={}", options.limit().get());
}
