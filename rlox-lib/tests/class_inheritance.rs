mod common;

const INPUT: &str = r###"
class Doughnut {
  cook() {
    print "Fry until golden brown.";
  }
}

class BostonCream < Doughnut {}

BostonCream().cook();
"###;

const RESULT: &str = r###"
Fry until golden brown.
"###;

#[test]
fn test_class_inheritance() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
