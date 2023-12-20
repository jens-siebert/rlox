use rlox::base::parser::{Expr, Visitor};
use rlox::base::scanner::{Token, TokenType};

struct AstPrinter {}

impl AstPrinter {
    fn new() -> Self {
        AstPrinter {}
    }

    fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, expressions: &[&Expr]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for expr in expressions {
            result.push(' ');
            result.push_str(expr.accept(self).as_str());
        }
        result.push(')');

        result
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => {
                self.parenthesize(&operator.lexeme, &[&left, &right])
            }
            Expr::Grouping { expression } => self.parenthesize("group", &[&expression]),
            Expr::Literal { value } => match &value {
                None => String::from("nil"),
                Some(v) => v.to_string(),
            },
            Expr::Unary { operator, right } => self.parenthesize(&operator.lexeme, &[&right]),
        }
    }
}

fn main() {
    let op_minus = &Token::new(TokenType::Minus, String::from("-"), 1);
    let literal_1 = &Expr::literal(Some(Box::new(123)));
    let unary = &Expr::unary(op_minus, literal_1);
    let op_star = &Token::new(TokenType::Star, String::from("*"), 1);
    let literal_2 = &Expr::literal(Some(Box::new(45.67)));
    let group = &Expr::grouping(literal_2);
    let expression = &Expr::binary(unary, op_star, group);

    let ast_printer = AstPrinter::new();
    println!("{}", ast_printer.print(expression));
}
