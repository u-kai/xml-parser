use super::token::{Token, TokenType};
enum StateMachine {
    CharBlank,
    CharChar,
    StartStart,
    EndChar,
    StartChar,
    StartSlash,
}
