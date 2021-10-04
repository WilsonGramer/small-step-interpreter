use small_step::{Expression, Value};

fn main() {
    let mut expr = Expression::Let(
        "f",
        Box::new(Expression::Function(
            "a",
            Box::new(Expression::Function(
                "b",
                Box::new(Expression::Add(
                    Box::new(Expression::Name("a")),
                    Box::new(Expression::Name("b")),
                )),
            )),
        )),
        Box::new(Expression::Apply(
            Box::new(Expression::Apply(
                Box::new(Expression::Name("f")),
                Box::new(Expression::Value(Value::Number(1))),
            )),
            Box::new(Expression::Value(Value::Number(2))),
        )),
    );

    loop {
        println!("{}", expr);

        match expr {
            Expression::Value(_) => break,
            _ => match expr.evaluate() {
                Some(step) => expr = step,
                None => {
                    println!("Error");
                    break;
                }
            },
        }
    }
}
