use daru_script::daruscript_v1::run;

#[test]
fn addition() {
    assert_eq!(run("(3-1)+2").to_string(), "4");
    assert_eq!(run("3-1+2").to_string(), "4");
    assert_eq!(run("3-(1+2)").to_string(), "0");
    assert_eq!(run("{val x = 3; x + 1}").to_string(), "4");
}

#[test]
fn multiplication() {
    assert_eq!(run("3*2").to_string(), "6");
    assert_eq!(run("3*2+1").to_string(), "7");
    assert_eq!(run("3*(2+1)").to_string(), "9");
    assert_eq!(run("{val x = 3; x * 2}").to_string(), "6");
}

#[test]
fn division() {
    assert_eq!(run("6/2").to_string(), "3");
    assert_eq!(run("6/2+1").to_string(), "4");
    assert_eq!(run("6/(2+1)").to_string(), "2");
    assert_eq!(run("{val x = 6; x / 2}").to_string(), "3");
}

#[test]
fn closure() {
    assert_eq!(run("{x => x}(26) + {x => x}(4)").to_string(), "30");
    assert_eq!(run("{x => {y => x + y} }(3)(5)").to_string(), "8");
    assert_eq!(run("{t => t}(26) * {k => k}(6)").to_string(), "156");
    assert_eq!(run("{v => v}(24) / {z => z}(3)").to_string(), "8");
    assert_eq!(run("{l => l}({m => 2*m})(10) + 2").to_string(), "22");
}