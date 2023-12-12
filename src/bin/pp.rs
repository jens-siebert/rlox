use rlox::base::parser::{Binary, Grouping, Literal, Unary, Expr, Visitor};
use rlox::base::scanner::{Token, TokenType};

struct AstPrinter {}

impl AstPrinter {
    fn new() -> Self {
        AstPrinter {}
    }

    fn print(&self, expr: Box<dyn Expr>) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, expressions: &[&Box<dyn Expr>]) -> String {
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

impl Visitor for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String{
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        match &expr.value {
            None => String::from("nil"),
            Some(v) => v.to_string()
        }
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}

fn main() {
    let expression = Binary::new(
        Unary::new(
            Token::new(TokenType::Minus, String::from("-"), 1),
            Literal::new(Some(Box::new(123))),
        ),
        Token::new(TokenType::Star, String::from("*"), 1),
        Grouping::new(Literal::new(Some(Box::new(45.67)))),
    );

    let ast_printer = AstPrinter::new();
    println!("{}", ast_printer.print(expression));
}
