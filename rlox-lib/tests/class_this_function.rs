mod common;

const INPUT: &str = r###"
class Thing {
  getCallback() {
    fun localFunction() {
      print this;
    }

    return localFunction;
  }
}

var callback = Thing().getCallback();
callback();
"###;

const RESULT: &str = r###"
Thing instance
"###;

#[test]
fn test_class_this_function() {
    assert_eq!(
        common::interpret(INPUT).unwrap(),
        RESULT.strip_prefix('\n').unwrap()
    )
}
