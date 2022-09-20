use super::{exp, field, op, stm, ty};

pub type Dec = Box<Dec_>;
pub type DecList = Vec<Dec>;

#[derive(Clone)]
pub struct Dec_ {
    pos: (i32, i32),
    data: DecData,
}

#[derive(Clone)]
pub enum DecData {
    Func(String, field::FieldList, ty::Type, stm::Stm),
    Oper(op::Oper, field::FieldList, ty::Type, stm::Stm),
    JsImport(String, field::FieldList, ty::Type, String, String),
    JsExport(String, String),

    Var(String, ty::Type, exp::Exp, bool),
    Class(String, ClassMemberList, Vec<String>),
    Template(Dec, Vec<String>),
}

impl Dec_ {
    fn FuncDec(
        pos: (i32, i32),
        name: String,
        params: field::FieldList,
        result: ty::Type,
        body: stm::Stm,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Func(name, params, result, body),
        })
    }
    fn OperDec(
        pos: (i32, i32),
        op: op::Oper,
        params: field::FieldList,
        result: ty::Type,
        body: stm::Stm,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Oper(op, params, result, body),
        })
    }
    fn JsImportDec(
        pos: (i32, i32),
        name: String,
        params: field::FieldList,
        result: ty::Type,
        module: String,
        id: String,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::JsImport(name, params, result, module, id),
        })
    }
    fn JsExportDec(pos: (i32, i32), name: String, exportName: String) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::JsExport(name, exportName),
        })
    }
    fn VarDec(pos: (i32, i32), name: String, ty: ty::Type, init: exp::Exp, isEscape: bool) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Var(name, ty, init, isEscape),
        })
    }
    fn ClassDec(
        pos: (i32, i32),
        name: String,
        classMembers: ClassMemberList,
        inheritance: Vec<String>,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Class(name, classMembers, inheritance),
        })
    }
    fn TemplateDec(pos: (i32, i32), dec: Dec, tyParams: Vec<String>) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Template(dec, tyParams),
        })
    }
}

pub enum MemberSpecifier {
    Public,
    Private,
}

impl Clone for MemberSpecifier {
    fn clone(&self) -> Self {
        match self {
            Self::Public => Self::Public,
            Self::Private => Self::Private,
        }
    }
}

pub struct ClassMember_ {
    specifier: MemberSpecifier,
    dec: Dec,
}

pub type ClassMember = Box<ClassMember_>;

impl Clone for ClassMember_ {
    fn clone(&self) -> Self {
        Self {
            specifier: self.specifier.clone(),
            dec: self.dec.clone(),
        }
    }
}

pub type ClassMemberList = Vec<ClassMember>;
