use std::f32::consts::E;

#[derive(PartialEq, Eq, Debug)]
enum PostgresObject {
    Table(String, String),
    View(String, String),
}

#[derive(PartialEq, Eq, Debug)]
enum PostgresTk {
    SELECT,
    FROM,
    COMMA,
    IDENTIFIER(String),
}

#[derive(Debug)]
enum LexErr {
    InvalidToken,
}
impl PostgresTk {
    pub fn new(acc: Vec<char>) -> Result<Self, LexErr> {
        let str = String::from_iter(acc);
        match str.as_str() {
            "select" => Ok(PostgresTk::SELECT),
            "from" => Ok(PostgresTk::FROM),
            ident => Ok(PostgresTk::IDENTIFIER(ident.to_string())),
        }
    }
}
struct LexSQL<I: Iterator<Item = (u32, char)>> {
    chars: I,
    curr_char: Option<char>,
    next_char: Option<char>,
    curr_loc: u32,
    next_loc: u32,
}

impl<I> LexSQL<I>
where
    I: Iterator<Item = (u32, char)>,
{
    pub fn new(content: I) -> Self {
        let mut lexer = LexSQL {
            chars: content,
            curr_char: None,
            next_char: None,
            curr_loc: 0,
            next_loc: 0,
        };
        let _ = lexer.get_next_char();
        let _ = lexer.get_next_char();
        return lexer;
    }

    pub fn get_next_char(&mut self) -> Option<char> {
        let next_char = match self.chars.next() {
            None => {
                self.curr_loc = self.next_loc;
                None
            }
            Some((loc, ch)) => {
                self.curr_loc = self.next_loc;
                self.next_loc = loc;
                Some(ch)
            }
        };
        self.curr_char = self.next_char;
        self.next_char = next_char;
        self.next_char
    }

    pub fn tokenize(&mut self) -> Result<Vec<PostgresTk>, LexErr> {
        let mut pg_tokens = Vec::new();
        let mut acc: Vec<char> = Vec::new();
        while let Some(ch) = self.curr_char {
            match ch {
                ',' => {
                    // Skip
                    pg_tokens.push(PostgresTk::new(acc)?);
                    pg_tokens.push(PostgresTk::COMMA);
                    acc = Vec::new();
                }
                ' ' | ';' => {
                    if !acc.is_empty() {
                        pg_tokens.push(PostgresTk::new(acc)?);
                        acc = Vec::new();
                    }
                    if ch == ';' {
                        break;
                    }
                }
                ch => acc.push(ch),
            }
            let _ = self.get_next_char();
        }
        return Ok(pg_tokens);
    }
}

#[test]
fn test_make_lexer() {
    let input = String::from("SELECT * FROM schema.table;").to_lowercase();
    let with_indices = input.char_indices().map(|(i, c)| (i as u32, c));
    let lexer = LexSQL::new(with_indices);
    assert_eq!(lexer.curr_char, Some('s'));
    assert_eq!(lexer.next_char, Some('e'));
    assert_eq!(lexer.curr_loc, 0);
    assert_eq!(lexer.next_loc, 1);
}

#[test]
fn test_tokenize() {
    let input = String::from("SELECT * FROM schema.table;").to_lowercase();
    let with_indices = input.char_indices().map(|(i, c)| (i as u32, c));
    let mut lexer = LexSQL::new(with_indices);
    let tks = lexer.tokenize();
    assert!(tks.is_ok());
    let tks_ = tks.expect("");
    let answers = vec![
        PostgresTk::SELECT,
        PostgresTk::IDENTIFIER("*".to_string()),
        PostgresTk::FROM,
        PostgresTk::IDENTIFIER("schema.table".to_string()),
    ];
    for (actual, expected) in tks_.iter().zip(answers.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_column_selectors() {
    let input = String::from("SELECT first_col, second_col FROM schema.table;").to_lowercase();
    let with_indices = input.char_indices().map(|(i, c)| (i as u32, c));
    let mut lexer = LexSQL::new(with_indices);
    let tks = lexer.tokenize();
    assert!(tks.is_ok());
    let tks_ = tks.expect("");
    let answers = vec![
        PostgresTk::SELECT,
        PostgresTk::IDENTIFIER("first_col".to_string()),
        PostgresTk::COMMA,
        PostgresTk::IDENTIFIER("second_col".to_string()),
        PostgresTk::FROM,
        PostgresTk::IDENTIFIER("schema.table".to_string()),
    ];

    for (found, expected) in tks_.iter().zip(answers.iter()) {
        assert_eq!(found, expected);
    }
}

struct ParseSQL {}

fn main() {}
