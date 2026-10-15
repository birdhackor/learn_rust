use api_contract_demo::Options;

fn main() {
    let options = Options::default();
    assert!(!options.ignore_case());
    println!("未修改的caller：ignore_case={}", options.ignore_case());
}
