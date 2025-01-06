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


use crate::symbol_expr::{SymbolExpr, Value};
use crate::symbol_parser::parse_expression;

use num_complex::Complex64;
use hashbrown::{HashMap, HashSet};

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;

// Python interface to SymbolExpr
#[pyclass(sequence, module = "qiskit._accelerate.circuit")]
#[derive(Clone, Debug)]
pub struct PySymbolExpr {
    expr: SymbolExpr,
}

// enum for argument for operators
#[derive(FromPyObject, Clone, Debug)]
pub enum ParameterValue {
    #[pyo3(transparent, annotation = "float")]
    Real(f64),
    #[pyo3(transparent, annotation = "complex")]
    Complex(Complex64),
    #[pyo3(transparent, annotation = "int")]
    Int(i32),
    #[pyo3(transparent, annotation = "str")]
    Str(String),
    Expr(PySymbolExpr),
}


#[pymethods]
impl PySymbolExpr {
    #[new]
    #[pyo3(signature = (in_expr=None))]
    pub fn new(
        in_expr: Option<String>,
    ) -> PyResult<Self> {
        match in_expr {
            Some(e) => Ok(PySymbolExpr {
                expr : parse_expression(&e),
            }),
            None => Ok(PySymbolExpr {
                expr : SymbolExpr::Value(Value::Real(0.0)),
            }),
        }
    }

    #[staticmethod]
    pub fn Symbol(name: String) -> Self {
        PySymbolExpr {
            expr : parse_expression(&name),
        }
    }


    pub fn sin(&self) -> Self {
        Self {
            expr: self.expr.sin(),
        }
    }
    pub fn cos(&self) -> Self {
        Self {
            expr: self.expr.cos(),
        }
    }
    pub fn tan(&self) -> Self {
        Self {
            expr: self.expr.tan(),
        }
    }
    pub fn asin(&self) -> Self {
        Self {
            expr: self.expr.asin(),
        }
    }
    pub fn acos(&self) -> Self {
        Self {
            expr: self.expr.acos(),
        }
    }
    pub fn atan(&self) -> Self {
        Self {
            expr: self.expr.atan(),
        }
    }
    pub fn exp(&self) -> Self {
        Self {
            expr: self.expr.exp(),
        }
    }
    pub fn log(&self) -> Self {
        Self {
            expr: self.expr.log(),
        }
    }
    pub fn abs(&self) -> Self {
        Self {
            expr: self.expr.abs(),
        }
    }
    pub fn sign(&self) -> f64 {
        self.expr.sign()
    }
    pub fn complex(&self) -> PyResult<Complex64> {
        match self.expr.complex() {
            Some(c) => Ok(c),
            None=> Err(pyo3::exceptions::PyRuntimeError::new_err("Expression has some undefined symbols.")),
        }
    }
    pub fn float(&self) -> PyResult<f64> {
        match self.expr.real() {
            Some(r) => Ok(r),
            None=> Err(pyo3::exceptions::PyRuntimeError::new_err("Expression has some undefined symbols.")),
        }
   }
    pub fn copy(&self) -> Self {
        Self {
            expr: self.expr.clone(),
        }
    }
    pub fn conjugate(&self) -> Self {
        Self {
            expr: self.expr.conjugate(),
        }
    }
    pub fn derivative(&self, param: String) -> Self {
        Self {
            expr: self.expr.derivative(&param),
        }
    }

    #[getter]
    pub fn is_real(&self) -> bool {
        self.expr.is_real()
    }
    #[getter]
    pub fn is_complex(&self) -> bool {
        self.expr.is_complex()
    }

    #[getter]
    pub fn symbols(&self) -> HashSet<String> {
        self.expr.symbols()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.expr.to_string()
    }

    pub fn bind(&self, maps: HashMap<String, f64>) -> Self {
        Self {
            expr: self.expr.bind(&maps),
        }
    }
    pub fn subs(&self, in_maps: HashMap<String, Self>) -> Self {
        let maps : HashMap::<String, SymbolExpr> = 
            in_maps.iter().map(|(key, val)| (key.clone(), val.expr.clone())).collect();
        Self {
            expr: self.expr.subs(&maps),
        }
    }

    // ====================================
    // operator overrides
    // ====================================
    pub fn __eq__(&self, rhs: ParameterValue) -> bool {
        match rhs {
            ParameterValue::Real(r) => self.expr == SymbolExpr::Value( Value::Real(r.clone())),
            ParameterValue::Complex(c) => self.expr == SymbolExpr::Value( Value::Complex(c.clone())),
            ParameterValue::Int(r) => self.expr == SymbolExpr::Value( Value::Real(r.clone().into())),
            ParameterValue::Str(s) => self.expr == parse_expression(&s),
            ParameterValue::Expr(e) => self.expr == e.expr,
        }
    }
    pub fn __ne__(&self, rhs: ParameterValue) -> bool {
        match rhs {
            ParameterValue::Real(r) => self.expr != SymbolExpr::Value( Value::Real(r.clone())),
            ParameterValue::Complex(c) => self.expr != SymbolExpr::Value( Value::Complex(c.clone())),
            ParameterValue::Int(r) => self.expr != SymbolExpr::Value( Value::Real(r.clone().into())),
            ParameterValue::Str(s) => self.expr != parse_expression(&s),
            ParameterValue::Expr(e) => self.expr != e.expr,
        }
    }
    pub fn __neg__(&self) -> Self {
        Self {
            expr: -&self.expr,
        }
    }
    pub fn __add__(&self, rhs: ParameterValue) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: &self.expr + &SymbolExpr::Value( Value::Real(r.clone()))},
            ParameterValue::Complex(c) => Self {expr: &self.expr + &SymbolExpr::Value( Value::Complex(c.clone()))},
            ParameterValue::Int(r) => Self {expr: &self.expr + &SymbolExpr::Value( Value::Real(r.clone().into()))},
            ParameterValue::Str(s) => Self {expr: &self.expr + &parse_expression(&s)},
            ParameterValue::Expr(e) => Self {expr: &self.expr + &e.expr},
        }
    }
    pub fn __radd__(&self, rhs: ParameterValue) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: &SymbolExpr::Value( Value::Real(r.clone())) + &self.expr},
            ParameterValue::Complex(c) => Self {expr: &SymbolExpr::Value( Value::Complex(c.clone())) + &self.expr},
            ParameterValue::Int(r) => Self {expr: &SymbolExpr::Value( Value::Real(r.clone().into())) + &self.expr},
            ParameterValue::Str(s) => Self {expr: &parse_expression(&s) + &self.expr},
            ParameterValue::Expr(e) => Self {expr: &e.expr + &self.expr},
        }
    }
    pub fn __sub__(&self, rhs: ParameterValue) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: &self.expr - &SymbolExpr::Value( Value::Real(r.clone()))},
            ParameterValue::Complex(c) => Self {expr: &self.expr - &SymbolExpr::Value( Value::Complex(c.clone()))},
            ParameterValue::Int(r) => Self {expr: &self.expr - &SymbolExpr::Value( Value::Real(r.clone().into()))},
            ParameterValue::Str(s) => Self {expr: &self.expr - &parse_expression(&s)},
            ParameterValue::Expr(e) => Self {expr: &self.expr - &e.expr},
        }
    }
    pub fn __rsub__(&self, rhs: ParameterValue) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: &SymbolExpr::Value( Value::Real(r.clone())) - &self.expr},
            ParameterValue::Complex(c) => Self {expr: &SymbolExpr::Value( Value::Complex(c.clone())) - &self.expr},
            ParameterValue::Int(r) => Self {expr: &SymbolExpr::Value( Value::Real(r.clone().into())) - &self.expr},
            ParameterValue::Str(s) => Self {expr: &parse_expression(&s) - &self.expr},
            ParameterValue::Expr(e) => Self {expr: &e.expr - &self.expr},
        }
    }
    pub fn __mul__(&self, rhs: ParameterValue) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: &self.expr * &SymbolExpr::Value( Value::Real(r.clone()))},
            ParameterValue::Complex(c) => Self {expr: &self.expr * &SymbolExpr::Value( Value::Complex(c.clone()))},
            ParameterValue::Int(r) => Self {expr: &self.expr * &SymbolExpr::Value( Value::Real(r.clone().into()))},
            ParameterValue::Str(s) => Self {expr: &self.expr * &parse_expression(&s)},
            ParameterValue::Expr(e) => Self {expr: &self.expr * &e.expr},
        }
    }
    pub fn __rmul__(&self, rhs: ParameterValue) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: &SymbolExpr::Value( Value::Real(r.clone())) * &self.expr},
            ParameterValue::Complex(c) => Self {expr: &SymbolExpr::Value( Value::Complex(c.clone())) * &self.expr},
            ParameterValue::Int(r) => Self {expr: &SymbolExpr::Value( Value::Real(r.clone().into())) * &self.expr},
            ParameterValue::Str(s) => Self {expr: &parse_expression(&s) * &self.expr},
            ParameterValue::Expr(e) => Self {expr: &e.expr * &self.expr},
        }
    }
    pub fn __truediv__(&self, rhs: ParameterValue) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: &self.expr / &SymbolExpr::Value( Value::Real(r.clone()))},
            ParameterValue::Complex(c) => Self {expr: &self.expr / &SymbolExpr::Value( Value::Complex(c.clone()))},
            ParameterValue::Int(r) => Self {expr: &self.expr / &SymbolExpr::Value( Value::Real(r.clone().into()))},
            ParameterValue::Str(s) => Self {expr: &self.expr / &parse_expression(&s)},
            ParameterValue::Expr(e) => Self {expr: &self.expr / &e.expr},
        }
    }
    pub fn __rtruediv__(&self, rhs: ParameterValue) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: &SymbolExpr::Value( Value::Real(r.clone())) / &self.expr},
            ParameterValue::Complex(c) => Self {expr: &SymbolExpr::Value( Value::Complex(c.clone())) / &self.expr},
            ParameterValue::Int(r) => Self {expr: &SymbolExpr::Value( Value::Real(r.clone().into())) / &self.expr},
            ParameterValue::Str(s) => Self {expr: &parse_expression(&s) / &self.expr},
            ParameterValue::Expr(e) => Self {expr: &e.expr / &self.expr},
        }
    }
    pub fn __pow__(&self, rhs: ParameterValue, _modulo: Option<i32>) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: self.expr.pow(&SymbolExpr::Value( Value::Real(r.clone())))},
            ParameterValue::Complex(c) => Self {expr: self.expr.pow(&SymbolExpr::Value( Value::Complex(c.clone())))},
            ParameterValue::Int(r) => Self {expr: self.expr.pow(&SymbolExpr::Value( Value::Real(r.clone().into())))},
            ParameterValue::Str(s) => Self {expr: self.expr.pow(&parse_expression(&s))},
            ParameterValue::Expr(e) => Self {expr: self.expr.pow(&e.expr)},
        }
    }
    pub fn __rpow__(&self, rhs: ParameterValue, _modulo: Option<i32>) -> Self {
        match rhs {
            ParameterValue::Real(r) => Self {expr: SymbolExpr::Value( Value::Real(r.clone())).pow(&self.expr)},
            ParameterValue::Complex(c) => Self {expr: SymbolExpr::Value( Value::Complex(c.clone())).pow(&self.expr)},
            ParameterValue::Int(r) => Self {expr: SymbolExpr::Value( Value::Real(r.clone().into())).pow(&self.expr)},
            ParameterValue::Str(s) => Self {expr: parse_expression(&s).pow(&self.expr)},
            ParameterValue::Expr(e) => Self {expr: e.expr.pow(&self.expr)},
        }
    }
    pub fn __str__(&self) -> String {
        self.expr.to_string()
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.expr.to_string().hash(&mut hasher);
        hasher.finish()
    }
}

#[pyfunction]
pub fn sin(expr: &PySymbolExpr) -> PySymbolExpr {
    expr.sin()
}

#[pyfunction]
pub fn cos(expr: &PySymbolExpr) -> PySymbolExpr {
    expr.cos()
}

#[pyfunction]
pub fn tan(expr: &PySymbolExpr) -> PySymbolExpr {
    expr.tan()
}

#[pyfunction]
pub fn asin(expr: &PySymbolExpr) -> PySymbolExpr {
    expr.asin()
}

#[pyfunction]
pub fn acos(expr: &PySymbolExpr) -> PySymbolExpr {
    expr.acos()
}

#[pyfunction]
pub fn atan(expr: &PySymbolExpr) -> PySymbolExpr {
    expr.atan()
}

#[pyfunction]
pub fn abs(expr: &PySymbolExpr) -> PySymbolExpr {
    expr.abs()
}

#[pyfunction]
pub fn exp(expr: &PySymbolExpr) -> PySymbolExpr {
    expr.exp()
}

#[pyfunction]
pub fn log(expr: &PySymbolExpr) -> PySymbolExpr {
    expr.log()
}

#[pyfunction]
pub fn sign(expr: &PySymbolExpr) -> f64 {
    expr.sign()
}
