use daru_script::language::run;

#[test]
fn addition() {
    assert_eq!(run("(3-1)+2"), 4);
    assert_eq!(run("3-1+2"), 4);
    assert_eq!(run("3-(1+2)"), 0);
    assert_eq!(run("{val x = 3; x + 1}"), 4);
}

#[test]
fn multiplication() {
    assert_eq!(run("3*2"), 6);
    assert_eq!(run("3*2+1"), 7);
    assert_eq!(run("3*(2+1)"), 9);
    assert_eq!(run("{val x = 3; x * 2}"), 6);
}

#[test]
fn division() {
    assert_eq!(run("6/2"), 3);
    assert_eq!(run("6/2+1"), 4);
    assert_eq!(run("6/(2+1)"), 2);
    assert_eq!(run("{val x = 6; x / 2}"), 3);
}