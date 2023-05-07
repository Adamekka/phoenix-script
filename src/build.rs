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

    let mut lexer: Lexer = Lexer {
        text: file_contents,
        position: 0,
        syntax_token: SyntaxToken {
            text: "".to_string(),
            token_type: SyntaxTokenType::BadToken,
        },
    };

    loop {
        lexer.next_token();

        if lexer.syntax_token.token_type == SyntaxTokenType::EndOfFile {
            break;
        } else {
            dbg!(&lexer);
        }
    }
}
