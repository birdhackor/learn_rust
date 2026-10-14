struct Meters(i32);
struct MeterBatch(Vec<Meters>);

fn main() {
    let mut route = MeterBatch(vec![Meters(80)]);
    route.push(Meters(120));
}
