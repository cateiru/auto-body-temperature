use auto_temp::Temp;

fn main() {
    let temp = Temp::new(None, Some(36.7), Some(35.0));

    for element in temp.create_multiple(6) {
        println!("{:.1}", element);
    }
}
