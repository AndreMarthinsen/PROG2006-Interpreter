use std::fmt::{Display, Formatter};
use crate::op::Op;






#[derive(Clone)]
pub struct Signature {
    pub stack_args: Params,
    pub modifers: Params,
    pub ret: Constraint,
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.stack_args, self.ret)
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.modifers == other.modifers &&
            self.stack_args == other.stack_args &&
            self.ret == other.ret
    }
}

pub fn nullary(ret_type: Constraint) -> Signature {
    Signature { stack_args: Params::Nullary, modifers: Params::Nullary, ret: ret_type }
}

pub fn homogenous_binary(arg_type: Constraint, ret_type: Constraint) -> Signature {
    Signature {
        stack_args: Params::Binary (arg_type.clone(), arg_type),
        modifers: Params::Nullary,
        ret: ret_type,
    }
}

pub fn heterogeneous_binary(lh_arg: Constraint, rh_arg: Constraint, ret_type: Constraint) -> Signature {
    Signature {
        stack_args: Params::Binary (lh_arg, rh_arg),
        modifers: Params::Nullary,
        ret: ret_type,
    }
}

pub fn unary(arg_type: Constraint, ret_type: Constraint) -> Signature {
    Signature { stack_args: Params::Unary ( arg_type ), modifers: Params::Nullary, ret: ret_type,}
}



#[derive(Clone, PartialEq)]
pub enum Params {
    Nullary,
    Unary(Constraint),
    Binary(Constraint, Constraint),
    Temary(Constraint, Constraint, Constraint),
    //NAry(Vec<Vec<Constraint>>)
}

impl Display for Params {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Params::Nullary =>
                write!(f, "Void"),
            Params::Unary(c) =>
                write!(f, "{}", c),
            Params::Binary(c1, c2) =>
                write!(f, "{}, {}", c1, c2),
            Params::Temary(c1, c2, c3) =>
                write!(f, "{}, {}, {}", c1, c2, c3)
        }
    }
}









#[derive(Clone, PartialEq)]
pub enum Constraint {

    // Exact types
    Void,
    String,
    List,
    Integer,
    Float,
    Bool,
    Quotation,
    Error,
    Symbol,
    Function(Box<Signature>),

    // TypeClasses below
    Any,
    Ord,
    Eq,
    Num,
    Functor,
    Boolean,
    Enum,
    Display,
    Executable,
    Sized
}

impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match  self {
            Constraint::Void => write!(f, "Void"),
            Constraint::String => write!(f, "String"),
            Constraint::List => write!(f, "List"),
            Constraint::Integer => write!(f, "Integer"),
            Constraint::Float => write!(f, "Float"),
            Constraint::Bool => write!(f, "Bool"),
            Constraint::Quotation => write!(f, "Quotation"),
            Constraint::Error => write!(f, "Error"),
            Constraint::Symbol => write!(f, "Symbol"),
            Constraint::Function(_) => write!(f, "Function"),
            Constraint::Any => write!(f, "Any"),
            Constraint::Ord => write!(f, "Ord"),
            Constraint::Eq => write!(f, "Eq"),
            Constraint::Num => write!(f, "Num"),
            Constraint::Functor => write!(f, "Functor"),
            Constraint::Boolean => write!(f, "Boolean"),
            Constraint::Enum => write!(f, "Enum"),
            Constraint::Display => write!(f, "Display"),
            Constraint::Executable => write!(f, "Executable"),
            Constraint::Sized => write!(f, "Sized"),
        }
    }
}

impl Constraint {
    pub fn is_satisfied_by(&self, t: &Type) -> bool {
        if self == &t.as_constraint() {
            return true
        } else {
            match self {
                Constraint::Any => {
                    t.implements(&TypeClass::Any)
                }
                Constraint::Ord => {
                    t.implements(&TypeClass::Ordering)
                }
                Constraint::Eq => {
                    t.implements(&TypeClass::Eq)
                }
                Constraint::Num => {
                    t.implements(&TypeClass::Num)
                }
                Constraint::Functor => {
                    t.implements(&TypeClass::Functor)
                }
                Constraint::Boolean => {
                    t.implements(&TypeClass::Boolean)
                }
                Constraint::Enum => {
                    t.implements(&TypeClass::Enum)
                },
                Constraint::Display => {
                    t.implements(&TypeClass::Display)
                },
                Constraint::Executable => {
                    t.implements(&TypeClass::Executable)
                },
                Constraint::Sized => {
                    t.implements(&TypeClass::Sized)
                }
                _ => false,
            }
        }
    }
}






#[derive(PartialEq, Clone)]
pub enum Type {
    Void,
    String,
    List,
    Integer,
    Float,
    Bool,
    Quotation,
    Error,
    Symbol,
    Function(Signature)
}


impl Type {
    pub fn as_constraint(&self) -> Constraint {
        match self {
            Type::Void => Constraint::Void,
            Type::String => Constraint::String,
            Type::List => Constraint::List,
            Type::Integer => Constraint::Integer,
            Type::Float => Constraint::Float,
            Type::Bool => Constraint::Bool,
            Type::Quotation => Constraint::Quotation,
            Type::Error => Constraint::Error,
            Type::Symbol => Constraint::Symbol,
            Type::Function(sig) => Constraint::Function(Box::new(sig.clone()))
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "Void"),
            Type::String => write!(f, "String"),
            Type::List => write!(f, "List"),
            Type::Integer => write!(f, "Integer"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::Quotation =>write!(f, "Quotation"),
            Type::Error => write!(f, "Error"),
            Type::Symbol => write!(f, "Symbol"),
            Type::Function(fun) => write!(f, "Func {}", fun),
        }
    }
}




#[derive(PartialEq, Copy, Clone)]
pub(crate) enum TypeClass {
    Any,
    Ordering, // Comparisons
    Eq,
    Num, // Arithmetic operations
    Functor, // Mapping
    Boolean, // Types with a truth value
    Enum, //
    Display,
    Executable,
    Sized
}


fn void_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Eq |
        TypeClass::Any |
        TypeClass::Display => true,
        _ => false,
    }
}

fn string_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Any |
        TypeClass::Boolean |
        TypeClass::Display |
        TypeClass::Eq |
        TypeClass::Sized => true,
        _ => false,
    }
}

fn list_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Any |
        TypeClass::Eq |
        TypeClass::Functor |
        TypeClass::Boolean |
        TypeClass::Display |
        TypeClass::Sized => true,
        _ => false,
    }
}

fn integer_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Any |
        TypeClass::Ordering |
        TypeClass::Eq |
        TypeClass::Num |
        TypeClass::Boolean |
        TypeClass::Display => true,
        _ => false,
    }
}

fn float_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Any |
        TypeClass::Ordering |
        TypeClass::Eq |
        TypeClass::Num |
        TypeClass::Boolean |
        TypeClass::Display => true,
        _ => false,
    }
}

fn bool_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Any |
        TypeClass::Ordering |
        TypeClass::Eq |
        TypeClass::Num |
        TypeClass::Boolean |
        TypeClass::Enum |
        TypeClass::Display => true,
        _ => false,
    }
}

fn quotation_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Any |
        TypeClass::Boolean |
        TypeClass::Display |
        TypeClass::Executable |
        TypeClass::Sized => true,
        _ => false,
    }
}

fn error_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Any => true,
        TypeClass::Boolean |
        TypeClass::Display => true,
        _ => false
    }
}

fn symbol_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Any |
        TypeClass::Display => true,
        _ => false
    }
}

fn function_implements(class: &TypeClass) -> bool {
    match class {
        TypeClass::Any => true,
        TypeClass::Executable => true,
        _ => false
    }
}


impl Type {
    fn implements(&self, class: &TypeClass) -> bool {
        match self {
            Type::Void => void_implements(class),
            Type::String => string_implements(class),
            Type::List => list_implements(class),
            Type::Integer => integer_implements(class),
            Type::Float => float_implements(class),
            Type::Bool => bool_implements(class),
            Type::Quotation => quotation_implements(class),
            Type::Error => error_implements(class),
            Type::Symbol => symbol_implements(class),
            Type::Function(_) => function_implements(class),
            _ => false
        }
    }
}

/// Defines coercion rules for binary operations working
/// on two potentially different types that can be coerced into
/// a number type.
pub fn numeric_coercion(t1: &Type, t2: &Type) -> Option<Type> {
    if t1 == t2 {
        return Some(t1.clone());
    }
    match (t1, t2) {
        (Type::Integer, Type::Float) |
        (Type::Float, Type::Integer) => {
            Some(Type::Float)
        },
        (Type::Bool, Type::Integer) |
        (Type::Integer, Type::Bool) => {
            Some(Type::Integer)
        },
        (Type::Bool, Type::Float) |
        (Type::Float, Type::Bool) => {
            Some(Type::Float)
        },
        (_, _) => None
    }
}









