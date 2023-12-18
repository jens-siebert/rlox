use rlox::base::parser::{Expr, ExprRef, Visitor};
use rlox::base::scanner::{Token, TokenType};

struct AstPrinter {}

impl AstPrinter {
    fn new() -> Self {
        AstPrinter {}
    }

    fn print(&self, expr: ExprRef) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, expressions: &[&ExprRef]) -> String {
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
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(left, operator, right) => {
                self.parenthesize(&operator.lexeme, &[&left, &right])
            }
            Expr::Grouping(expression) => self.parenthesize("group", &[&expression]),
            Expr::Literal(value) => match &value {
                None => String::from("nil"),
                Some(v) => v.to_string(),
            },
            Expr::Unary(operator, right) => self.parenthesize(&operator.lexeme, &[&right]),
        }
    }
}

fn main() {
    let expression = Expr::binary_ref(
        Expr::unary_ref(
            Token::new_ref(TokenType::Minus, String::from("-"), 1),
            Expr::literal_ref(Some(Box::new(123))),
        ),
        Token::new_ref(TokenType::Star, String::from("*"), 1),
        Expr::grouping_ref(Expr::literal_ref(Some(Box::new(45.67)))),
    );

    let ast_printer = AstPrinter::new();
    println!("{}", ast_printer.print(expression));
}
