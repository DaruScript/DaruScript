use daru_script::language::run;

#[test]
fn addition() {
    assert_eq!(run("(3-1)+2"), 4);
    assert_eq!(run("3-1+2"), 4);
    assert_eq!(run("3-(1+2)"), 0);
    assert_eq!(run("{val x = 3; x + 1}"), 4);
}
