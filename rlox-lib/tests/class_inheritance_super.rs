mod common;

const INPUT: &str = r###"
class Doughnut {
  cook() {
    print "Fry until golden brown.";
  }
}

class BostonCream < Doughnut {
  cook() {
    super.cook();
    print "Pipe full of custard and coat with chocolate.";
  }
}

BostonCream().cook();
"###;

const RESULT: &str = r###"
Fry until golden brown.
Pipe full of custard and coat with chocolate.
"###;

#[test]
fn test_class_inheritance_super() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
