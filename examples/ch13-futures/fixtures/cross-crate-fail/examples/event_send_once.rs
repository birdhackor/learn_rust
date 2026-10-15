use ch13_futures::readiness::amount_pair;

fn main() {
    let (_work, event) = amount_pair();
    event.send(80);
    event.send(120);
}
