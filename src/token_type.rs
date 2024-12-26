#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Println,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    //
    EOF,
    //Anders,
    //Nietus,
    //Functie,
    //Voor,
    //Als,
    //Niks,
    //Of,
    //Print,
    //Return,
    //Super,
    //Dit,
    //Wellus,
    //Var,
    //Terwijl,
}
