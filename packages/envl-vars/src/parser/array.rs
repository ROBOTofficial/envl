use std::slice::Iter;

use crate::{
    misc::{
        token::{Token, Value},
        variable::VariableValue,
    },
    parser::{
        error::{ARRAY_CLOSED, COMMA_POSITION, COMMA_REQUIRED, SYNTAX_IN_ARRAY},
        Parser, ParserError,
    },
};

impl Parser {
    pub fn parse_array<'a>(
        &self,
        tokens: &mut Iter<'a, Token>,
    ) -> Result<VariableValue, ParserError> {
        let mut array_contents = Vec::new();
        let mut parser_error: Option<ParserError> = None;
        let mut comma_used = false;
        let mut array_closed = false;
        let mut last_position = None;

        'parse_array_loop: loop {
            if let Some(token) = tokens.next() {
                last_position = Some(token.position.clone());

                match &token.value {
                    Value::LeftSquareBracket => match self.parse_array(tokens) {
                        Ok(v) => {
                            if !array_contents.is_empty() && !comma_used {
                                parser_error = Some(ParserError {
                                    kind: COMMA_REQUIRED.kind,
                                    code: COMMA_REQUIRED.code,
                                    message: COMMA_REQUIRED.message.to_string(),
                                    position: token.position.clone(),
                                });
                                break 'parse_array_loop;
                            }
                            array_contents.push(v.clone());
                            comma_used = false;
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_array_loop;
                        }
                    },
                    Value::RightSquareBracket => {
                        array_closed = true;
                        break 'parse_array_loop;
                    }
                    Value::Comma => {
                        if comma_used {
                            parser_error = Some(ParserError {
                                kind: COMMA_POSITION.kind,
                                code: COMMA_POSITION.code,
                                message: COMMA_POSITION.message.to_string(),
                                position: token.position.clone(),
                            });
                            break 'parse_array_loop;
                        }
                        comma_used = true;
                    }
                    Value::LeftCurlyBracket => match self.parse_struct(tokens) {
                        Ok(value) => {
                            if !array_contents.is_empty() && !comma_used {
                                parser_error = Some(ParserError {
                                    kind: COMMA_REQUIRED.kind,
                                    code: COMMA_REQUIRED.code,
                                    message: COMMA_REQUIRED.message.to_string(),
                                    position: token.position.clone(),
                                });
                                break 'parse_array_loop;
                            }
                            array_contents.push(value.clone());
                            comma_used = false;
                        }
                        Err(err) => {
                            parser_error = Some(err);
                            break 'parse_array_loop;
                        }
                    },
                    Value::Ident(value) => {
                        let value = self.parse_value(value, &token.position);
                        match value {
                            Ok(v) => {
                                if !array_contents.is_empty() && !comma_used {
                                    parser_error = Some(ParserError {
                                        kind: COMMA_REQUIRED.kind,
                                        code: COMMA_REQUIRED.code,
                                        message: COMMA_REQUIRED.message.to_string(),
                                        position: token.position.clone(),
                                    });
                                    break 'parse_array_loop;
                                }
                                array_contents.push(v.clone());
                                comma_used = false;
                            }
                            Err(err) => {
                                parser_error = Some(err);
                                break 'parse_array_loop;
                            }
                        }
                    }
                    Value::Comment(_) => {}
                    _ => {
                        parser_error = Some(ParserError {
                            kind: SYNTAX_IN_ARRAY.kind,
                            code: SYNTAX_IN_ARRAY.code,
                            message: SYNTAX_IN_ARRAY.message.to_string(),
                            position: token.position.clone(),
                        });
                        break 'parse_array_loop;
                    }
                }
            } else {
                break 'parse_array_loop;
            }
        }

        if let Some(err) = parser_error {
            Err(err)
        } else {
            if let Some(position) = last_position {
                if !array_closed {
                    return Err(ParserError {
                        kind: ARRAY_CLOSED.kind,
                        code: ARRAY_CLOSED.code,
                        message: ARRAY_CLOSED.message.to_string(),
                        position,
                    });
                }
            }
            Ok(VariableValue::Array(array_contents))
        }
    }
}
