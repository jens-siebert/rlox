mod common;

const INPUT: &str = r###"
class Foo {
  init() {
    print this;
  }
}

var foo = Foo();
print foo.init();
"###;

const RESULT: &str = r###"
Foo instance
Foo instance
nil
"###;

#[test]
fn test_class_init() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
