mod common;

const INPUT: &str = r###"
var a = "global";
{
  fun showA() {
    print a;
  }

  showA();
  var a = "block";
  showA();

  print a;
}
"###;

const RESULT: &str = r###"
global
global
block
"###;

#[test]
fn test_resolving() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
