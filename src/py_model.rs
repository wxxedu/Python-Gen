use crate::py_core::py_closure::PyClosure;
use crate::py_core::py_to_lines::PyToLines;
use crate::py_model::py_doc::PyDoc;
use crate::py_model::py_elif::PyElif;
use crate::py_model::py_for::PyFor;
use crate::py_model::py_func::PyFunc;
use crate::py_model::py_if::PyIf;
use crate::py_model::py_line::PyLine;
use crate::py_model::py_while::PyWhile;
use crate::py_model::PyModel::{
    Doc, Elif, For, Func, FuncInvoke, If, Line, While,
};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

pub mod py_line;

pub mod py_if;

pub mod py_elif;

pub mod py_for;

pub mod py_while;

pub mod py_func;

pub mod py_doc;

pub mod py_class;

/// The enum representing a Python model
#[derive(Debug, Clone)]
pub enum PyModel {
    Line(PyLine),
    If(PyIf),
    Elif(PyElif),
    For(PyFor),
    While(PyWhile),
    Func(PyFunc),
    FuncInvoke(PyFunc),
    Doc(PyDoc),
}

impl Display for PyModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.to_lines() {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl PyModel {
    pub fn write_to_file(&self, file: &mut File) -> std::io::Result<()> {
        file.write_all(self.to_string().as_bytes())
    }
}

impl PyToLines for PyModel {
    fn to_lines(&self) -> Vec<String> {
        match self {
            Line(val) => val.to_lines(),
            If(val) => val.to_lines(),
            Elif(val) => val.to_lines(),
            For(val) => val.to_lines(),
            While(val) => val.to_lines(),
            Func(val) => val.to_lines(),
            FuncInvoke(val) => vec![val.invoke_str()],
            Doc(val) => val.to_lines(),
        }
    }
}
