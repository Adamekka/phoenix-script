/// Lexer for the language
#[derive(Clone, Debug)]
struct Lexer {
    text: String,
    position: usize,
    syntax_token: SyntaxToken,
}

#[derive(Clone, Debug)]
struct SyntaxToken {
    text: String,
    token_type: SyntaxTokenType,
}

#[derive(Clone, Debug, PartialEq)]
enum SyntaxTokenType {
    WhiteSpace,
    Number(std::result::Result<isize, std::num::ParseIntError>),
    Plus,
    Minus,
    Star,
    Slash,
    OpenParenthesis,
    CloseParenthesis,
    BadToken,
    EndOfFile,
}

impl Lexer {
    /// Get the current character in the text
    fn current(&self) -> char {
        if self.position >= self.text.len() {
            return '\0';
        }
        self.text.chars().nth(self.position).unwrap()
    }

    /// Get the next token in the text
    fn next_token(&mut self) {
        // Whitespace
        if self.current().is_whitespace() {
            let start: usize = self.position;

            while self.current().is_whitespace() {
                self.position += 1;
            }

            let length: usize = self.position - start;
            let text: &str = &self.text[start..start + length];

            self.syntax_token = SyntaxToken {
                text: text.to_string(),
                token_type: SyntaxTokenType::WhiteSpace,
            };

            return;
        // Number
        } else if self.current().is_numeric() {
            let start: usize = self.position;

            while self.current().is_numeric() {
                self.position += 1;
            }

            let length: usize = self.position - start;
            let text: &str = &self.text[start..start + length];
            let value: std::result::Result<isize, std::num::ParseIntError> = text.parse::<isize>();

            self.syntax_token = SyntaxToken {
                text: text.to_string(),
                token_type: SyntaxTokenType::Number(value),
            };
            return;
        }

        // Operators
        match self.current() {
            '+' => {
                self.position += 1;
                self.syntax_token = SyntaxToken {
                    text: "+".to_string(),
                    token_type: SyntaxTokenType::Plus,
                };
            }
            '-' => {
                self.position += 1;
                self.syntax_token = SyntaxToken {
                    text: "-".to_string(),
                    token_type: SyntaxTokenType::Minus,
                };
            }
            '*' => {
                self.position += 1;
                self.syntax_token = SyntaxToken {
                    text: "*".to_string(),
                    token_type: SyntaxTokenType::Star,
                };
            }
            '/' => {
                self.position += 1;
                self.syntax_token = SyntaxToken {
                    text: "/".to_string(),
                    token_type: SyntaxTokenType::Slash,
                };
            }
            '(' => {
                self.position += 1;
                self.syntax_token = SyntaxToken {
                    text: "(".to_string(),
                    token_type: SyntaxTokenType::OpenParenthesis,
                };
            }
            ')' => {
                self.position += 1;
                self.syntax_token = SyntaxToken {
                    text: ")".to_string(),
                    token_type: SyntaxTokenType::CloseParenthesis,
                };
            }
            '\0' => {
                self.position += 1;
                self.syntax_token = SyntaxToken {
                    text: "".to_string(),
                    token_type: SyntaxTokenType::EndOfFile,
                };
            }
            _ => {
                self.position += 1;
                self.syntax_token = SyntaxToken {
                    text: "".to_string(),
                    token_type: SyntaxTokenType::BadToken,
                };
            }
        }
    }
}

/// Parser for the language
#[derive(Debug)]
struct Parser {
    lexer: Lexer,
    position: usize,
    tokens: Vec<SyntaxToken>,
}

impl Parser {
    fn parse(&mut self) {
        loop {
            self.lexer.next_token();

            // Whitespace or bad token
            if self.lexer.syntax_token.token_type == SyntaxTokenType::WhiteSpace
                || self.lexer.syntax_token.token_type == SyntaxTokenType::BadToken
            {
                continue;
            // End of file
            } else if self.lexer.syntax_token.token_type == SyntaxTokenType::EndOfFile {
                break;
            } else {
                self.tokens.push(self.lexer.syntax_token.clone());
            }
        }
    }

    fn peek(&self, offset: usize) -> SyntaxToken {
        let index: usize = self.position + offset;

        if index >= self.tokens.len() {
            return SyntaxToken {
                text: "".to_string(),
                token_type: SyntaxTokenType::EndOfFile,
            };
        }

        self.tokens[index].clone()
    }

    fn current(&self) -> SyntaxToken {
        self.peek(0)
    }
}

#[derive(Clone, Debug)]
struct ExpressionSyntax {
    position: usize,
    left: ExpressionSyntaxEnum,
    operator_token: OperatorToken,
    right: ExpressionSyntaxEnum,
}

#[derive(Clone, Debug)]
enum ExpressionSyntaxEnum {
    ExpressionSyntax(Box<ExpressionSyntax>),
    Number(isize),
}

#[derive(Clone, Debug)]
enum OperatorToken {
    Plus,
    Minus,
    Star,
    Slash,
}

impl ExpressionSyntax {
    fn parse(&mut self, parser: &mut Parser) {
        // Find open parenthesis
        self.position = parser
            .tokens
            .iter()
            .position(|x: &SyntaxToken| x.text == "(")
            .expect("Failed to find open parenthesis, expected '('");

        // Get left expression
        if let Some(value) = parser
            .tokens
            .iter()
            .filter_map(|token: &SyntaxToken| match &token.token_type {
                SyntaxTokenType::Number(value) => Some(value),
                _ => None,
            })
            .nth(self.position)
        {
            self.position += 2;
            self.left =
                ExpressionSyntaxEnum::Number(value.clone().expect("Failed to parse number"));
        }

        // Get operator
        self.operator_token = match parser.tokens[self.position].text.as_str() {
            "+" => OperatorToken::Plus,
            "-" => OperatorToken::Minus,
            "*" => OperatorToken::Star,
            "/" => OperatorToken::Slash,
            _ => panic!(
                "Invalid operator, expected '+', '-', '*', or '/', found '{}'",
                parser.tokens[self.position].text
            ),
        };

        self.position += 1;

        // Get right expression
        self.right = match &parser.tokens[self.position].token_type {
            SyntaxTokenType::Number(value) => {
                ExpressionSyntaxEnum::Number(value.clone().expect("Failed to parse number"))
            }
            _ => panic!(
                "Invalid number, expected number after operator, found '{}'",
                parser.tokens[self.position].text
            ),
        }
    }
}

pub fn build(args: clap::ArgMatches) {
    // Get file to build
    let file: &String;
    if let Some(arg_match) = args.subcommand_matches("build") {
        file = arg_match
            .get_one::<String>("file")
            .expect("Failed to get file");
    } else {
        unreachable!("Subcommand is required");
    }

    println!("Building {}", file);

    // Get file contents
    let file_contents: String = std::fs::read_to_string(file).expect("Failed to read file");

    let mut parser: Parser = Parser {
        position: 0,
        tokens: Vec::new(),
        lexer: Lexer {
            text: file_contents,
            position: 0,
            syntax_token: SyntaxToken {
                text: "".to_string(),
                token_type: SyntaxTokenType::BadToken,
            },
        },
    };

    let mut expression: ExpressionSyntax = ExpressionSyntax {
        position: 0,
        left: ExpressionSyntaxEnum::Number(0),
        operator_token: OperatorToken::Plus,
        right: ExpressionSyntaxEnum::Number(0),
    };

    parser.parse();
    dbg!(&parser);
    expression.parse(&mut parser);
    dbg!(&expression);
}
