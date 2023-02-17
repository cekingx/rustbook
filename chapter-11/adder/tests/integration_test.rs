use adder;
mod common;

#[test]
fn testing() {
  common::setup();
  let rec1 = adder::Rectangle::create(8, 5);
  let rec2 = adder::Rectangle::create(9, 6);
  assert!(rec2.can_hold(&rec1));
}