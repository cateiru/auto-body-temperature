use auto_temp::Temp;

#[test]
fn create() {
  let temp = Temp::new(None, Some(36.7), Some(35.0));

  let now_temp = temp.create();

  assert!(36.7 >= now_temp && 35.0 <= now_temp, "Oops!");
}
