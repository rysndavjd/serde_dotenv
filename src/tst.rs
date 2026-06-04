for (i, c) in val.char_indices() {
if c == '\\' && quote != QuoteState::SingleQuoted {
escaped = true;
continue;
            }
match quote {
QuoteState::Unquoted => {
if c == '\'' {
quote_pos = i;
quote = QuoteState::SingleQuoted;
continue;
                    } else if c == '"' {
quote_pos = i;
quote = QuoteState::DoubleQuoted;
continue;
                    }
                }
QuoteState::SingleQuoted => {
if c == '\'' {
debug_assert!(i < quote_pos);
quote_pos = 0;
quote = QuoteState::Unquoted;
continue;
                    }
                }
QuoteState::DoubleQuoted => {
if c == '"' {
debug_assert!(i < quote_pos);
quote_pos = 0;
quote = QuoteState::Unquoted;
continue;
                    }
                }
            }
if matches!(
c,
'|' | '&' | ';' | '<' | '>' | '(' | ')' | '`' | '"' | '\'' | '\\'
            ) && quote == QuoteState::Unquoted
            {
return Err(Error::UnescapedShellChar { char: c, index: i });
            }
value.push(c);
        }
if escaped {
return Err(Error::DanglingEscape);
        }
match quote {
QuoteState::SingleQuoted => {
return Err(Error::UnterminatedSingleQuote { index: quote_pos });
            }
QuoteState::DoubleQuoted => {
return Err(Error::UnterminatedDoubleQuote { index: quote_pos });
            }
_ => (),
        }