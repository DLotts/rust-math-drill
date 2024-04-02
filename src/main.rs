use rand::prelude::*;
use text_io::try_read;

fn main() {
    for i in 1..=10 {
        let (ans, expr) = expression();
        print!("{i}: {expr} = ");
        let what: i8 = try_read!("{}").unwrap_or(100i8);
        if what == ans {
            println!("Correct");
        } else {
            println!("   = {ans}");
        }
    }
}

/**
* Retrns a negative or positive single digit add or subtract math expression as a string
* and evaluation as i8.
*/
fn expression() -> (i8, String) {
    // generate a and b random single digit:
    let mut rng = rand::thread_rng();
    let a: i8 = rng.gen_range(-9..=9);
    let b: i8 = rng.gen_range(-9..=9);
    // choose an operator + or -
    let op = "+-".chars().choose(&mut rng).unwrap();
    // whats the answer?
    let answer = if op == '-' { a - b } else { a + b };
    let expression = format!("{a} {op} {}", if b >= 0 {
        format!("{}", b)
    } else {
        format!("({})", b)
    });
    (answer, expression)
}
