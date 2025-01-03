use crate::{
    crash,
    expr::Expr,
    stmt::{If, Stmt},
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.expression()
    }

    pub fn parse_statements(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        if self.matches(vec![TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(
            TokenType::Identifier,
            "Je moet wel een naam aan de variabele geven",
        );

        let mut value = Expr::Lit(Literal::Nil);
        if self.matches(vec![TokenType::Equal]) {
            value = self.expression();
        }

        self.consume(TokenType::Semicolon, "Je bent de ';' vergeten druiloor");
        Stmt::Var(name, value)
    }

    fn statement(&mut self) -> Stmt {
        if self.matches(vec![TokenType::Print]) {
            return self.print_statement();
        } else if self.matches(vec![TokenType::Println]) {
            return self.println_statement();
        } else if self.matches(vec![TokenType::LeftBrace]) {
            return self.block_statement();
        } else if self.matches(vec![TokenType::If]) {
            return self.if_statement();
        } else if self.matches(vec![TokenType::While]) {
            return self.while_statement();
        } else if self.matches(vec![TokenType::For]) {
            return self.for_statement();
        }
        self.expr_statement()
    }

    fn block_statement(&mut self) -> Stmt {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RightBrace, "je bent een '}' vergeten druiloor");
        Stmt::Block(statements)
    }

    fn if_statement(&mut self) -> Stmt {
        let first_if = If::new(self.expression(), self.statement());

        let mut else_ifs = Vec::new();

        let mut other = None;
        while self.matches(vec![TokenType::Else]) {
            if self.matches(vec![TokenType::If]) {
                let else_if = If::new(self.expression(), self.statement());
                else_ifs.push(else_if);
            } else {
                other = Some(Box::new(self.statement()));
                break;
            }
        }

        Stmt::If(first_if, else_ifs, other)
    }

    fn while_statement(&mut self) -> Stmt {
        let expr = self.expression();
        let statement = self.statement();

        Stmt::While(expr, Box::new(statement))
    }

    fn for_statement(&mut self) -> Stmt {
        let name = self.consume(
            TokenType::Identifier,
            "Je moet wel een naam aan de variabele geven.",
        );
        self.consume(TokenType::From, "Verwachtte 'van'.");

        let start = self.expression();
        self.consume(TokenType::Until, "Verwachtte 'tot'.");
        let end = self.expression();

        let statement = self.statement();

        Stmt::For(name, start, end, Box::new(statement))
    }

    fn print_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Je bent een ';' vergeten druiloor");
        Stmt::Print(expr)
    }

    fn println_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Je bent een ';' vergeten druiloor");
        Stmt::Println(expr)
    }

    fn expr_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Je bent een ';' vergeten druiloor");
        Stmt::Expr(expr)
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();

        if self.matches(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment();

            match expr {
                Expr::Var(name) => return Expr::Assign(name, Box::new(value)),
                _ => crash(equals.line, "dit kan je niet assignen."),
            }
        }

        expr
    }

    fn or(&mut self) -> Expr {
        let left = self.and();

        while self.matches(vec![TokenType::Or]) {
            let op = self.previous();
            let right = self.and();
            return Expr::Logic(Box::new(left), op, Box::new(right));
        }

        left
    }

    fn and(&mut self) -> Expr {
        let left = self.equality();

        while self.matches(vec![TokenType::And]) {
            let op = self.previous();
            let right = self.equality();
            return Expr::Logic(Box::new(left), op, Box::new(right));
        }

        left
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(vec![TokenType::Plus, TokenType::Minus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(vec![TokenType::Star, TokenType::Slash]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.power();
            return Expr::Unary(op, Box::new(right));
        }

        self.power()
    }

    fn power(&mut self) -> Expr {
        let mut expr = self.primary();

        while self.matches(vec![TokenType::Caret]) {
            let op = self.previous();
            let right = self.primary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn primary(&mut self) -> Expr {
        if self.matches(vec![TokenType::True]) {
            return Expr::Lit(Literal::True);
        } else if self.matches(vec![TokenType::False]) {
            return Expr::Lit(Literal::False);
        } else if self.matches(vec![TokenType::Nil]) {
            return Expr::Lit(Literal::Nil);
        }

        if self.matches(vec![TokenType::Identifier]) {
            return Expr::Var(self.previous());
        }

        if self.matches(vec![TokenType::Number, TokenType::String]) {
            return Expr::Lit(self.previous().literal);
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(
                TokenType::RightParen,
                "Je bent de ')' vergeten (je mag niet meer op mijn kinderfeestje komen)",
            );

            return Expr::Grouping(Box::new(expr));
        }

        let str = format!("{:?} past hier niet oelewapper.", self.peek().kind);
        crash(self.peek().line, &str);
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Token {
        if self.check(token_type) {
            self.advance()
        } else {
            crash(self.peek().line, msg);
        }
    }

    fn matches(&mut self, t: Vec<TokenType>) -> bool {
        for i in 0..t.len() {
            if self.check(t[i]) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, kind: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
