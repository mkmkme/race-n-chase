use std::collections::HashMap;

use file_decoder::Decoder;

#[macro_use]
extern crate log;

pub mod errors;
pub mod file_decoder;

use errors::FXTError;

#[derive(Debug)]
enum FXTToken {
    Key(String),
    Value(String),
    End,
}

#[derive(PartialEq, Eq)]
enum State {
    Key,
    Value,
    End,
}

struct Parser<T: Decoder> {
    state: State,
    decoder: T,
}

/// Parses a FXT file and returns a HashMap with the keys and values.
pub fn parse_fxt(filename: &str) -> Result<HashMap<String, String>, FXTError> {
    let decoder = file_decoder::FileDecoder::new(filename)?;
    parse_fxt_impl(decoder)
}

fn parse_fxt_impl<T: Decoder + Iterator<Item = char>>(
    decoder: T,
) -> Result<HashMap<String, String>, FXTError> {
    let mut ret = HashMap::new();
    let mut cur_key = None;
    let mut parser = Parser::new(decoder);
    loop {
        match parser.parse_token() {
            FXTToken::Key(key) => {
                if cur_key.is_some() {
                    warn!(
                        "Got key '{}' without value at position {}",
                        key,
                        parser.get_position()
                    );
                }
                cur_key = Some(FXTToken::Key(key));
            }
            FXTToken::Value(value) => {
                if let Some(FXTToken::Key(key)) = cur_key.take() {
                    ret.insert(key, value);
                } else {
                    warn!(
                        "Got value '{}' without key at position {}",
                        value,
                        parser.get_position()
                    );
                }
            }
            FXTToken::End => {
                if let Some(FXTToken::Key(key)) = cur_key.take() {
                    warn!(
                        "Got key without value at the end: {}, position {}",
                        key,
                        parser.get_position()
                    );
                }
                return Ok(ret);
            }
        }
    }
}

impl<T: Decoder + Iterator<Item = char>> Parser<T> {
    fn new(decoder: T) -> Self {
        Parser {
            state: State::End,
            decoder,
        }
    }

    fn get_position(&self) -> usize {
        self.decoder.position()
    }

    fn parse_token(&mut self) -> FXTToken {
        let mut internal = String::new();
        for c in self.decoder.by_ref() {
            match c {
                '[' => {
                    if self.state == State::Key {
                        panic!("Unexpected '[' at position {}", self.decoder.position());
                    }
                    self.state = State::Key;
                    if !internal.is_empty() {
                        return FXTToken::Value(internal);
                    }
                }
                ']' => {
                    if self.state == State::Value {
                        panic!("Unexpected ']' at position {}", self.decoder.position());
                    }
                    self.state = State::Value;
                    if !internal.is_empty() {
                        return FXTToken::Key(internal);
                    }
                }
                // Values can contain null bytes
                '\u{0}' => {
                    if self.state != State::Value {
                        panic!(
                            "Unexpected null byte at position {}",
                            self.decoder.position()
                        );
                    }
                    continue;
                }
                _ => {
                    internal.push(c);
                }
            }
        }
        FXTToken::End
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONST_STR: &str =
                "[1001]Hello, world!\u{0}[1002]Yes, it works!\u{0}[1003]One-word\u{0}[1004]tschüss\u{0}[][]";
    struct MockDecoder(usize);

    impl MockDecoder {
        fn new() -> MockDecoder {
            MockDecoder(0)
        }
    }

    impl Decoder for MockDecoder {
        fn position(&self) -> usize {
            self.0
        }
    }

    impl Iterator for MockDecoder {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            let pos = self.0;
            self.0 += 1;
            CONST_STR.chars().nth(pos)
        }
    }

    #[test]
    fn mock_works() {
        let decoder = MockDecoder::new();
        let first_six: String = decoder.take(6).collect();
        assert_eq!(first_six, "[1001]");
    }

    #[test]
    fn parsing_works() {
        let decoder = MockDecoder::new();
        let map = parse_fxt_impl(decoder).unwrap();
        assert_eq!(map.get("1001").unwrap(), "Hello, world!");
        assert_eq!(map.get("1002").unwrap(), "Yes, it works!");
        assert_eq!(map.get("1003").unwrap(), "One-word");
        assert_eq!(map.get("1004").unwrap(), "tschüss");
    }
}
