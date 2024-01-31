mod common;

const INPUT: &str = r###"
var a = "global a";
var b = "global b";
var c = "global c";
{
  var a = "outer a";
  var b = "outer b";
  {
    var a = "inner a";
    print a;
    print b;
    print c;
  }
  print a;
  print b;
  print c;
}
print a;
print b;
print c;
"###;

const RESULT: &str = r###"
inner a
outer b
global c
outer a
outer b
global c
global a
global b
global c
"###;

#[test]
fn test_scope() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
