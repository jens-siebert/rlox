mod common;

const INPUT: &str = r###"
class Cake {
  taste() {
    var adjective = "delicious";
    print "The " + this.flavor + " cake is " + adjective + "!";
  }
}

var cake = Cake();
cake.flavor = "German chocolate";
cake.taste();
"###;

const RESULT: &str = r###"
The German chocolate cake is delicious!
"###;

#[test]
fn test_class_this() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
