#[test]
fn it_works() {
  let result = 2 + 2;
  assert_eq!(result, 4);
}

#[test]
fn this_fails() {
  assert!("".contains("Make this fail"), "Somehow this failed {}", "huh")
}
