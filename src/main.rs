use auto_temp::Temp;

fn main() {
    let temp = Temp::new(Some(36.5), Some(0.1)).unwrap();

    for element in temp.create_multiple(14) {
        println!("{:.1}", element);
    }
}
