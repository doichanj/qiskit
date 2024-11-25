// This code is part of Qiskit.
//
// (C) Copyright IBM 2023, 2024
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

extern crate nom;
use nom::IResult;
use nom::Parser;
use nom::character::complete::{char, multispace0, alpha1, alphanumeric1};
use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, map_res, recognize};
use nom::branch::{alt, permutation};
use nom::sequence::{delimited, pair};
use nom::multi::{many0, many0_count};
use nom::number::complete::double;

use num_complex::c64;

use std::rc::Rc;
use std::cell::RefCell;
use crate::symbol_expr::{SymbolExpr, BinaryOps, Symbol, Value, Unary, UnaryOps};

#[derive(Clone)]
struct BinaryOpContainer {
    op: BinaryOps,
    expr: SymbolExpr,
}

impl BinaryOpContainer {
    fn accum(self, rhs: BinaryOpContainer) -> BinaryOpContainer {
        match rhs.op {
            BinaryOps::Add => BinaryOpContainer{op: rhs.op, expr: self.expr + rhs.expr,}, 
            BinaryOps::Sub => BinaryOpContainer{op: rhs.op, expr: self.expr - rhs.expr,}, 
            BinaryOps::Mul => BinaryOpContainer{op: rhs.op, expr: self.expr * rhs.expr,}, 
            BinaryOps::Div => BinaryOpContainer{op: rhs.op, expr: self.expr / rhs.expr,}, 
            BinaryOps::Pow => BinaryOpContainer{op: rhs.op, expr: self.expr.pow(rhs.expr),}, 
            _ => self.clone(),
        }
    }
}

fn parse_value(s: &str) -> IResult<&str, BinaryOpContainer> {
    map_res(
        double,
        |v| -> Result<BinaryOpContainer, &str> {
            Ok(BinaryOpContainer{op: BinaryOps::Nop, expr: SymbolExpr::Value( Value::Real(v))})
        }
    )(s)
}

fn parse_imaginary_value(s: &str) -> IResult<&str, BinaryOpContainer> {
    map_res(
        permutation((
            double,
            char('i'),
        )),
        |(v, _)| -> Result<BinaryOpContainer, &str> {
            Ok(BinaryOpContainer{op: BinaryOps::Nop, expr: SymbolExpr::Value( Value::Complex(c64(0.0, v)))})
        }
    )(s)
}


fn parse_symbol_string(s: &str) -> IResult<&str, &str> {
    recognize(
      pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"))))
      )
    ).parse(s)
}

fn parse_symbol(s: &str) -> IResult<&str, BinaryOpContainer> {
    map_res(
        parse_symbol_string,
        |v: &str| -> Result<BinaryOpContainer, &str> {
            Ok(BinaryOpContainer{op: BinaryOps::Nop, expr: SymbolExpr::Symbol( Symbol{name: v.to_string()})})
        }
    )(s)
}

fn parse_unary(s: &str) -> IResult<&str, BinaryOpContainer> {
    map_res(
        permutation((
            alphanumeric1,
            delimited(
                char('('),
                delimited(multispace0, parse_addsub, multispace0),
                char(')'),
            ),
        )),
        |(v, expr)| -> Result<BinaryOpContainer, &str> {
            let op = match v {
                "sin" => UnaryOps::Sin,
                "asin" => UnaryOps::Asin,
                "cos" => UnaryOps::Cos,
                "acos" => UnaryOps::Acos,
                "tan" => UnaryOps::Tan,
                "atan" => UnaryOps::Atan,
                "log" => UnaryOps::Log,
                "exp" => UnaryOps::Exp,
                &_ => return Err("unsupported unary operation found."),
            };
            Ok(BinaryOpContainer{op: BinaryOps::Nop, expr: SymbolExpr::Unary( Rc::new(RefCell::new(Unary{op: op, expr: expr.expr})))})
        }
    )(s)
}

fn parse_neg(s: &str) -> IResult<&str, BinaryOpContainer> {
    map_res(
        permutation((
            char('-'),
            alt((
                parse_imaginary_value,
                parse_value,
                parse_unary,
                parse_symbol,
                delimited(
                    char('('),
                    delimited(multispace0, parse_addsub, multispace0),
                    char(')'),
                ),
            )),
        )),
        |(_, expr)| -> Result<BinaryOpContainer, &str> {
            Ok(BinaryOpContainer{op: BinaryOps::Nop, expr: SymbolExpr::Unary( Rc::new(RefCell::new(Unary{op: UnaryOps::Neg, expr: expr.expr})))})
        }
    )(s)
}

fn parse_expr(s: &str) -> IResult<&str, BinaryOpContainer> {
    alt((
        parse_imaginary_value,
        parse_value,
        parse_neg,
        parse_unary,
        parse_symbol,
        delimited(
            char('('),
            delimited(multispace0, parse_addsub, multispace0),
            char(')'),
        ),
    ))(s)
}

// parse power
fn parse_pow(s: &str) -> IResult<&str, BinaryOpContainer> {
    map_res(
        permutation((
            parse_expr,
            many0(
                map_res(
                    permutation((
                        multispace0,
                        char('*'),
                        char('*'),
                        multispace0,
                        parse_expr,
                    )),
                    |(_, _, _, _, mut rhs)| -> Result<BinaryOpContainer, &str> {
                        rhs.op = BinaryOps::Pow;
                        Ok(rhs)
                    }
                )
            ),
        )),
        |(lhs, rvec)| -> Result<BinaryOpContainer, &str> {
            Ok(rvec.iter().fold(lhs, |acc, x| { acc.accum(x.clone())}))
        }
    )(s)
}

// parse mul and div
fn parse_muldiv(s: &str) -> IResult<&str, BinaryOpContainer> {
    map_res(
        permutation((
            parse_pow,
            many0(
                map_res(
                    permutation((
                        multispace0,
                        alt((char('*'), char('/'),)),
                        multispace0,
                        parse_pow,
                    )),
                    |(_, opr, _, mut rhs)| -> Result<BinaryOpContainer, &str> {
                        if opr == '*' {
                            rhs.op = BinaryOps::Mul;
                            Ok(rhs)
                        } else {
                            rhs.op = BinaryOps::Div;
                            Ok(rhs)
                        }
                    }
                )
            ),
        )),
        |(lhs, rvec)| -> Result<BinaryOpContainer, &str> {
            Ok(rvec.iter().fold(lhs, |acc, x| { acc.accum(x.clone())}))
        }
    )(s)
}

// parse add and sub
fn parse_addsub(s: &str) -> IResult<&str, BinaryOpContainer> {
    map_res(
        permutation((
            parse_muldiv,
            many0(
                map_res(
                    permutation((
                        multispace0,
                        alt((char('+'), char('-'),)),
                        multispace0,
                        parse_muldiv,
                    )),
                    |(_, opr, _, mut rhs)| -> Result<BinaryOpContainer, &str> {
                        if opr == '+' {
                            rhs.op = BinaryOps::Add;
                            Ok(rhs)
                        } else {
                            rhs.op = BinaryOps::Sub;
                            Ok(rhs)
                        }
                    }
                )
            ),
        )),
        |(lhs, rvec)| -> Result<BinaryOpContainer, &str> {
            Ok(rvec.iter().fold(lhs, |acc, x| { acc.accum(x.clone())}))
        }
    )(s)
}

pub fn parse_expression(s: &str) -> SymbolExpr {
    let mut parser = all_consuming(parse_addsub);
    parser(s).unwrap().1.expr
}


