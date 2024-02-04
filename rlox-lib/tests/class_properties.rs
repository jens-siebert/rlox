mod common;

const INPUT: &str = r###"
class Bagel {}
var bagel = Bagel();
bagel.taste = "delicious!";

print bagel.taste;
"###;

const RESULT: &str = r###"
delicious!
"###;

#[test]
fn test_class_declaration() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
