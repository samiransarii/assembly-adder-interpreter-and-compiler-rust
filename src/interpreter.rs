use sexp::Atom::*;
use sexp::*;

// Define the expression type
#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Negate(Box<Expr>),
}

// Funtion to evaluate an expression for an interpreter
pub fn eval(e: &Expr) -> i32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add1(e1) => eval(e1) + 1,
        Expr::Sub1(e1) => eval(e1) - 1,
        Expr::Negate(e1) => -eval(e1),
    }
}

// Function to parse S-Expressions into Expr
pub fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => Expr::Add1(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::Sub1(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "negate" => Expr::Negate(Box::new(parse_expr(e))),
            _ => panic!("parse error"),
        },
        _ => panic!("parse error"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //  Basic Tests
    #[test]
    fn test_num() {
        let expr = Expr::Num(10);
        assert_eq!(eval(&expr), 10);
    }

    #[test]
    fn test_add1() {
        let expr = Expr::Add1(Box::new(Expr::Num(5)));
        assert_eq!(eval(&expr), 6);
    }

    #[test]
    fn test_sub1() {
        let expr = Expr::Sub1(Box::new(Expr::Num(5)));
        assert_eq!(eval(&expr), 4);
    }

    #[test]
    fn test_negate() {
        let expr = Expr::Negate(Box::new(Expr::Num(5)));
        assert_eq!(eval(&expr), -5);
    }

    //  Nested Expressions
    #[test]
    fn test_add1_sub1() {
        let expr = Expr::Add1(Box::new(Expr::Sub1(Box::new(Expr::Num(5)))));
        assert_eq!(eval(&expr), 5);
    }

    #[test]
    fn test_sub1_add1() {
        let expr = Expr::Sub1(Box::new(Expr::Add1(Box::new(Expr::Num(5)))));
        assert_eq!(eval(&expr), 5);
    }

    #[test]
    fn test_negate_add1() {
        let expr = Expr::Negate(Box::new(Expr::Add1(Box::new(Expr::Num(3)))));
        assert_eq!(eval(&expr), -4);
    }

    #[test]
    fn test_negate_negate() {
        let expr = Expr::Negate(Box::new(Expr::Negate(Box::new(Expr::Num(7)))));
        assert_eq!(eval(&expr), 7);
    }

    //  Zero Edge Cases
    #[test]
    fn test_add1_zero() {
        let expr = Expr::Add1(Box::new(Expr::Num(0)));
        assert_eq!(eval(&expr), 1);
    }

    #[test]
    fn test_sub1_zero() {
        let expr = Expr::Sub1(Box::new(Expr::Num(0)));
        assert_eq!(eval(&expr), -1);
    }

    #[test]
    fn test_negate_zero() {
        let expr = Expr::Negate(Box::new(Expr::Num(0)));
        assert_eq!(eval(&expr), 0);
    }

    //  Negative Numbers
    #[test]
    fn test_add1_negative() {
        let expr = Expr::Add1(Box::new(Expr::Num(-5)));
        assert_eq!(eval(&expr), -4);
    }

    #[test]
    fn test_sub1_negative() {
        let expr = Expr::Sub1(Box::new(Expr::Num(-5)));
        assert_eq!(eval(&expr), -6);
    }

    #[test]
    fn test_negate_negative() {
        let expr = Expr::Negate(Box::new(Expr::Num(-8)));
        assert_eq!(eval(&expr), 8);
    }

    //  Large Number Handling (Boundary Tests)
    #[test]
    fn test_large_add1() {
        let expr = Expr::Add1(Box::new(Expr::Num(i32::MAX - 1)));
        assert_eq!(eval(&expr), i32::MAX);
    }

    #[test]
    fn test_large_sub1() {
        let expr = Expr::Sub1(Box::new(Expr::Num(i32::MIN + 1)));
        assert_eq!(eval(&expr), i32::MIN);
    }

    #[test]
    fn test_large_negate() {
        let expr = Expr::Negate(Box::new(Expr::Num(i32::MAX)));
        assert_eq!(eval(&expr), -i32::MAX);
    }

    //  Handling of Overflow (Rust will panic if overflow checks are enabled)
    #[test]
    #[should_panic]
    fn test_overflow_add1() {
        let expr = Expr::Add1(Box::new(Expr::Num(i32::MAX)));
        eval(&expr); // This should panic due to integer overflow
    }

    #[test]
    #[should_panic]
    fn test_overflow_sub1() {
        let expr = Expr::Sub1(Box::new(Expr::Num(i32::MIN)));
        eval(&expr); // This should panic due to integer underflow
    }

    //  Deeply Nested Expressions
    #[test]
    fn test_deep_nested() {
        let expr = Expr::Add1(Box::new(Expr::Add1(Box::new(Expr::Add1(Box::new(
            Expr::Num(10),
        ))))));
        assert_eq!(eval(&expr), 13);
    }

    #[test]
    fn test_negate_deep() {
        let expr = Expr::Negate(Box::new(Expr::Negate(Box::new(Expr::Negate(Box::new(
            Expr::Num(10),
        ))))));
        assert_eq!(eval(&expr), -10);
    }

    #[test]
    fn test_mixed_operations() {
        let expr = Expr::Sub1(Box::new(Expr::Negate(Box::new(Expr::Add1(Box::new(
            Expr::Num(3),
        ))))));
        assert_eq!(eval(&expr), -5);
    }
}
