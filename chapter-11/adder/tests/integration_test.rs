use adder;

#[test]
fn testing() {
  let rec1 = adder::Rectangle::create(8, 5);
  let rec2 = adder::Rectangle::create(9, 6);
  assert!(rec2.can_hold(&rec1));
}