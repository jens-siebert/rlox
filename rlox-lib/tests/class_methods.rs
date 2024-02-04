mod common;

const INPUT: &str = r###"
class Bacon {
  eat() {
    print "Crunch crunch crunch!";
  }
}

Bacon().eat();
"###;

const RESULT: &str = r###"
Crunch crunch crunch!
"###;

#[test]
fn test_class_declaration() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
