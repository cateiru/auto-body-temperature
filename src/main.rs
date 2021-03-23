use auto_temp::Temp;

fn main() {
    let temp = Temp::new(None, None).unwrap();

    for element in temp.create_multiple(14) {
        println!("{:.1}", element);
    }
}
