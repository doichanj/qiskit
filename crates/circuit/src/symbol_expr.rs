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

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub, Neg};
use std::convert::From;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum SymbolExpr {
    Symbol(Symbol),
    Value(Value),
    Unary(Rc<RefCell<Unary>>),
    Binary(Rc<RefCell<Binary>>),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name : String,
}

#[derive(Debug, Clone)]
pub struct Value {
    pub value : f64,
}

#[derive(Debug, Clone)]
pub enum UnaryOps {
    Abs,
    Neg,
    Sin,
    Asin,
    Cos,
    Acos,
    Tan,
    Atan,
    Exp,
    Log,
}

#[derive(Debug, Clone)]
pub enum BinaryOps {
    Nop,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug)]
pub struct Unary {
    pub op : UnaryOps,
    pub expr : SymbolExpr,
}

#[derive(Debug)]
pub struct Binary {
    pub op : BinaryOps,
    pub lhs : SymbolExpr,
    pub rhs : SymbolExpr,
}

impl Clone for SymbolExpr {
    fn clone(&self) -> Self {
        match self {
            SymbolExpr::Symbol(e) => SymbolExpr::Symbol(e.clone()),
            SymbolExpr::Value(e) => SymbolExpr::Value(e.clone()),
            SymbolExpr::Unary(e) => SymbolExpr::Unary( Rc::new(RefCell::new(e.borrow().clone())) ),
            SymbolExpr::Binary(e) => SymbolExpr::Binary( Rc::new(RefCell::new(e.borrow().clone())) ),
        }
    }
}

impl SymbolExpr {
    pub fn to_string(&self) -> String {
        match self {
            SymbolExpr::Symbol(e) => e.to_string(),
            SymbolExpr::Value(e) => e.to_string(),
            SymbolExpr::Unary(e) => e.borrow().to_string(),
            SymbolExpr::Binary(e) => e.borrow().to_string(),
        }
    }

    pub fn subs(&self, maps: &HashMap<String, f64>) -> SymbolExpr {
        match self {
            SymbolExpr::Symbol(e) => e.subs(maps),
            SymbolExpr::Value(e) => SymbolExpr::Value(e.clone()),
            SymbolExpr::Unary(e) => e.borrow().subs(maps),
            SymbolExpr::Binary(e) => e.borrow().subs(maps),
        }
    }

    pub fn eval(&self, recurse: bool) -> Option<f64> {
        match self {
            SymbolExpr::Symbol(_) => None,
            SymbolExpr::Value(e) => Some(e.value),
            SymbolExpr::Unary(e) => e.borrow().eval(recurse),
            SymbolExpr::Binary(e) => e.borrow().eval(recurse),
        }
    }

    pub fn symbols(&self) -> HashSet<String> {
        match self {
            SymbolExpr::Symbol(e) => HashSet::<String>::from([e.name.clone()]),
            SymbolExpr::Value(_) => HashSet::<String>::new(),
            SymbolExpr::Unary(e) => e.borrow().symbols(),
            SymbolExpr::Binary(e) => e.borrow().symbols(),
        }
    }

    pub fn abs(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: l.value.abs(),}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Abs, expr: self.clone()} ))),
        }
    }
    pub fn sin(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: l.value.sin(),}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Sin, expr: self.clone()} ))),
        }
    }
    pub fn asin(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: l.value.asin(),}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Asin, expr: self.clone()} ))),
        }
    }
    pub fn cos(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: l.value.cos(),}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Cos, expr: self.clone()} ))),
        }
    }
    pub fn acos(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: l.value.acos(),}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Acos, expr: self.clone()} ))),
        }
    }
    pub fn tan(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: l.value.tan(),}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Tan, expr: self.clone()} ))),
        }
    }
    pub fn atan(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: l.value.atan(),}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Atan, expr: self.clone()} ))),
        }
    }
    pub fn exp(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: l.value.exp(),}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Exp, expr: self.clone()} ))),
        }
    }
    pub fn log(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: l.value.ln(),}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Log, expr: self.clone()} ))),
        }
    }
    pub fn pow(self, rhs: SymbolExpr) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => match rhs {
                SymbolExpr::Value(r) => SymbolExpr::Value( Value{value: l.value.powf(r.value),}),
                _ => SymbolExpr::Binary( Rc::new(RefCell::new( Binary{ op: BinaryOps::Pow, lhs: SymbolExpr::Value(l.clone()), rhs: rhs.clone()})) ),
            },
            _ => SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Pow, lhs: self.clone(), rhs: rhs.clone()} ))),
        }
    }
}

impl Add for SymbolExpr {
    type Output = SymbolExpr;
    fn add(self, rhs: SymbolExpr) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => match rhs {
                SymbolExpr::Value(r) => SymbolExpr::Value( Value{value: l.value + r.value,}),
                _ => if l.value == 0.0 {
                    rhs.clone()
                }else{
                    SymbolExpr::Binary( Rc::new(RefCell::new( Binary{ op: BinaryOps::Add, lhs: SymbolExpr::Value(l.clone()), rhs: rhs.clone()})) )
                },
            },
            _ => match rhs {
                SymbolExpr::Value(r) => if r.value == 0.0 {
                    self.clone()
                }else{
                    if r.value < 0.0 {
                        SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Sub, lhs: self.clone(), rhs: SymbolExpr::Value( Value{value: -r.value})} )))
                    } else {
                        SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Add, lhs: self.clone(), rhs: SymbolExpr::Value(r.clone())} )))
                    }
                },
                _ => if self == rhs {
                    SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Mul, lhs: SymbolExpr::Value( Value{value: 2.0}), rhs: self.clone()} )))
                } else {
                    SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Add, lhs: self.clone(), rhs: rhs.clone()} )))
                },
            },
        }
    }
}

impl Sub for SymbolExpr {
    type Output = SymbolExpr;
    fn sub(self, rhs: SymbolExpr) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => match rhs {
                SymbolExpr::Value(r) => SymbolExpr::Value( Value{value: l.value - r.value,}),
                _ => if l.value == 0.0 {
                    rhs.neg()
                }else{
                    SymbolExpr::Binary( Rc::new(RefCell::new( Binary{ op: BinaryOps::Sub, lhs: SymbolExpr::Value(l.clone()), rhs: rhs.clone()})) )
                },
            },
            _ => match rhs {
                SymbolExpr::Value(r) => if r.value == 0.0 {
                    self.clone()
                }else{
                    if r.value < 0.0 {
                        SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Add, lhs: self.clone(), rhs: SymbolExpr::Value( Value{value: -r.value})} )))
                    } else {
                        SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Sub, lhs: self.clone(), rhs: SymbolExpr::Value(r.clone())} )))
                    }
                },
                _ => if self == rhs {
                    SymbolExpr::Value( Value{value: 0.0})
                } else {
                    SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Sub, lhs: self.clone(), rhs: rhs.clone()} )))
                },
            },
        }
    }
}

impl Mul for SymbolExpr {
    type Output = SymbolExpr;
    fn mul(self, rhs: SymbolExpr) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => match rhs {
                SymbolExpr::Value(r) => SymbolExpr::Value( Value{value: l.value * r.value,}),
                _ => if l.value == 0.0 {
                    SymbolExpr::Value( Value{value: 0.0})
                } else if l.value == 1.0 {
                    rhs.clone()
                } else if l.value == -1.0 {
                    rhs.neg()
                }else{
                    SymbolExpr::Binary( Rc::new(RefCell::new( Binary{ op: BinaryOps::Mul, lhs: SymbolExpr::Value(l.clone()), rhs: rhs.clone()})) )
                },
            },
            _ => match rhs {
                SymbolExpr::Value(r) => if r.value == 0.0 {
                    SymbolExpr::Value( Value{value: 0.0})
                } else if r.value == 1.0 {
                    self.clone()
                } else if r.value == -1.0 {
                    self.neg()
                }else{
                    if r.value < 0.0 {
                        SymbolExpr::Unary( Rc::new(RefCell::new(Unary { op: UnaryOps::Neg, expr: SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Mul, lhs: SymbolExpr::Value( Value{value: -r.value}), rhs: self.clone()} ))) } )))
                    } else {
                        SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Mul, lhs: SymbolExpr::Value(r.clone()), rhs: self.clone()} )))
                    }
                },
                _ => SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Mul, lhs: self.clone(), rhs: rhs.clone()} ))),
            },
        }
    }
}

impl Div for SymbolExpr {
    type Output = SymbolExpr;
    fn div(self, rhs: SymbolExpr) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => match rhs {
                SymbolExpr::Value(r) => SymbolExpr::Value( Value{value: l.value / r.value,}),
                _ => if l.value == 1.0 {
                    rhs.clone()
                } else if l.value == -1.0 {
                    rhs.neg()
                }else{
                    SymbolExpr::Binary( Rc::new(RefCell::new( Binary{ op: BinaryOps::Div, lhs: SymbolExpr::Value(l.clone()), rhs: rhs.clone()})) )
                },
            },
            _ => match rhs {
                SymbolExpr::Value(r) => if r.value == 1.0 {
                    self.clone()
                } else if r.value == -1.0 {
                    self.neg()
                }else{
                    if r.value < 0.0 {
                        SymbolExpr::Unary( Rc::new(RefCell::new(Unary { op: UnaryOps::Neg, expr: SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Div, lhs: self.clone(), rhs: SymbolExpr::Value( Value{value: -r.value})} ))) } )))
                    } else {
                        SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Div, lhs: self.clone(), rhs: SymbolExpr::Value(r.clone())} )))
                    }
                },
                _ => if self == rhs {
                    SymbolExpr::Value( Value{value: 1.0})
                } else {
                    SymbolExpr::Binary( Rc::new(RefCell::new(Binary{ op: BinaryOps::Div, lhs: self.clone(), rhs: rhs.clone()} )))
                },
            },
        }
    }
}

impl Neg for SymbolExpr {
    type Output = SymbolExpr;
    fn neg(self) -> SymbolExpr {
        match self {
            SymbolExpr::Value(l) => SymbolExpr::Value( Value{value: -l.value,}),
            _ => SymbolExpr::Unary( Rc::new(RefCell::new(Unary{ op: UnaryOps::Neg, expr: self.clone()} ))),
        }
    }
}

impl PartialEq for SymbolExpr {
    fn eq(&self, rexpr: &Self) -> bool {
        match self {
            SymbolExpr::Symbol(l) => match rexpr {
                SymbolExpr::Symbol(r) => l == r,
                _ => false,
            },
            SymbolExpr::Value(l) => match rexpr {
                SymbolExpr::Value(r) => l == r,
                _ => false,
            },
            SymbolExpr::Unary(l) => match rexpr {
                SymbolExpr::Unary(r) => l == r,
                _ => false,
            },
            SymbolExpr::Binary(l) => match rexpr {
                SymbolExpr::Binary(r) => l == r,
                _ => false,
            },
        }
    }
}


// ===============================================================
//  implementations for Symbol
// ===============================================================
impl Symbol {
    pub fn new(expr: &str) -> Self {
        Self { name: expr.to_string(), }
    }
    pub fn to_string(&self) -> String {
        self.name.clone()
    }

    pub fn subs(&self, maps: &HashMap<String, f64>) -> SymbolExpr {
        match maps.get(&self.name) {
            Some(v) => SymbolExpr::Value( Value{value: *v,}),
            None =>  SymbolExpr::Symbol(self.clone()),
        }
    }
}

impl From<&str> for Symbol {
    fn from(v: &str) -> Self {
        Self::new(v)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, r: &Self) -> bool {
        self.name == r.name
    }
}

// ===============================================================
//  implementations for Value
// ===============================================================
impl Value {
    pub fn default() -> Self {
        Self { value: 0.0, }
    }
    pub fn new(val: f64) -> Self {
        Self { value: val, }
    }
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::new(v)
    }
}

impl PartialEq for Value {
    fn eq(&self, r: &Self) -> bool {
        self.value == r.value
    }
}

// ===============================================================
//  implementations for Unary operators
// ===============================================================
impl Clone for Unary {
    fn clone(&self) -> Unary {
        Unary {
            op : self.op.clone(),
            expr : self.expr.clone(),
        }
    }
}

impl Unary {
    pub fn to_string(&self) -> String {
        let s = self.expr.to_string();
        match self.op {
            UnaryOps::Abs => String::from(format!("abs({})", s)),
            UnaryOps::Neg => match &self.expr {
                SymbolExpr::Value(e) => String::from(format!("{}", -e.value)),
                SymbolExpr::Binary(e) => match e.borrow().op {
                    BinaryOps::Add | BinaryOps::Sub => String::from(format!("-({})", s)),
                    _ => String::from(format!("-{}", s)),
                },
                _ => String::from(format!("-{}", s)),
            },
            UnaryOps::Sin => String::from(format!("sin({})", s)),
            UnaryOps::Asin => String::from(format!("asin({})", s)),
            UnaryOps::Cos => String::from(format!("cos({})", s)),
            UnaryOps::Acos => String::from(format!("acos({})", s)),
            UnaryOps::Tan => String::from(format!("tan({})", s)),
            UnaryOps::Atan => String::from(format!("atan({})", s)),
            UnaryOps::Exp => String::from(format!("exp({})", s)),
            UnaryOps::Log => String::from(format!("log({})", s)),
//            _ => todo!(),
        }
    }

    pub fn subs(&self, maps: &HashMap<String, f64>) -> SymbolExpr {
        let new_expr = Unary{ op: self.op.clone(), expr: self.expr.subs(maps),};
        match new_expr.clone().eval(false) {
            Some(v) => SymbolExpr::Value(Value{value: v}),
            None => SymbolExpr::Unary( Rc::new(RefCell::new(new_expr)))
        }
    }

    pub fn eval(&self, recurse: bool) -> Option<f64> {
        let val : f64;
        if recurse {
            match self.expr.eval(recurse) {
                Some(v) => val = v,
                None => return None,
            }
        }
        else {
            match &self.expr {
                SymbolExpr::Value(e) => val = e.value,
                _ => return None,
            }         
        }
        match self.op {
            UnaryOps::Abs => Some(val.abs()),
            UnaryOps::Neg => Some(-val),
            UnaryOps::Sin => Some(val.sin()),
            UnaryOps::Asin => Some(val.asin()),
            UnaryOps::Cos => Some(val.cos()),
            UnaryOps::Acos => Some(val.acos()),
            UnaryOps::Tan => Some(val.tan()),
            UnaryOps::Atan => Some(val.atan()),
            UnaryOps::Exp => Some(val.exp()),
            UnaryOps::Log => Some(val.ln()),
        }
    }

    pub fn symbols(&self) -> HashSet<String> {
        self.expr.symbols()
    }
}

impl PartialEq for Unary {
    fn eq(&self, r: &Self) -> bool {
        self.expr == r.expr
    }
}

// ===============================================================
//  implementations for Binary operators
// ===============================================================
impl Clone for Binary {
    fn clone(&self) -> Self {
        Binary {
            op : self.op.clone(),
            lhs : self.lhs.clone(),
            rhs : self.rhs.clone(),
        }
    }
}

impl Binary {
    pub fn to_string(&self) -> String {
        let s_lhs = self.lhs.to_string();
        let s_rhs = self.rhs.to_string();
        let op_lhs = match &self.lhs {
            SymbolExpr::Binary(e) => match e.borrow().op {
                BinaryOps::Add | BinaryOps::Sub => true,
                _ => false,
            },
            _ => false,
        };
        let op_rhs = match &self.rhs {
            SymbolExpr::Binary(e) => match e.borrow().op {
                BinaryOps::Add | BinaryOps::Sub => true,
                _ => false,
            },
            _ => false,
        };

        match self.op {
            BinaryOps::Add => String::from(format!("{} + {}", s_lhs, s_rhs)),
            BinaryOps::Sub => String::from(format!("{} - {}", s_lhs, s_rhs)),
            BinaryOps::Mul => if op_lhs {
                if op_rhs {
                    String::from(format!("({})*({})", s_lhs, s_rhs))
                } else {
                    String::from(format!("({})*{}", s_lhs, s_rhs))
                }
            } else {
                if op_rhs {
                    String::from(format!("{}*({})", s_lhs, s_rhs))
                } else {
                    String::from(format!("{}*{}", s_lhs, s_rhs))
                }
            },
            BinaryOps::Div => if op_lhs {
                if op_rhs {
                    String::from(format!("({})/({})", s_lhs, s_rhs))
                } else {
                    String::from(format!("({})/{}", s_lhs, s_rhs))
                }
            } else {
                if op_rhs {
                    String::from(format!("{}/({})", s_lhs, s_rhs))
                } else {
                    String::from(format!("{}/{}", s_lhs, s_rhs))
                }
            },
            BinaryOps::Pow => match &self.lhs {
                SymbolExpr::Binary(_) | SymbolExpr::Unary(_) => match &self.rhs {
                    SymbolExpr::Binary(_) | SymbolExpr::Unary(_) => String::from(format!("({})**({})", s_lhs, s_rhs)),
                    SymbolExpr::Value(r) => if r.value < 0.0 {
                        String::from(format!("({})**({})", s_lhs, s_rhs))
                    } else {
                        String::from(format!("({})**{}", s_lhs, s_rhs))
                    },
                    _ => String::from(format!("({})**{}", s_lhs, s_rhs)),
                },
                SymbolExpr::Value(l) => if l.value < 0.0 {
                    match &self.rhs {
                        SymbolExpr::Binary(_) | SymbolExpr::Unary(_) => String::from(format!("({})**({})", s_lhs, s_rhs)),
                        _ => String::from(format!("({})**{}", s_lhs, s_rhs)),
                    }
                } else {
                    match &self.rhs {
                        SymbolExpr::Binary(_) | SymbolExpr::Unary(_) => String::from(format!("{}**({})", s_lhs, s_rhs)),
                        _ => String::from(format!("{}**{}", s_lhs, s_rhs)),
                    }
                },
                _ => match &self.rhs {
                    SymbolExpr::Binary(_) | SymbolExpr::Unary(_) => String::from(format!("{}**({})", s_lhs, s_rhs)),                  
                    SymbolExpr::Value(r) => if r.value < 0.0 {
                        String::from(format!("{}**({})", s_lhs, s_rhs))
                    } else {
                        String::from(format!("{}**{}", s_lhs, s_rhs))
                    },
                    _ => String::from(format!("{}**{}", s_lhs, s_rhs)),
                },
            },
            _ => String::from(format!("{} {}", s_lhs, s_rhs)),
        }
    }

    pub fn subs(&self, maps: &HashMap<String, f64>) -> SymbolExpr {
        let new_expr = Binary{ op: self.op.clone(), lhs: self.lhs.subs(maps), rhs: self.rhs.subs(maps),};
        match new_expr.clone().eval(false) {
            Some(v) => SymbolExpr::Value(Value{value: v}),
            None => SymbolExpr::Binary( Rc::new(RefCell::new(new_expr)))
        }
    }

    pub fn eval(&self, recurse: bool) -> Option<f64> {
        let lval : f64;
        let rval : f64;
        if recurse {
            match self.lhs.eval(recurse) {
                Some(v) => lval = v,
                None => return None,
            }
            match self.rhs.eval(recurse) {
                Some(v) => rval = v,
                None => return None,
            }
        }
        else {
            match &self.lhs {
                SymbolExpr::Value(e) => lval = e.value,
                _ => return None,
            }         
            match &self.rhs {
                SymbolExpr::Value(e) => rval = e.value,
                _ => return None,
            }         
        }
        match self.op {
            BinaryOps::Add => Some(lval + rval),
            BinaryOps::Sub => Some(lval - rval),
            BinaryOps::Mul => Some(lval * rval),
            BinaryOps::Div => Some(lval / rval),
            BinaryOps::Pow => Some(lval.powf(rval)),
            BinaryOps::Nop => None,
        }
    }

    pub fn symbols(&self) -> HashSet<String> {
        let mut symbols = HashSet::<String>::new();
        for s in self.lhs.symbols().union(&self.rhs.symbols()) {
            symbols.insert(s.to_string());
        }
        symbols
    }
}

impl PartialEq for Binary {
    fn eq(&self, r: &Self) -> bool {
        self.lhs == r.lhs && self.rhs == r.rhs
    }
}




