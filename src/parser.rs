use std::{slice::Iter, vec};

use crate::lexer::Token;

#[derive(Clone, Debug)]
pub struct Expression {
    pub term: Term,
    pub following: Vec<(TermOperator, Term)>,
}

#[derive(Clone, Debug)]
pub struct Term {
    pub factor: Factor,
    pub following: Vec<(FactorOperator, Factor)>,
}

#[derive(Clone, Debug)]
pub enum TermOperator {
    Plus,
    Minus,
}

#[derive(Clone, Debug)]
pub struct Factor {
    pub prefix: Option<TermOperator>,
    pub variant: FactorVariant,
}

#[derive(Clone, Debug)]
pub enum FactorVariant {
    Number(f64),
    Expression(Box<Expression>),
}

#[derive(Clone, Debug)]
pub enum FactorOperator {
    Mul,
    Div,
}

#[derive(Clone, Debug)]
pub enum Atom {
    Number(f64),
    Expression(Expression),
}

pub struct Parser<'parser> {
    tokens: Iter<'parser, Token>,
    current_token: Token,
    current_token_index: usize,
}

impl<'parser> Parser<'parser> {
    pub fn new(tokens: &'parser Vec<Token>) -> Parser<'parser> {
        Self {
            tokens: tokens.iter(),
            current_token: Token::EOF,
            current_token_index: 0,
        }
    }

    fn advance(&mut self) {
        self.current_token_index += 1;
        if let Some(next) = self.tokens.next() {
            self.current_token = next.clone()
        } else {
            self.current_token = Token::EOF;
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.advance();
        let expr = self.expression()?;
        if self.current_token != Token::EOF {
            return Err(format!("expected EOF: found {:?}", self.current_token));
        };
        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expression, String> {
        let term = self.term()?;
        let mut following: Vec<(TermOperator, Term)> = vec![];

        loop {
            let following_operator = match self.current_token {
                Token::Plus => TermOperator::Plus,
                Token::Minus => TermOperator::Minus,
                // Means that there are no useful other term
                _ => break,
            };
            self.advance();
            following.push((following_operator, self.term()?))
        }

        Ok(Expression { term, following })
    }
    fn term(&mut self) -> Result<Term, String> {
        let factor = self.factor()?;
        let mut following: Vec<(FactorOperator, Factor)> = vec![];

        loop {
            let following_operator = match self.current_token {
                Token::Mul => FactorOperator::Mul,
                Token::Div => FactorOperator::Div,
                _ => break,
            };

            self.advance();
            following.push((following_operator, self.factor()?))
        }

        Ok(Term { factor, following })
    }

    fn factor(&mut self) -> Result<Factor, String> {
        let operator = match self.current_token {
            Token::Plus => Some(TermOperator::Plus),
            Token::Minus => Some(TermOperator::Minus),
            _ => None,
        };

        // Only advance if an optional operator is specified
        if operator.is_some() {
            self.advance();
        }

        let atom = self.atom()?;

        Ok(match atom {
            Atom::Number(n) => Factor {
                prefix: operator,
                variant: FactorVariant::Number(n),
            },
            Atom::Expression(e) => Factor {
                prefix: operator,
                variant: FactorVariant::Expression(Box::new(e)),
            },
        })
    }

    fn atom(&mut self) -> Result<Atom, String> {
        if self.current_token == Token::LParen {
            self.advance();
            let expression = self.expression()?;
            if self.current_token != Token::RParen {
                return Err(format!("Expected `)`, found {:?}", self.current_token));
            }
            self.advance();
            return Ok(Atom::Expression(expression));
        };

        let num = match &self.current_token {
            Token::Number(n) => n.parse::<f64>().unwrap(),
            _ => return Err(format!("Expected number, found {:?}", self.current_token)),
        };
        self.advance();
        Ok(Atom::Number(num))
    }
}
