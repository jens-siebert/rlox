mod common;

const INPUT: &str = r###"
class DevonshireCream {
  serveOn() {
    return "Scones";
  }
}

print DevonshireCream;
"###;

const RESULT: &str = r###"
DevonshireCream
"###;

#[test]
fn test_class_declaration() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
