//! Calculator.

use std::collections::HashMap;

use anyhow::*;
use etrace::*;

use super::syntax::{BinOp, Command, Expression};

/// Calculator's context.
#[derive(Debug, Default, Clone)]
pub struct Context {
    anonymous_counter: usize,
    variables: HashMap<String, f64>,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current anonymous variable counter.
    pub fn current_counter(&self) -> usize {
        self.anonymous_counter
    }

    /// Calculates the given expression. (We assume the absence of overflow.)
    pub fn calc_expression(&self, expression: &Expression) -> Result<f64> {
        match expression {
            Expression::Num(v) => Ok(*v),
            Expression::Variable(s) => self
                .variables
                .get(s)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("Undifined variable: {}", s)),
            Expression::BinOp { op, lhs, rhs } => match op {
                BinOp::Add => {
                    let expr_l = self.calc_expression(lhs)?;
                    let expr_r = self.calc_expression(rhs)?;
                    Ok(expr_l + expr_r)
                }
                BinOp::Subtract => {
                    let expr_l = self.calc_expression(lhs)?;
                    let expr_r = self.calc_expression(rhs)?;
                    Ok(expr_l - expr_r)
                }
                BinOp::Multiply => {
                    let expr_l = self.calc_expression(lhs)?;
                    let expr_r = self.calc_expression(rhs)?;
                    Ok(expr_l * expr_r)
                }
                BinOp::Divide => {
                    let expr_l = self.calc_expression(lhs)?;
                    let expr_r = self.calc_expression(rhs)?;
                    if expr_r != 0.0 {
                        Ok(expr_l / expr_r)
                    } else {
                        bail!("Division by zero")
                    }
                }
                BinOp::Power => {
                    let expr_l = self.calc_expression(lhs)?;
                    let expr_r = self.calc_expression(rhs)?;

                    Ok(expr_l.powf(expr_r))
                }
            },
        }
    }

    /// Calculates the given command. (We assume the absence of overflow.)
    ///
    /// If there is no variable lhs in the command (i.e. `command.variable = None`), its value
    /// should be stored at `$0`, `$1`, `$2`, ... respectively.
    ///
    /// # Example
    ///
    /// After calculating commad `3 + 5` => Context's variables = `{($0,8)}`
    ///
    /// After calculating commad `v = 3 - 2` => Context's variables = `{($0,8),(v,1))}`
    ///
    /// After calculating commad `3 ^ 2` => Context's variables = `{($0,8),(v,1),($1,9)}`
    pub fn calc_command(&mut self, command: &Command) -> Result<(String, f64)> {
        let (var, value) = match &command.variable {
            Some(s) => {
                let result = self.calc_expression(&command.expression)?;
                (s.clone(), result)
            }
            None => {
                let index = self.anonymous_counter;
                let s = format!("${}", index);
                self.anonymous_counter += 1;
                let result = self.calc_expression(&command.expression)?;
                (s, result)
            }
        };
        let _ = self.variables.insert(var.clone(), value);
        Ok((var, value))
    }
}
