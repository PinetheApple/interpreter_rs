use codecrafters_interpreter::Expr;

use crate::evaluate::evaluate;

pub fn run(expressions: Vec<Expr>) -> Result<(), ()> {
    for expr in expressions {
        run_expression(expr)?;
    }

    Ok(())
}

fn run_expression(expr: Expr) -> Result<(), ()> {
    match expr {
        Expr::PrintStatement(expr) => {
            let output = evaluate(*expr)?;
            output.print();
        }
        _ => return Err(()),
    }

    Ok(())
}
