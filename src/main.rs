use rust_kakeibo_app::services;
use std::io;

fn main() {
    let mut service_type = String::new();
    println!("実行居たい内容を入力してください(0: 登録, 1: 集計)");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("数値で入力してください");
    services::validate::InputValidator::validate_service_type(service_type);

    if service_type == 0 {
        println!("toouroku service");
    } else if service_type == 1 {
        println!("shukei service");
    }
}
