use auto_temp::Temp;

#[test]
fn multiple_create() {
  let temp = Temp::new(None, None).unwrap();

  let now_temp = temp.create_multiple(100);

  assert_eq!(now_temp.len(), 100, "Oops! now_temp: {}", now_temp.len());
}
