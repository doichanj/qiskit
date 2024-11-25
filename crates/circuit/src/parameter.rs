// This code is part of Qiskit.
//
// (C) Copyright IBM 2024
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

// ParameterExpression class using symengine C wrapper interface

use std::convert::From;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg};
use std::collections::HashMap;

use crate::symbol_expr::{SymbolExpr, Value};
use crate::symbol_parser::parse_expression;

use num_complex::Complex64;

#[derive(Debug)]
pub struct Parameter {
    pub expr_: SymbolExpr,
}

impl Parameter {
    pub fn default() -> Self {
        Self {
            expr_: SymbolExpr::Value( Value::Real(0.0)),
        }
    }

    pub fn new(expr: &str) -> Self {
        Self {
            expr_: parse_expression(expr),
        }
    }

    pub fn to_string(&self) -> String {
        self.expr_.to_string()
    }

    pub fn num_symbols(&self) -> usize {
        self.expr_.symbols().len()
    }

    pub fn symbols(&self) -> Vec<String> {
        let mut symbols: Vec<String> = self
            .expr_.symbols()
            .iter()
            .map(|key| key.to_string())
            .collect();
        symbols.sort();
        symbols
    }
    pub fn has_symbol(&self, symbol: String) -> bool {
        self.expr_.symbols().contains(&symbol)
    }

    pub fn bind(&mut self, param: String, value: f64) {
        let map = HashMap::<String, f64>::from([(param, value),]);
        self.expr_.subs(&map);
    }
    pub fn subs(&mut self, maps: &HashMap<String, f64>) {
        self.expr_.subs(&maps);
    }

    pub fn real(&self) -> f64 {
        self.expr_.real()
    }
    pub fn imag(&self) -> f64 {
        self.expr_.imag()
    }
    pub fn complex(&self) -> Complex64 {
        self.expr_.complex()
    }

    pub fn is_complex(&self) -> bool {
        self.expr_.is_complex()
    }
    pub fn is_real(&self) -> bool {
        self.expr_.is_real()
    }

    fn add_expr(self, rhs: Self) -> Self {
        Self {
            expr_: self.expr_ + rhs.expr_,
        }
    }

    fn add_assign_expr(&mut self, rhs: Self) {
        self.expr_ = self.expr_.clone() + rhs.expr_;
    }

    fn sub_expr(self, rhs: Self) -> Self {
        Self {
            expr_: self.expr_ - rhs.expr_,
        }
    }

    fn sub_assign_expr(&mut self, rhs: Self) {
        self.expr_ = self.expr_.clone() - rhs.expr_;
    }

    fn mul_expr(self, rhs: Self) -> Self {
        Self {
            expr_: self.expr_ * rhs.expr_,
        }
    }

    fn mul_assign_expr(&mut self, rhs: Self) {
        self.expr_ = self.expr_.clone() * rhs.expr_;
    }

    fn div_expr(self, rhs: Self) -> Self {
        Self {
            expr_: self.expr_ / rhs.expr_,
        }
    }

    fn div_assign_expr(&mut self, rhs: Self) {
        self.expr_ = self.expr_.clone() / rhs.expr_;
    }

    pub fn sin(self) -> Self {
        Self {
            expr_: self.expr_.sin(),
        } 
    }

    pub fn cos(self) -> Self {
        Self {
            expr_: self.expr_.cos(),
        } 
    }

    pub fn tan(self) -> Self {
        Self {
            expr_: self.expr_.tan(),
        } 
    }

    pub fn arcsin(self) -> Self {
        Self {
            expr_: self.expr_.asin(),
        } 
    }

    pub fn arccos(self) -> Self {
        Self {
            expr_: self.expr_.acos(),
        } 
    }

    pub fn arctan(self) -> Self {
        Self {
            expr_: self.expr_.atan(),
        } 
    }

    pub fn exp(self) -> Self {
        Self {
            expr_: self.expr_.exp(),
        } 
    }

    pub fn log(self) -> Self {
        Self {
            expr_: self.expr_.log(),
        } 
    }

    pub fn abs(self) -> Self {
        Self {
            expr_: self.expr_.abs(),
        } 
    }

    pub fn pow<T: Into<Self>>(self, prm: T) -> Self {
        Self {expr_: self.expr_.pow(prm.into().expr_),}
    }
}

impl Clone for Parameter {
    fn clone(&self) -> Self {
        Self {
            expr_: self.expr_.clone()
        }
    }
}

impl PartialEq for Parameter {
    fn eq(&self, rprm: &Self) -> bool {
        self.expr_ == rprm.expr_
    }
}


// =============================
// Make from Rust native types
// =============================

impl From<i32> for Parameter {
    fn from(v: i32) -> Self {
        Self {
            expr_: SymbolExpr::Value( Value::Real(v as f64)),
        }
    }
}
impl From<i64> for Parameter {
    fn from(v: i64) -> Self {
        Self {
            expr_: SymbolExpr::Value( Value::Real(v as f64)),
        }
    }
}

impl From<u32> for Parameter {
    fn from(v: u32) -> Self {
        Self {
            expr_: SymbolExpr::Value( Value::Real(v as f64)),
        }
    }
}

impl From<f64> for Parameter {
    fn from(v: f64) -> Self {
        Self {
            expr_: SymbolExpr::Value( Value::Real(v)),
        }
    }
}

impl From<Complex64> for Parameter {
    fn from(v: Complex64) -> Self {
        Self {
            expr_: SymbolExpr::Value( Value::Complex(v)),
        }
    }
}

impl From<&str> for Parameter {
    fn from(v: &str) -> Self {
        Self::new(v)
    }
}

// =============================
// Unary operations
// =============================
impl Neg for Parameter {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            expr_: -self.expr_,
        }
    }
}

// =============================
// Add
// =============================

macro_rules! add_impl_expr {
    ($($t:ty)*) => ($(
        impl Add<$t> for Parameter {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn add(self, other: $t) -> Self::Output {
                self.add_expr(other.into())
            }
        }

        impl AddAssign<$t> for Parameter {
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, other: $t) {
                self.add_assign_expr(other.into())
            }
        }
    )*)
}

add_impl_expr! {f64 i32 u32 Parameter}

// =============================
// Sub
// =============================

macro_rules! sub_impl_expr {
    ($($t:ty)*) => ($(
        impl Sub<$t> for Parameter {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn sub(self, other: $t) -> Self::Output {
                self.sub_expr(other.into())
            }
        }

        impl SubAssign<$t> for Parameter {
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, other: $t) {
                self.sub_assign_expr(other.into())
            }
        }
    )*)
}

sub_impl_expr! {f64 i32 u32 Parameter}

// =============================
// Mul
// =============================

macro_rules! mul_impl_expr {
    ($($t:ty)*) => ($(
        impl Mul<$t> for Parameter {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn mul(self, other: $t) -> Self::Output {
                self.mul_expr(other.into())
            }
        }

        impl MulAssign<$t> for Parameter {
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, other: $t) {
                self.mul_assign_expr(other.into())
            }
        }
    )*)
}

mul_impl_expr! {f64 i32 u32 Parameter}

// =============================
// Div
// =============================

macro_rules! div_impl_expr {
    ($($t:ty)*) => ($(
        impl Div<$t> for Parameter {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn div(self, other: $t) -> Self::Output {
                self.div_expr(other.into())
            }
        }

        impl DivAssign<$t> for Parameter {
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, other: $t) {
                self.div_assign_expr(other.into())
            }
        }
    )*)
}

div_impl_expr! {f64 i32 u32 Parameter}

#[cfg(test)]
mod tests {
    use super::*;

    //  add
    #[test]
    fn test_add_x_y() {
        // x + y = x + y
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        assert_eq!(expr1.add_expr(expr2).to_string(), "x+y")
    }

    #[test]
    fn test_add_x_x() {
        //  x + x = 2*x
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.clone().add_expr(expr1).to_string(), "2*x")
    }

    #[test]
    fn test_add_x_y_z() {
        // (x + y) + z = x + (y + z)
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        let expr3 = Parameter::new("z");

        assert_eq!(
            expr1
                .clone()
                .add_expr(expr2.clone())
                .add_expr(expr3.clone())
                .to_string(),
            expr1.add_expr(expr2.add_expr(expr3)).to_string()
        );
    }

    #[test]
    fn test_add_ops() {
        // x + y = x + y
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        assert_eq!((expr1 + expr2).to_string(), "x+y")
    }

    #[test]
    fn test_add_x_f64() {
        // 1.0 + x = 1.0 + x
        let expr1 = Parameter::new("x");
        let expr2: f64 = 1.0;
        assert_eq!((expr1 + expr2).to_string(), "1+x")
    }

    #[test]
    fn test_add_x_u32() {
        // 1 + x = 1 + x
        let expr1 = Parameter::new("x");
        let expr2: u32 = 1;
        assert_eq!((expr1 + expr2).to_string(), "1+x")
    }

    // add_assign

    #[test]
    fn test_add_assign_x_y() {
        // x += y => x + y
        let mut expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        expr1.add_assign_expr(expr2);
        assert_eq!(expr1.to_string(), "x+y")
    }

    #[test]
    fn test_add_assign_x_x() {
        //  x +=x => 2*x
        let mut expr1 = Parameter::new("x");
        let expr2 = Parameter::new("x");
        expr1.add_assign_expr(expr2);
        assert_eq!(expr1.to_string(), "2*x")
    }

    #[test]
    fn test_add_assign_ops() {
        // x += y => x + y
        let mut expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        expr1 += expr2;
        assert_eq!(expr1.to_string(), "x+y")
    }

    #[test]
    fn test_add_expr_x_f64() {
        // x += 1.0 => 1.0 + x
        let mut expr1 = Parameter::new("x");
        let expr2: f64 = 1.0;
        expr1 += expr2;
        assert_eq!(expr1.to_string(), "1+x")
    }

    #[test]
    fn test_add_assign_x_u32() {
        // x += 1 => 1 + x
        let mut expr1 = Parameter::new("x");
        let expr2: u32 = 1;
        expr1 += expr2;
        assert_eq!(expr1.to_string(), "1+x")
    }

    //  sub

    #[test]
    fn test_sub_x_y() {
        // x - y = x - y
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        assert_eq!(expr1.sub_expr(expr2).to_string(), "x-y")
    }

    #[test]
    fn test_sub_x_x() {
        //  x - x = 0
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.clone().sub_expr(expr1).to_string(), "0")
    }

    #[test]
    fn test_sub_commutative_x_y() {
        // x - y != y - x
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        assert_ne!(
            expr1.clone().sub_expr(expr2.clone()).to_string(),
            expr2.sub_expr(expr1).to_string()
        );
    }

    #[test]
    fn test_sub_ops() {
        // x - y = x - y
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        assert_eq!((expr1 - expr2).to_string(), "x-y")
    }

    #[test]
    fn test_sub_x_f64() {
        // x - 1.0 = x - 1.0
        let expr1 = Parameter::new("x");
        let expr2: f64 = 1.0;
        assert_eq!((expr1 - expr2).to_string(), "x-1")
    }

    #[test]
    fn test_sub_x_u32() {
        // x - 1 = x - 1
        let expr1 = Parameter::new("x");
        let expr2: u32 = 1;
        assert_eq!((expr1 - expr2).to_string(), "x-1")
    }

    // sub_assign

    #[test]
    fn test_sub_assign_x_y() {
        // x -= y => x - y
        let mut expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        expr1.sub_assign_expr(expr2);
        assert_eq!(expr1.to_string(), "x-y")
    }

    #[test]
    fn test_sub_assign_x_x() {
        //  x -=x => 2*x
        let mut expr1 = Parameter::new("x");
        expr1.sub_assign_expr(expr1.clone());
        assert_eq!(expr1.to_string(), "0")
    }

    #[test]
    fn test_sub_assign_ops() {
        // x -= y => x - y
        let mut expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        expr1 -= expr2;
        assert_eq!(expr1.to_string(), "x-y")
    }

    #[test]
    fn test_sub_expr_x_f64() {
        // x -= 1.0 => -1.0 + x
        let mut expr1 = Parameter::new("x");
        let expr2: f64 = 1.0;
        expr1 -= expr2;
        assert_eq!(expr1.to_string(), "x-1")
    }

    #[test]
    fn test_sub_assign_x_u32() {
        // x -= 1 => -1 + x
        let mut expr1 = Parameter::new("x");
        let expr2: u32 = 1;
        expr1 -= expr2;
        assert_eq!(expr1.to_string(), "x-1")
    }

    //  mul
    #[test]
    fn test_mul_x_y() {
        // x * y = x*y
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        assert_eq!(expr1.mul_expr(expr2).to_string(), "x*y")
    }

    #[test]
    fn test_mul_x_x() {
        //  x * x = x**2
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.clone().mul_expr(expr1).to_string(), "x*x")
    }

    #[test]
    fn test_mul_x_y_z() {
        // (x * y) * z = x * (y * z)
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        let expr3 = Parameter::new("z");

        assert_eq!(
            expr1
                .clone()
                .mul_expr(expr2.clone())
                .mul_expr(expr3.clone())
                .to_string(),
            expr1.mul_expr(expr2.mul_expr(expr3)).to_string()
        );
    }

    #[test]
    fn test_mul_ops() {
        // x * y = x*y
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        assert_eq!((expr1 * expr2).to_string(), "x*y")
    }

    #[test]
    fn test_mul_x_f64() {
        // 2.0 * x = 2*x
        let expr1 = Parameter::new("x");
        let expr2: f64 = 2.0;
        assert_eq!((expr1 * expr2).to_string(), "2*x")
    }

    #[test]
    fn test_mul_x_f64_identity() {
        // 1.0 * x = x
        let expr1 = Parameter::new("x");
        let expr2: f64 = 1.0;
        assert_eq!((expr1 * expr2).to_string(), "x")
    }

    #[test]
    fn test_mul_x_f64_zero() {
        // 0.0 + x = 0.0
        let expr1 = Parameter::new("x");
        let expr2: f64 = 0.0;
        assert_eq!((expr1 * expr2).to_string(), "0")
    }

    #[test]
    fn test_mul_x_u32() {
        // 2 * x = 2*x
        let expr1 = Parameter::new("x");
        let expr2: u32 = 2;
        assert_eq!((expr1 * expr2).to_string(), "2*x")
    }

    #[test]
    fn test_mul_x_u32_identity() {
        // 1 * x = x
        let expr1 = Parameter::new("x");
        let expr2: u32 = 1;
        assert_eq!((expr1 * expr2).to_string(), "x")
    }

    #[test]
    fn test_mul_x_u32_zero() {
        // 0 * x = 0
        let expr1 = Parameter::new("x");
        let expr2: u32 = 0;
        assert_eq!((expr1 * expr2).to_string(), "0")
    }

    // mul assign

    #[test]
    fn test_mul_assign_x_y() {
        // x *= y => x*y
        let mut expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        expr1 *= expr2;
        assert_eq!(expr1.to_string(), "x*y")
    }

    #[test]
    fn test_mul_assign_x_x() {
        //  x *= x => x**2
        let mut expr1 = Parameter::new("x");
        expr1 *= expr1.clone();
        assert_eq!(expr1.to_string(), "x*x")
    }

    #[test]
    fn test_mul_assign_x_f64() {
        // x *= 2.0 => 2.0*x
        let mut expr1 = Parameter::new("x");
        let expr2: f64 = 2.0;
        expr1 *= expr2;
        assert_eq!(expr1.to_string(), "2*x")
    }

    #[test]
    fn test_mul_assign_x_f64_identity() {
        // x *= 1.0 => 1.0*x
        let mut expr1 = Parameter::new("x");
        let expr2: f64 = 1.0;
        expr1 *= expr2;
        assert_eq!(expr1.to_string(), "x")
    }

    #[test]
    fn test_mul_assign_x_f64_zero() {
        // x *= 0.0 => 0.0
        let mut expr1 = Parameter::new("x");
        let expr2: f64 = 0.0;
        expr1 *= expr2;
        assert_eq!(expr1.to_string(), "0")
    }

    #[test]
    fn test_mul_assign_x_u32() {
        // x *= 2 => 2*x
        let mut expr1 = Parameter::new("x");
        let expr2: u32 = 2;
        expr1 *= expr2;
        assert_eq!(expr1.to_string(), "2*x")
    }

    #[test]
    fn test_mul_assign_x_u32_identity() {
        // x *= 1 => x
        let mut expr1 = Parameter::new("x");
        let expr2: u32 = 1;
        expr1 *= expr2;
        assert_eq!(expr1.to_string(), "x")
    }

    #[test]
    fn test_mul_assign_x_u32_zero() {
        // x *= 0 => 0
        let mut expr1 = Parameter::new("x");
        let expr2: u32 = 0;
        expr1 *= expr2;
        assert_eq!(expr1.to_string(), "0")
    }

    //  div

    #[test]
    fn test_div_x_y() {
        // x / y = x/y
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        assert_eq!(expr1.div_expr(expr2).to_string(), "x/y")
    }

    #[test]
    fn test_div_x_x() {
        // x / x = 1
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.clone().div_expr(expr1).to_string(), "1")
    }

    #[test]
    fn test_div_x_one() {
        // x / 1 = x
        let expr1 = Parameter::new("x");
        let expr2: u32 = 1;
        assert_eq!((expr1 / expr2).to_string(), "x");
    }

    #[test]
    fn test_div_x_f64() {
        // x / 2.0 = 0.5*x
        let expr1 = Parameter::new("x");
        let expr2: f64 = 2.0;
        assert_eq!((expr1 / expr2).to_string(), "x/2");
    }

    #[test]
    fn test_div_x_u32() {
        // x / 2 = (1/2)*x
        let expr1 = Parameter::new("x");
        let expr2: u32 = 2;
        assert_eq!((expr1 / expr2).to_string(), "x/2");
    }

    // div_assign

    #[test]
    fn test_div_assign_x_y() {
        // x /= y => x/y
        let mut expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        expr1 /= expr2;
        assert_eq!(expr1.to_string(), "x/y")
    }

    #[test]
    fn test_div_assign_x_x() {
        // x / x = 1
        let mut expr1 = Parameter::new("x");
        expr1 /= expr1.clone();
        assert_eq!(expr1.to_string(), "1")
    }

    #[test]
    fn test_div_assign_x_one() {
        // x / 1 = x
        let mut expr1 = Parameter::new("x");
        let expr2: u32 = 1;
        expr1 /= expr2;
        assert_eq!(expr1.to_string(), "x");
    }

    #[test]
    fn test_div_assign_x_f64() {
        // x / 2.0 = 0.5*x
        let mut expr1 = Parameter::new("x");
        let expr2: f64 = 2.0;
        expr1 /= expr2;
        assert_eq!(expr1.to_string(), "x/2");
    }

    #[test]
    fn test_div_assign_x_u32() {
        // x / 2 = (1/2)*x
        let mut expr1 = Parameter::new("x");
        let expr2: u32 = 2;
        expr1 /= expr2;
        assert_eq!(expr1.to_string(), "x/2");
    }

    #[test]
    fn test_sin() {
        // x.sin() = sin(x)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.sin().to_string(), "sin(x)");
    }

    #[test]
    fn test_cos() {
        // x.cos() = cos(x)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.cos().to_string(), "cos(x)");
    }

    #[test]
    fn test_tan() {
        // x.tan() = tan(x)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.tan().to_string(), "tan(x)");
    }

    #[test]
    fn test_arcsin() {
        // x.arcsin() = arcsin(x)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.arcsin().to_string(), "asin(x)");
    }

    #[test]
    fn test_arccos() {
        // x.arccos() = arccos(x)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.arccos().to_string(), "acos(x)");
    }

    #[test]
    fn test_arctan() {
        // x.arctan() = arctan(x)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.arctan().to_string(), "atan(x)");
    }

    #[test]
    fn test_exp() {
        // x.exp() = exp(x)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.exp().to_string(), "exp(x)");
    }

    #[test]
    fn test_log() {
        // x.log() = log(x)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.log().to_string(), "log(x)");
    }

    #[test]
    fn test_abs() {
        // x.abs() = Abs(x)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.abs().to_string(), "abs(x)");
    }

    #[test]
    fn test_pow() {
        // x ** y = x**y
        let expr1 = Parameter::new("x");
        let expr2 = Parameter::new("y");
        assert_eq!(expr1.pow(expr2).to_string(), "x**y");
    }

    #[test]
    fn test_pow_f64() {
        // x ** 2.0 = x**2.0
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.pow(2.0).to_string(), "x**2");
    }

    #[test]
    fn test_pow_i32() {
        // x ** -2 = x**(-2)
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.pow(-2).to_string(), "x**(-2)");
    }

    #[test]
    fn test_pow_u32() {
        // x ** 2 = x**2
        let expr1 = Parameter::new("x");
        assert_eq!(expr1.pow(2).to_string(), "x**2");
    }
}
