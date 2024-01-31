mod common;

const INPUT: &str = r###"
fun fib(n) {
  if (n <= 1) return n;
  return fib(n - 2) + fib(n - 1);
}

for (var i = 0; i < 20; i = i + 1) {
  print fib(i);
}
"###;

const RESULT: &str = r###"
0
1
1
2
3
5
8
13
21
34
55
89
144
233
377
610
987
1597
2584
4181
"###;

#[test]
fn test_fibonacci() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
