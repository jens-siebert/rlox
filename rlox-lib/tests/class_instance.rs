mod common;

const INPUT: &str = r###"
class Bagel {}
var bagel = Bagel();
print bagel;
"###;

const RESULT: &str = r###"
Bagel instance
"###;

#[test]
fn test_class_declaration() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
