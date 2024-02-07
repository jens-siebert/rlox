mod common;

const INPUT: &str = r###"
class Foo {
  init() {
    return;
  }
}
"###;

const RESULT: &str = r###"
"###;

#[test]
fn test_class_init_return() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
