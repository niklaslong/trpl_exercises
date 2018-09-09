extern crate adder;

mod common;

#[test]
fn creates_rectangle() {
  common::setup();
  let r = adder::Rectangle::new(2, 3);

  assert_eq!(r.width, 2);
  assert_eq!(r.length, 3);
}
