use std::path;
use std::io::{File};
use std::io::BufferedReader;
use std::vec::Vec;
use collections::HashMap;
use std::strbuf::StrBuf;
use std::result::Result;

#[deriving(Eq,Show)]
pub enum ConfigLineType {
    Whitespace(~str),
    Comment(~str),
    Tokens(Vec<StrBuf>)
}

#[deriving(Eq)]
pub enum ParseMode {
    ExpectToken,
    Token,
}


pub fn parse_config_line(line:~str) ->Result< ConfigLineType, ~str> {
    //! Parse one line of config file
    //! where line can be:
    //! 
    //! #comment
    //! key value
    //! key "value"
    //! key " \"valu\n e "
    //! command with many "parameters"
  
    let trimmed_line = line.trim();
    if trimmed_line.len() == 0 {
        return Ok(Whitespace(line.clone()))
    }
    if trimmed_line.chars().next().unwrap() == '#' {
        return Ok(Comment(line.clone()));
    }
    let mut state:ParseMode = ExpectToken;
    let mut token = StrBuf::new();
    let mut tokens = vec![];
    let mut quoted = false;

    for chr in trimmed_line.chars() {
        if state == ExpectToken {
            if chr == '"' {
                state = Token;
                quoted = true;
                token = StrBuf::new();
            } else if chr.is_whitespace() {
                //pass
            } else if chr != '\\' {
                token = StrBuf::new();
                token.push_char(chr);
                state = Token;
            } else { return Err(~"invalid line") }
        } else if state == Token {
            //TODO: handle escape sequences
            if chr == '"' {
                if !quoted {
                    return Err(~"quote opened unexpectedly")
                } else {
                    tokens.push(token.clone());
                    state = ExpectToken;
                }
            } else if chr.is_whitespace() {
                if quoted {
                    token.push_char(chr);
                } else {
                    state = ExpectToken;
                    tokens.push(token.clone());
                }
            } else {
                token.push_char(chr);
            }
        }
    }
    if state == Token && quoted {
        return Err(~"open quote")
    }
    if state == Token {
        tokens.push(token);
    }
    Ok(Tokens(tokens))
}


#[test]
fn test_parse_config_line(){
  let s = StrBuf::from_str;

  assert!( parse_config_line(~"") == Ok(Whitespace(~"")));
  assert!( parse_config_line(~"   ") == Ok(Whitespace(~"   ")));
  assert!( parse_config_line(~"#comment") == Ok(Comment(~"#comment")));
  assert!( parse_config_line(~"key") == Ok(Tokens( vec![s("key")] )));
  assert!( parse_config_line(~"key key1 key2") == Ok(Tokens( vec![s("key"), s("key1"), s("key2")] )));
  assert!( parse_config_line(~"\"foo\"") == Ok(Tokens( vec![s("foo")] )));
  assert!( parse_config_line(~"\"foo bar\"") == Ok(Tokens( vec![s("foo bar")] )));
  assert_eq!( parse_config_line(~"\"foo"), Err(~"open quote"));
}


pub fn parse_config_lines(lines: Vec<~str>) -> Result<HashMap<~str, ~str>, ~str> {
    let mut result:HashMap<~str, ~str> = HashMap::new();
    for (idx, line) in lines.iter().enumerate() {
        match parse_config_line(line.to_owned()) {
            Ok(r) => match r {
              Tokens(tokens) => if tokens.len() == 2 {
                result.insert(tokens.get(0).clone().into_owned(), tokens.get(1).clone().into_owned());
              } else {
                return Err(format!("error in line {}: to many tokens", idx));
              },
              _ => (), //ignore whitespace and comments
            },
            Err(e) => return Err(format!("error in line {}: {}", idx, e))
        }
    }
    Ok(result)
}


pub fn read_env_var_config(value:~str) -> Vec<~str> {
    //! read configuration stored in env variable
    let mut lines = vec![];
    //semicolon-separated list of options
    for l in value.split(';'){
        lines.push(l.to_owned());
    }

    lines
}


pub fn read_file_config(file_path:path::Path) -> Vec<~str> {
    //! obtain configuration stored in file
    let mut lines = vec![];
 
    //read config from file
    let mut file = match File::open(&file_path) {
        Ok(f) => BufferedReader::new(f),
        Err(e) => fail!("file error: {}", e)
    };
    for line in file.lines(){
        let l:~str = line.unwrap().clone();
        lines.push(l);
    }

    lines
}
