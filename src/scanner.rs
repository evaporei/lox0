use crate::error;
use crate::token::{Token, TokenType};

const KEYWORDS: &[(&str, TokenType)] = &[
    ("and", TokenType::And),
    ("or", TokenType::Or),
    ("true", TokenType::True),
    ("false", TokenType::False),
    ("class", TokenType::Class),
    ("super", TokenType::Super),
    ("this", TokenType::This),
    ("var", TokenType::Var),
    ("fun", TokenType::Fun),
    ("return", TokenType::Return),
    ("if", TokenType::If),
    ("else", TokenType::Else),
    ("this", TokenType::This),
    ("while", TokenType::While),
    ("for", TokenType::For),
    ("print", TokenType::Print),
];

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".into(), self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let ty = match self.advance() {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' => {
                if self.match_('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '/' => {
                if self.match_('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return;
                } else {
                    TokenType::Slash
                }
            }
            // Ignore whitespace
            ' ' | '\r' | '\t' => {
                return;
            }
            '\n' => {
                self.line += 1;
                return;
            }
            '"' => TokenType::String(self.string()),
            c => {
                if self.is_digit(c) {
                    TokenType::Number(self.number())
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    self.error("Unexpected character.")
                }
            }
        };

        self.add_token(ty);
    }

    fn identifier(&mut self) -> TokenType {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        match KEYWORDS.iter().find(|(k, _)| k == &text) {
            Some((_, ty)) => ty.clone(),
            None => TokenType::Identifier(text.to_string()),
        }
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) -> f64 {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.source[self.start..self.current].parse().unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn string(&mut self) -> String {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
        }

        self.advance();

        (&self.source[self.start + 1..self.current - 1]).into()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, ty: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(ty, text.into(), self.line));
    }

    fn error(&self, msg: &str) -> ! {
        error::error(self.line, msg)
    }
}

#[test]
fn test_print() {
    let source = "print \"Hello, world!\";";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Print, "print".into(), 1),
            Token::new(
                TokenType::String("Hello, world!".into()),
                "\"Hello, world!\"".into(),
                1
            ),
            Token::new(TokenType::Semicolon, ";".into(), 1),
            Token::new(TokenType::EOF, "".into(), 1),
        ]
    );
}

#[test]
fn test_boolean() {
    let source = "
    true;  // Not false.
    false; // Not *not* false.
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::True, "true".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::False, "false".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::EOF, "".into(), 4),
        ]
    );
}

#[test]
fn test_numbers() {
    let source = "
    1234;  // An integer.
    12.34; // A decimal number.
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Number(1234.0), "1234".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::Number(12.34), "12.34".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::EOF, "".into(), 4),
        ]
    );
}

#[test]
fn test_strings() {
    let source = "
    \"I am a string\";
    \"\";    // The empty string.
    \"123\"; // This is a string, not a number.
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(
                TokenType::String("I am a string".into()),
                "\"I am a string\"".into(),
                2
            ),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::String("".into()), "\"\"".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::String("123".into()), "\"123\"".into(), 4),
            Token::new(TokenType::Semicolon, ";".into(), 4),
            Token::new(TokenType::EOF, "".into(), 5),
        ]
    );
}

#[test]
fn test_arithmetic() {
    let source = "
    add + me;
    subtract - me;
    multiply * me;
    divide / me;
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Identifier("add".into()), "add".into(), 2),
            Token::new(TokenType::Plus, "+".into(), 2),
            Token::new(TokenType::Identifier("me".into()), "me".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(
                TokenType::Identifier("subtract".into()),
                "subtract".into(),
                3
            ),
            Token::new(TokenType::Minus, "-".into(), 3),
            Token::new(TokenType::Identifier("me".into()), "me".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(
                TokenType::Identifier("multiply".into()),
                "multiply".into(),
                4
            ),
            Token::new(TokenType::Star, "*".into(), 4),
            Token::new(TokenType::Identifier("me".into()), "me".into(), 4),
            Token::new(TokenType::Semicolon, ";".into(), 4),
            Token::new(TokenType::Identifier("divide".into()), "divide".into(), 5),
            Token::new(TokenType::Slash, "/".into(), 5),
            Token::new(TokenType::Identifier("me".into()), "me".into(), 5),
            Token::new(TokenType::Semicolon, ";".into(), 5),
            Token::new(TokenType::EOF, "".into(), 6),
        ]
    );
}

#[test]
fn test_negate() {
    let source = "
    -negateMe;
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Minus, "-".into(), 2),
            Token::new(
                TokenType::Identifier("negateMe".into()),
                "negateMe".into(),
                2
            ),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::EOF, "".into(), 3),
        ]
    );
}

#[test]
fn test_comparison() {
    let source = "
    less < than;
    lessThan <= orEqual;
    greater > than;
    greaterThan >= orEqual;
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Identifier("less".into()), "less".into(), 2),
            Token::new(TokenType::Less, "<".into(), 2),
            Token::new(TokenType::Identifier("than".into()), "than".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(
                TokenType::Identifier("lessThan".into()),
                "lessThan".into(),
                3
            ),
            Token::new(TokenType::LessEqual, "<=".into(), 3),
            Token::new(TokenType::Identifier("orEqual".into()), "orEqual".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::Identifier("greater".into()), "greater".into(), 4),
            Token::new(TokenType::Greater, ">".into(), 4),
            Token::new(TokenType::Identifier("than".into()), "than".into(), 4),
            Token::new(TokenType::Semicolon, ";".into(), 4),
            Token::new(
                TokenType::Identifier("greaterThan".into()),
                "greaterThan".into(),
                5
            ),
            Token::new(TokenType::GreaterEqual, ">=".into(), 5),
            Token::new(TokenType::Identifier("orEqual".into()), "orEqual".into(), 5),
            Token::new(TokenType::Semicolon, ";".into(), 5),
            Token::new(TokenType::EOF, "".into(), 6),
        ]
    );
}

#[test]
fn test_equality() {
    let source = "
    1 == 2;         // false.
    \"cat\" != \"dog\"; // true.

    // different types
    314 == \"pi\"; // false.
    123 == \"123\"; // false.
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Number(1.0), "1".into(), 2),
            Token::new(TokenType::EqualEqual, "==".into(), 2),
            Token::new(TokenType::Number(2.0), "2".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::String("cat".into()), "\"cat\"".into(), 3),
            Token::new(TokenType::BangEqual, "!=".into(), 3),
            Token::new(TokenType::String("dog".into()), "\"dog\"".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::Number(314.0), "314".into(), 6),
            Token::new(TokenType::EqualEqual, "==".into(), 6),
            Token::new(TokenType::String("pi".into()), "\"pi\"".into(), 6),
            Token::new(TokenType::Semicolon, ";".into(), 6),
            Token::new(TokenType::Number(123.0), "123".into(), 7),
            Token::new(TokenType::EqualEqual, "==".into(), 7),
            Token::new(TokenType::String("123".into()), "\"123\"".into(), 7),
            Token::new(TokenType::Semicolon, ";".into(), 7),
            Token::new(TokenType::EOF, "".into(), 8),
        ]
    );
}

#[test]
fn test_logical_operators() {
    let source = "
    !true;  // false.
    !false; // true.
    
    true and false; // false.
    true and true;  // true.
 
    false or false; // false.
    true or false;  // true.
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Bang, "!".into(), 2),
            Token::new(TokenType::True, "true".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::Bang, "!".into(), 3),
            Token::new(TokenType::False, "false".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::True, "true".into(), 5),
            Token::new(TokenType::And, "and".into(), 5),
            Token::new(TokenType::False, "false".into(), 5),
            Token::new(TokenType::Semicolon, ";".into(), 5),
            Token::new(TokenType::True, "true".into(), 6),
            Token::new(TokenType::And, "and".into(), 6),
            Token::new(TokenType::True, "true".into(), 6),
            Token::new(TokenType::Semicolon, ";".into(), 6),
            Token::new(TokenType::False, "false".into(), 8),
            Token::new(TokenType::Or, "or".into(), 8),
            Token::new(TokenType::False, "false".into(), 8),
            Token::new(TokenType::Semicolon, ";".into(), 8),
            Token::new(TokenType::True, "true".into(), 9),
            Token::new(TokenType::Or, "or".into(), 9),
            Token::new(TokenType::False, "false".into(), 9),
            Token::new(TokenType::Semicolon, ";".into(), 9),
            Token::new(TokenType::EOF, "".into(), 10),
        ]
    );
}

#[test]
fn test_precedence_and_grouping() {
    let source = "
    var average = (min + max) / 2;
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Var, "var".into(), 2),
            Token::new(TokenType::Identifier("average".into()), "average".into(), 2),
            Token::new(TokenType::Equal, "=".into(), 2),
            Token::new(TokenType::LeftParen, "(".into(), 2),
            Token::new(TokenType::Identifier("min".into()), "min".into(), 2),
            Token::new(TokenType::Plus, "+".into(), 2),
            Token::new(TokenType::Identifier("max".into()), "max".into(), 2),
            Token::new(TokenType::RightParen, ")".into(), 2),
            Token::new(TokenType::Slash, "/".into(), 2),
            Token::new(TokenType::Number(2.0), "2".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::EOF, "".into(), 3),
        ]
    );
}

#[test]
fn test_block() {
    let source = "
    {
      print \"One statement.\";
      print \"Two statements.\";
    }";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::LeftBrace, "{".into(), 2),
            Token::new(TokenType::Print, "print".into(), 3),
            Token::new(
                TokenType::String("One statement.".into()),
                "\"One statement.\"".into(),
                3
            ),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::Print, "print".into(), 4),
            Token::new(
                TokenType::String("Two statements.".into()),
                "\"Two statements.\"".into(),
                4
            ),
            Token::new(TokenType::Semicolon, ";".into(), 4),
            Token::new(TokenType::RightBrace, "}".into(), 5),
            Token::new(TokenType::EOF, "".into(), 5),
        ]
    );
}

#[test]
fn test_variables() {
    let source = "
    var imAVariable = \"here is my value\";
    var iAmNil;

    var breakfast = \"bagels\";
    print breakfast; // \"bagels\".
    breakfast = \"beignets\";
    print breakfast; // \"beignets\".
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Var, "var".into(), 2),
            Token::new(
                TokenType::Identifier("imAVariable".into()),
                "imAVariable".into(),
                2
            ),
            Token::new(TokenType::Equal, "=".into(), 2),
            Token::new(
                TokenType::String("here is my value".into()),
                "\"here is my value\"".into(),
                2
            ),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::Var, "var".into(), 3),
            Token::new(TokenType::Identifier("iAmNil".into()), "iAmNil".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::Var, "var".into(), 5),
            Token::new(
                TokenType::Identifier("breakfast".into()),
                "breakfast".into(),
                5
            ),
            Token::new(TokenType::Equal, "=".into(), 5),
            Token::new(TokenType::String("bagels".into()), "\"bagels\"".into(), 5),
            Token::new(TokenType::Semicolon, ";".into(), 5),
            Token::new(TokenType::Print, "print".into(), 6),
            Token::new(
                TokenType::Identifier("breakfast".into()),
                "breakfast".into(),
                6
            ),
            Token::new(TokenType::Semicolon, ";".into(), 6),
            Token::new(
                TokenType::Identifier("breakfast".into()),
                "breakfast".into(),
                7
            ),
            Token::new(TokenType::Equal, "=".into(), 7),
            Token::new(
                TokenType::String("beignets".into()),
                "\"beignets\"".into(),
                7
            ),
            Token::new(TokenType::Semicolon, ";".into(), 7),
            Token::new(TokenType::Print, "print".into(), 8),
            Token::new(
                TokenType::Identifier("breakfast".into()),
                "breakfast".into(),
                8
            ),
            Token::new(TokenType::Semicolon, ";".into(), 8),
            Token::new(TokenType::EOF, "".into(), 9),
        ]
    );
}

#[test]
fn test_if_else() {
    let source = "
    if (condition) {
      print \"yes\";
    } else {
      print \"no\";
    }
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::If, "if".into(), 2),
            Token::new(TokenType::LeftParen, "(".into(), 2),
            Token::new(
                TokenType::Identifier("condition".into()),
                "condition".into(),
                2
            ),
            Token::new(TokenType::RightParen, ")".into(), 2),
            Token::new(TokenType::LeftBrace, "{".into(), 2),
            Token::new(TokenType::Print, "print".into(), 3),
            Token::new(TokenType::String("yes".into()), "\"yes\"".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::RightBrace, "}".into(), 4),
            Token::new(TokenType::Else, "else".into(), 4),
            Token::new(TokenType::LeftBrace, "{".into(), 4),
            Token::new(TokenType::Print, "print".into(), 5),
            Token::new(TokenType::String("no".into()), "\"no\"".into(), 5),
            Token::new(TokenType::Semicolon, ";".into(), 5),
            Token::new(TokenType::RightBrace, "}".into(), 6),
            Token::new(TokenType::EOF, "".into(), 7),
        ]
    );
}

#[test]
fn test_while() {
    let source = "
    var a = 1;
    while (a < 10) {
      print a;
      a = a + 1;
    }
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Var, "var".into(), 2),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 2),
            Token::new(TokenType::Equal, "=".into(), 2),
            Token::new(TokenType::Number(1.0), "1".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::While, "while".into(), 3),
            Token::new(TokenType::LeftParen, "(".into(), 3),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 3),
            Token::new(TokenType::Less, "<".into(), 3),
            Token::new(TokenType::Number(10.0), "10".into(), 3),
            Token::new(TokenType::RightParen, ")".into(), 3),
            Token::new(TokenType::LeftBrace, "{".into(), 3),
            Token::new(TokenType::Print, "print".into(), 4),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 4),
            Token::new(TokenType::Semicolon, ";".into(), 4),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 5),
            Token::new(TokenType::Equal, "=".into(), 5),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 5),
            Token::new(TokenType::Plus, "+".into(), 5),
            Token::new(TokenType::Number(1.0), "1".into(), 5),
            Token::new(TokenType::Semicolon, ";".into(), 5),
            Token::new(TokenType::RightBrace, "}".into(), 6),
            Token::new(TokenType::EOF, "".into(), 7),
        ]
    );
}

#[test]
fn test_for() {
    let source = "
    for (var a = 1; a < 10; a = a + 1) {
        print a;
    }
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::For, "for".into(), 2),
            Token::new(TokenType::LeftParen, "(".into(), 2),
            Token::new(TokenType::Var, "var".into(), 2),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 2),
            Token::new(TokenType::Equal, "=".into(), 2),
            Token::new(TokenType::Number(1.0), "1".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 2),
            Token::new(TokenType::Less, "<".into(), 2),
            Token::new(TokenType::Number(10.0), "10".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 2),
            Token::new(TokenType::Equal, "=".into(), 2),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 2),
            Token::new(TokenType::Plus, "+".into(), 2),
            Token::new(TokenType::Number(1.0), "1".into(), 2),
            Token::new(TokenType::RightParen, ")".into(), 2),
            Token::new(TokenType::LeftBrace, "{".into(), 2),
            Token::new(TokenType::Print, "print".into(), 3),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::RightBrace, "}".into(), 4),
            Token::new(TokenType::EOF, "".into(), 5),
        ]
    );
}

#[test]
fn test_functions() {
    let source = "
    makeBreakfast(bacon, eggs, toast);
    
    makeBreakfast();

    fun printSum(a, b) {
      print a + b;
    }

    fun returnSum(a, b) {
      return a + b;
    }";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(
                TokenType::Identifier("makeBreakfast".into()),
                "makeBreakfast".into(),
                2
            ),
            Token::new(TokenType::LeftParen, "(".into(), 2),
            Token::new(TokenType::Identifier("bacon".into()), "bacon".into(), 2),
            Token::new(TokenType::Comma, ",".into(), 2),
            Token::new(TokenType::Identifier("eggs".into()), "eggs".into(), 2),
            Token::new(TokenType::Comma, ",".into(), 2),
            Token::new(TokenType::Identifier("toast".into()), "toast".into(), 2),
            Token::new(TokenType::RightParen, ")".into(), 2),
            Token::new(TokenType::Semicolon, ";".into(), 2),
            Token::new(
                TokenType::Identifier("makeBreakfast".into()),
                "makeBreakfast".into(),
                4
            ),
            Token::new(TokenType::LeftParen, "(".into(), 4),
            Token::new(TokenType::RightParen, ")".into(), 4),
            Token::new(TokenType::Semicolon, ";".into(), 4),
            Token::new(TokenType::Fun, "fun".into(), 6),
            Token::new(
                TokenType::Identifier("printSum".into()),
                "printSum".into(),
                6
            ),
            Token::new(TokenType::LeftParen, "(".into(), 6),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 6),
            Token::new(TokenType::Comma, ",".into(), 6),
            Token::new(TokenType::Identifier("b".into()), "b".into(), 6),
            Token::new(TokenType::RightParen, ")".into(), 6),
            Token::new(TokenType::LeftBrace, "{".into(), 6),
            Token::new(TokenType::Print, "print".into(), 7),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 7),
            Token::new(TokenType::Plus, "+".into(), 7),
            Token::new(TokenType::Identifier("b".into()), "b".into(), 7),
            Token::new(TokenType::Semicolon, ";".into(), 7),
            Token::new(TokenType::RightBrace, "}".into(), 8),
            Token::new(TokenType::Fun, "fun".into(), 10),
            Token::new(
                TokenType::Identifier("returnSum".into()),
                "returnSum".into(),
                10
            ),
            Token::new(TokenType::LeftParen, "(".into(), 10),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 10),
            Token::new(TokenType::Comma, ",".into(), 10),
            Token::new(TokenType::Identifier("b".into()), "b".into(), 10),
            Token::new(TokenType::RightParen, ")".into(), 10),
            Token::new(TokenType::LeftBrace, "{".into(), 10),
            Token::new(TokenType::Return, "return".into(), 11),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 11),
            Token::new(TokenType::Plus, "+".into(), 11),
            Token::new(TokenType::Identifier("b".into()), "b".into(), 11),
            Token::new(TokenType::Semicolon, ";".into(), 11),
            Token::new(TokenType::RightBrace, "}".into(), 12),
            Token::new(TokenType::EOF, "".into(), 12),
        ]
    );
}

#[test]
fn test_closures() {
    let source = "
    fun addPair(a, b) {
      return a + b;
    }
    
    fun identity(a) {
      return a;
    }
    
    print identity(addPair)(1, 2); // Prints \"3\".
     
    fun outerFunction() {
      fun localFunction() {
        print \"I'm local!\";
      }
    
      localFunction();
    }

    fun returnFunction() {
      var outside = \"outside\";
    
      fun inner() {
        print outside;
      }
    
      return inner;
    }
    
    var fn = returnFunction();
    fn();
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Fun, "fun".into(), 2),
            Token::new(TokenType::Identifier("addPair".into()), "addPair".into(), 2),
            Token::new(TokenType::LeftParen, "(".into(), 2),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 2),
            Token::new(TokenType::Comma, ",".into(), 2),
            Token::new(TokenType::Identifier("b".into()), "b".into(), 2),
            Token::new(TokenType::RightParen, ")".into(), 2),
            Token::new(TokenType::LeftBrace, "{".into(), 2),
            Token::new(TokenType::Return, "return".into(), 3),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 3),
            Token::new(TokenType::Plus, "+".into(), 3),
            Token::new(TokenType::Identifier("b".into()), "b".into(), 3),
            Token::new(TokenType::Semicolon, ";".into(), 3),
            Token::new(TokenType::RightBrace, "}".into(), 4),
            Token::new(TokenType::Fun, "fun".into(), 6),
            Token::new(
                TokenType::Identifier("identity".into()),
                "identity".into(),
                6
            ),
            Token::new(TokenType::LeftParen, "(".into(), 6),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 6),
            Token::new(TokenType::RightParen, ")".into(), 6),
            Token::new(TokenType::LeftBrace, "{".into(), 6),
            Token::new(TokenType::Return, "return".into(), 7),
            Token::new(TokenType::Identifier("a".into()), "a".into(), 7),
            Token::new(TokenType::Semicolon, ";".into(), 7),
            Token::new(TokenType::RightBrace, "}".into(), 8),
            Token::new(TokenType::Print, "print".into(), 10),
            Token::new(
                TokenType::Identifier("identity".into()),
                "identity".into(),
                10
            ),
            Token::new(TokenType::LeftParen, "(".into(), 10),
            Token::new(
                TokenType::Identifier("addPair".into()),
                "addPair".into(),
                10
            ),
            Token::new(TokenType::RightParen, ")".into(), 10),
            Token::new(TokenType::LeftParen, "(".into(), 10),
            Token::new(TokenType::Number(1.0), "1".into(), 10),
            Token::new(TokenType::Comma, ",".into(), 10),
            Token::new(TokenType::Number(2.0), "2".into(), 10),
            Token::new(TokenType::RightParen, ")".into(), 10),
            Token::new(TokenType::Semicolon, ";".into(), 10),
            Token::new(TokenType::Fun, "fun".into(), 12),
            Token::new(
                TokenType::Identifier("outerFunction".into()),
                "outerFunction".into(),
                12
            ),
            Token::new(TokenType::LeftParen, "(".into(), 12),
            Token::new(TokenType::RightParen, ")".into(), 12),
            Token::new(TokenType::LeftBrace, "{".into(), 12),
            Token::new(TokenType::Fun, "fun".into(), 13),
            Token::new(
                TokenType::Identifier("localFunction".into()),
                "localFunction".into(),
                13
            ),
            Token::new(TokenType::LeftParen, "(".into(), 13),
            Token::new(TokenType::RightParen, ")".into(), 13),
            Token::new(TokenType::LeftBrace, "{".into(), 13),
            Token::new(TokenType::Print, "print".into(), 14),
            Token::new(
                TokenType::String("I'm local!".into()),
                "\"I'm local!\"".into(),
                14
            ),
            Token::new(TokenType::Semicolon, ";".into(), 14),
            Token::new(TokenType::RightBrace, "}".into(), 15),
            Token::new(
                TokenType::Identifier("localFunction".into()),
                "localFunction".into(),
                17
            ),
            Token::new(TokenType::LeftParen, "(".into(), 17),
            Token::new(TokenType::RightParen, ")".into(), 17),
            Token::new(TokenType::Semicolon, ";".into(), 17),
            Token::new(TokenType::RightBrace, "}".into(), 18),
            Token::new(TokenType::Fun, "fun".into(), 20),
            Token::new(
                TokenType::Identifier("returnFunction".into()),
                "returnFunction".into(),
                20
            ),
            Token::new(TokenType::LeftParen, "(".into(), 20),
            Token::new(TokenType::RightParen, ")".into(), 20),
            Token::new(TokenType::LeftBrace, "{".into(), 20),
            Token::new(TokenType::Var, "var".into(), 21),
            Token::new(
                TokenType::Identifier("outside".into()),
                "outside".into(),
                21
            ),
            Token::new(TokenType::Equal, "=".into(), 21),
            Token::new(
                TokenType::String("outside".into()),
                "\"outside\"".into(),
                21
            ),
            Token::new(TokenType::Semicolon, ";".into(), 21),
            Token::new(TokenType::Fun, "fun".into(), 23),
            Token::new(TokenType::Identifier("inner".into()), "inner".into(), 23),
            Token::new(TokenType::LeftParen, "(".into(), 23),
            Token::new(TokenType::RightParen, ")".into(), 23),
            Token::new(TokenType::LeftBrace, "{".into(), 23),
            Token::new(TokenType::Print, "print".into(), 24),
            Token::new(
                TokenType::Identifier("outside".into()),
                "outside".into(),
                24
            ),
            Token::new(TokenType::Semicolon, ";".into(), 24),
            Token::new(TokenType::RightBrace, "}".into(), 25),
            Token::new(TokenType::Return, "return".into(), 27),
            Token::new(TokenType::Identifier("inner".into()), "inner".into(), 27),
            Token::new(TokenType::Semicolon, ";".into(), 27),
            Token::new(TokenType::RightBrace, "}".into(), 28),
            Token::new(TokenType::Var, "var".into(), 30),
            Token::new(TokenType::Identifier("fn".into()), "fn".into(), 30),
            Token::new(TokenType::Equal, "=".into(), 30),
            Token::new(
                TokenType::Identifier("returnFunction".into()),
                "returnFunction".into(),
                30
            ),
            Token::new(TokenType::LeftParen, "(".into(), 30),
            Token::new(TokenType::RightParen, ")".into(), 30),
            Token::new(TokenType::Semicolon, ";".into(), 30),
            Token::new(TokenType::Identifier("fn".into()), "fn".into(), 31),
            Token::new(TokenType::LeftParen, "(".into(), 31),
            Token::new(TokenType::RightParen, ")".into(), 31),
            Token::new(TokenType::Semicolon, ";".into(), 31),
            Token::new(TokenType::EOF, "".into(), 32),
        ]
    );
}

#[test]
fn test_class() {
    let source = "
    class Breakfast {
      cook() {
        print \"Eggs a-fryin'!\";
      }
    
      serve(who) {
        print \"Enjoy your breakfast, \" + who + \".\";
      }
    }

    // Store it in variables.
    var someVariable = Breakfast;
    
    // Pass it to functions.
    someFunction(Breakfast);

    var breakfast = Breakfast();
    print breakfast; // \"Breakfast instance\".
    
    breakfast.meat = \"sausage\";
    breakfast.bread = \"sourdough\";

    class Breakfast {
      serve(who) {
        print \"Enjoy your \" + this.meat + \" and \" +
            this.bread + \", \" + who + \".\";
      }
    
      // ...
    }

    class Breakfast {
      init(meat, bread) {
        this.meat = meat;
        this.bread = bread;
      }
    
      // ...
    }
    
    var baconAndToast = Breakfast(\"bacon\", \"toast\");
    baconAndToast.serve(\"Dear Reader\");
    // \"Enjoy your bacon and toast, Dear Reader.\"
    ";

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Class, "class".into(), 2),
            Token::new(
                TokenType::Identifier("Breakfast".into()),
                "Breakfast".into(),
                2
            ),
            Token::new(TokenType::LeftBrace, "{".into(), 2),
            Token::new(TokenType::Identifier("cook".into()), "cook".into(), 3),
            Token::new(TokenType::LeftParen, "(".into(), 3),
            Token::new(TokenType::RightParen, ")".into(), 3),
            Token::new(TokenType::LeftBrace, "{".into(), 3),
            Token::new(TokenType::Print, "print".into(), 4),
            Token::new(
                TokenType::String("Eggs a-fryin'!".into()),
                "\"Eggs a-fryin'!\"".into(),
                4
            ),
            Token::new(TokenType::Semicolon, ";".into(), 4),
            Token::new(TokenType::RightBrace, "}".into(), 5),
            Token::new(TokenType::Identifier("serve".into()), "serve".into(), 7),
            Token::new(TokenType::LeftParen, "(".into(), 7),
            Token::new(TokenType::Identifier("who".into()), "who".into(), 7),
            Token::new(TokenType::RightParen, ")".into(), 7),
            Token::new(TokenType::LeftBrace, "{".into(), 7),
            Token::new(TokenType::Print, "print".into(), 8),
            Token::new(
                TokenType::String("Enjoy your breakfast, ".into()),
                "\"Enjoy your breakfast, \"".into(),
                8
            ),
            Token::new(TokenType::Plus, "+".into(), 8),
            Token::new(TokenType::Identifier("who".into()), "who".into(), 8),
            Token::new(TokenType::Plus, "+".into(), 8),
            Token::new(TokenType::String(".".into()), "\".\"".into(), 8),
            Token::new(TokenType::Semicolon, ";".into(), 8),
            Token::new(TokenType::RightBrace, "}".into(), 9),
            Token::new(TokenType::RightBrace, "}".into(), 10),
            Token::new(TokenType::Var, "var".into(), 13),
            Token::new(
                TokenType::Identifier("someVariable".into()),
                "someVariable".into(),
                13
            ),
            Token::new(TokenType::Equal, "=".into(), 13),
            Token::new(
                TokenType::Identifier("Breakfast".into()),
                "Breakfast".into(),
                13
            ),
            Token::new(TokenType::Semicolon, ";".into(), 13),
            Token::new(
                TokenType::Identifier("someFunction".into()),
                "someFunction".into(),
                16
            ),
            Token::new(TokenType::LeftParen, "(".into(), 16),
            Token::new(
                TokenType::Identifier("Breakfast".into()),
                "Breakfast".into(),
                16
            ),
            Token::new(TokenType::RightParen, ")".into(), 16),
            Token::new(TokenType::Semicolon, ";".into(), 16),
            Token::new(TokenType::Var, "var".into(), 18),
            Token::new(
                TokenType::Identifier("breakfast".into()),
                "breakfast".into(),
                18
            ),
            Token::new(TokenType::Equal, "=".into(), 18),
            Token::new(
                TokenType::Identifier("Breakfast".into()),
                "Breakfast".into(),
                18
            ),
            Token::new(TokenType::LeftParen, "(".into(), 18),
            Token::new(TokenType::RightParen, ")".into(), 18),
            Token::new(TokenType::Semicolon, ";".into(), 18),
            Token::new(TokenType::Print, "print".into(), 19),
            Token::new(
                TokenType::Identifier("breakfast".into()),
                "breakfast".into(),
                19
            ),
            Token::new(TokenType::Semicolon, ";".into(), 19),
            Token::new(
                TokenType::Identifier("breakfast".into()),
                "breakfast".into(),
                21
            ),
            Token::new(TokenType::Dot, ".".into(), 21),
            Token::new(TokenType::Identifier("meat".into()), "meat".into(), 21),
            Token::new(TokenType::Equal, "=".into(), 21),
            Token::new(
                TokenType::String("sausage".into()),
                "\"sausage\"".into(),
                21
            ),
            Token::new(TokenType::Semicolon, ";".into(), 21),
            Token::new(
                TokenType::Identifier("breakfast".into()),
                "breakfast".into(),
                22
            ),
            Token::new(TokenType::Dot, ".".into(), 22),
            Token::new(TokenType::Identifier("bread".into()), "bread".into(), 22),
            Token::new(TokenType::Equal, "=".into(), 22),
            Token::new(
                TokenType::String("sourdough".into()),
                "\"sourdough\"".into(),
                22
            ),
            Token::new(TokenType::Semicolon, ";".into(), 22),
            Token::new(TokenType::Class, "class".into(), 24),
            Token::new(
                TokenType::Identifier("Breakfast".into()),
                "Breakfast".into(),
                24
            ),
            Token::new(TokenType::LeftBrace, "{".into(), 24),
            Token::new(TokenType::Identifier("serve".into()), "serve".into(), 25),
            Token::new(TokenType::LeftParen, "(".into(), 25),
            Token::new(TokenType::Identifier("who".into()), "who".into(), 25),
            Token::new(TokenType::RightParen, ")".into(), 25),
            Token::new(TokenType::LeftBrace, "{".into(), 25),
            Token::new(TokenType::Print, "print".into(), 26),
            Token::new(
                TokenType::String("Enjoy your ".into()),
                "\"Enjoy your \"".into(),
                26
            ),
            Token::new(TokenType::Plus, "+".into(), 26),
            Token::new(TokenType::This, "this".into(), 26),
            Token::new(TokenType::Dot, ".".into(), 26),
            Token::new(TokenType::Identifier("meat".into()), "meat".into(), 26),
            Token::new(TokenType::Plus, "+".into(), 26),
            Token::new(TokenType::String(" and ".into()), "\" and \"".into(), 26),
            Token::new(TokenType::Plus, "+".into(), 26),
            Token::new(TokenType::This, "this".into(), 27),
            Token::new(TokenType::Dot, ".".into(), 27),
            Token::new(TokenType::Identifier("bread".into()), "bread".into(), 27),
            Token::new(TokenType::Plus, "+".into(), 27),
            Token::new(TokenType::String(", ".into()), "\", \"".into(), 27),
            Token::new(TokenType::Plus, "+".into(), 27),
            Token::new(TokenType::Identifier("who".into()), "who".into(), 27),
            Token::new(TokenType::Plus, "+".into(), 27),
            Token::new(TokenType::String(".".into()), "\".\"".into(), 27),
            Token::new(TokenType::Semicolon, ";".into(), 27),
            Token::new(TokenType::RightBrace, "}".into(), 28),
            Token::new(TokenType::RightBrace, "}".into(), 31),
            Token::new(TokenType::Class, "class".into(), 33),
            Token::new(
                TokenType::Identifier("Breakfast".into()),
                "Breakfast".into(),
                33
            ),
            Token::new(TokenType::LeftBrace, "{".into(), 33),
            Token::new(TokenType::Identifier("init".into()), "init".into(), 34),
            Token::new(TokenType::LeftParen, "(".into(), 34),
            Token::new(TokenType::Identifier("meat".into()), "meat".into(), 34),
            Token::new(TokenType::Comma, ",".into(), 34),
            Token::new(TokenType::Identifier("bread".into()), "bread".into(), 34),
            Token::new(TokenType::RightParen, ")".into(), 34),
            Token::new(TokenType::LeftBrace, "{".into(), 34),
            Token::new(TokenType::This, "this".into(), 35),
            Token::new(TokenType::Dot, ".".into(), 35),
            Token::new(TokenType::Identifier("meat".into()), "meat".into(), 35),
            Token::new(TokenType::Equal, "=".into(), 35),
            Token::new(TokenType::Identifier("meat".into()), "meat".into(), 35),
            Token::new(TokenType::Semicolon, ";".into(), 35),
            Token::new(TokenType::This, "this".into(), 36),
            Token::new(TokenType::Dot, ".".into(), 36),
            Token::new(TokenType::Identifier("bread".into()), "bread".into(), 36),
            Token::new(TokenType::Equal, "=".into(), 36),
            Token::new(TokenType::Identifier("bread".into()), "bread".into(), 36),
            Token::new(TokenType::Semicolon, ";".into(), 36),
            Token::new(TokenType::RightBrace, "}".into(), 37),
            Token::new(TokenType::RightBrace, "}".into(), 40),
            Token::new(TokenType::Var, "var".into(), 42),
            Token::new(
                TokenType::Identifier("baconAndToast".into()),
                "baconAndToast".into(),
                42
            ),
            Token::new(TokenType::Equal, "=".into(), 42),
            Token::new(
                TokenType::Identifier("Breakfast".into()),
                "Breakfast".into(),
                42
            ),
            Token::new(TokenType::LeftParen, "(".into(), 42),
            Token::new(TokenType::String("bacon".into()), "\"bacon\"".into(), 42),
            Token::new(TokenType::Comma, ",".into(), 42),
            Token::new(TokenType::String("toast".into()), "\"toast\"".into(), 42),
            Token::new(TokenType::RightParen, ")".into(), 42),
            Token::new(TokenType::Semicolon, ";".into(), 42),
            Token::new(
                TokenType::Identifier("baconAndToast".into()),
                "baconAndToast".into(),
                43
            ),
            Token::new(TokenType::Dot, ".".into(), 43),
            Token::new(TokenType::Identifier("serve".into()), "serve".into(), 43),
            Token::new(TokenType::LeftParen, "(".into(), 43),
            Token::new(
                TokenType::String("Dear Reader".into()),
                "\"Dear Reader\"".into(),
                43
            ),
            Token::new(TokenType::RightParen, ")".into(), 43),
            Token::new(TokenType::Semicolon, ";".into(), 43),
            Token::new(TokenType::EOF, "".into(), 45),
        ]
    );
}

#[test]
fn test_inheritance() {
    let source = "
    class Brunch < Breakfast {
      drink() {
        print \"How about a Bloody Mary?\";
      }
    }

    var benedict = Brunch(\"ham\", \"English muffin\");
    benedict.serve(\"Noble Reader\");

    class Brunch < Breakfast {
      init(meat, bread, drink) {
        super.init(meat, bread);
        this.drink = drink;
      }
    }
    ";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenType::Class, "class".into(), 2),
            Token::new(TokenType::Identifier("Brunch".into()), "Brunch".into(), 2),
            Token::new(TokenType::Less, "<".into(), 2),
            Token::new(
                TokenType::Identifier("Breakfast".into()),
                "Breakfast".into(),
                2
            ),
            Token::new(TokenType::LeftBrace, "{".into(), 2),
            Token::new(TokenType::Identifier("drink".into()), "drink".into(), 3),
            Token::new(TokenType::LeftParen, "(".into(), 3),
            Token::new(TokenType::RightParen, ")".into(), 3),
            Token::new(TokenType::LeftBrace, "{".into(), 3),
            Token::new(TokenType::Print, "print".into(), 4),
            Token::new(
                TokenType::String("How about a Bloody Mary?".into()),
                "\"How about a Bloody Mary?\"".into(),
                4
            ),
            Token::new(TokenType::Semicolon, ";".into(), 4),
            Token::new(TokenType::RightBrace, "}".into(), 5),
            Token::new(TokenType::RightBrace, "}".into(), 6),
            Token::new(TokenType::Var, "var".into(), 8),
            Token::new(
                TokenType::Identifier("benedict".into()),
                "benedict".into(),
                8
            ),
            Token::new(TokenType::Equal, "=".into(), 8),
            Token::new(TokenType::Identifier("Brunch".into()), "Brunch".into(), 8),
            Token::new(TokenType::LeftParen, "(".into(), 8),
            Token::new(TokenType::String("ham".into()), "\"ham\"".into(), 8),
            Token::new(TokenType::Comma, ",".into(), 8),
            Token::new(
                TokenType::String("English muffin".into()),
                "\"English muffin\"".into(),
                8
            ),
            Token::new(TokenType::RightParen, ")".into(), 8),
            Token::new(TokenType::Semicolon, ";".into(), 8),
            Token::new(
                TokenType::Identifier("benedict".into()),
                "benedict".into(),
                9
            ),
            Token::new(TokenType::Dot, ".".into(), 9),
            Token::new(TokenType::Identifier("serve".into()), "serve".into(), 9),
            Token::new(TokenType::LeftParen, "(".into(), 9),
            Token::new(
                TokenType::String("Noble Reader".into()),
                "\"Noble Reader\"".into(),
                9
            ),
            Token::new(TokenType::RightParen, ")".into(), 9),
            Token::new(TokenType::Semicolon, ";".into(), 9),
            Token::new(TokenType::Class, "class".into(), 11),
            Token::new(TokenType::Identifier("Brunch".into()), "Brunch".into(), 11),
            Token::new(TokenType::Less, "<".into(), 11),
            Token::new(
                TokenType::Identifier("Breakfast".into()),
                "Breakfast".into(),
                11
            ),
            Token::new(TokenType::LeftBrace, "{".into(), 11),
            Token::new(TokenType::Identifier("init".into()), "init".into(), 12),
            Token::new(TokenType::LeftParen, "(".into(), 12),
            Token::new(TokenType::Identifier("meat".into()), "meat".into(), 12),
            Token::new(TokenType::Comma, ",".into(), 12),
            Token::new(TokenType::Identifier("bread".into()), "bread".into(), 12),
            Token::new(TokenType::Comma, ",".into(), 12),
            Token::new(TokenType::Identifier("drink".into()), "drink".into(), 12),
            Token::new(TokenType::RightParen, ")".into(), 12),
            Token::new(TokenType::LeftBrace, "{".into(), 12),
            Token::new(TokenType::Super, "super".into(), 13),
            Token::new(TokenType::Dot, ".".into(), 13),
            Token::new(TokenType::Identifier("init".into()), "init".into(), 13),
            Token::new(TokenType::LeftParen, "(".into(), 13),
            Token::new(TokenType::Identifier("meat".into()), "meat".into(), 13),
            Token::new(TokenType::Comma, ",".into(), 13),
            Token::new(TokenType::Identifier("bread".into()), "bread".into(), 13),
            Token::new(TokenType::RightParen, ")".into(), 13),
            Token::new(TokenType::Semicolon, ";".into(), 13),
            Token::new(TokenType::This, "this".into(), 14),
            Token::new(TokenType::Dot, ".".into(), 14),
            Token::new(TokenType::Identifier("drink".into()), "drink".into(), 14),
            Token::new(TokenType::Equal, "=".into(), 14),
            Token::new(TokenType::Identifier("drink".into()), "drink".into(), 14),
            Token::new(TokenType::Semicolon, ";".into(), 14),
            Token::new(TokenType::RightBrace, "}".into(), 15),
            Token::new(TokenType::RightBrace, "}".into(), 16),
            Token::new(TokenType::EOF, "".into(), 17),
        ]
    );
}
