#![allow(deprecated)]

//! Parser.

use anyhow::{bail, Result};
use etrace::*;
use lazy_static::*;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;

use super::syntax::*;

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
mod inner {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "assignments/assignment04/syntax.pest"]
    pub(crate) struct SyntaxParser;
}

use inner::*;

/// Parses command.
///
/// ## Operator Associativty
///
/// For associativity of each operator, please follow [here](https://docs.rs/pest/latest/pest/prec_climber/struct.PrecClimber.html#examples).
///
/// e.g. `1+2+3` should be parsed into `(1+2)+3`, not `1+(2+3)` because the associativity of
/// plus("add" in our hw) operator is `Left`.
pub fn parse_command(line: &str) -> Result<Command> {
    let mut pairs = SyntaxParser::parse(Rule::command, line)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    let first = pairs
        .next()
        .ok_or_else(|| anyhow::anyhow!("Empty command"))?;

    let (variable, expression) = if first.as_rule() == Rule::var {
        let expr = pairs
            .next()
            .ok_or_else(|| anyhow::anyhow!("Expected expression"))?;
        (Some(first.as_str().to_string()), parse_expression(expr)?)
    } else {
        (None, parse_expression(first)?)
    };

    Ok(Command {
        variable,
        expression,
    })
}

lazy_static::lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::subtract, Assoc::Left) |
        Operator::new(Rule::add, Assoc::Left),
        Operator::new(Rule::divide, Assoc::Left) |
        Operator::new(Rule::multiply, Assoc::Left),


        Operator::new(Rule::power, Assoc::Right),
    ]);
}

fn parse_expression(pair: Pair<'_, Rule>) -> Result<Expression> {
    PREC_CLIMBER.climb(
        pair.into_inner(),
        |pair| match pair.as_rule() {
            Rule::num => Ok(Expression::Num(pair.as_str().parse()?)),
            Rule::var => Ok(Expression::Variable(pair.as_str().to_string())),
            Rule::expr => parse_expression(pair),
            _ => bail!("Unexpected rule: {:?}", pair.as_rule()),
        },
        |lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => BinOp::Add,
                Rule::subtract => BinOp::Subtract,
                Rule::multiply => BinOp::Multiply,
                Rule::divide => BinOp::Divide,
                Rule::power => BinOp::Power,
                _ => bail!("Unknown operator: {:?}", op.as_rule()),
            };
            Ok(Expression::BinOp {
                op,
                lhs: Box::new(lhs?),
                rhs: Box::new(rhs?),
            })
        },
    )
}
