mod common;

const INPUT: &str = r###"
fun count(n) {
  if (n > 1) count(n - 1);
  print n;
}

count(3);
"###;

const RESULT: &str = r###"
1
2
3
"###;

#[test]
fn test_recursion() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
