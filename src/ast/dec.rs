use super::{exp, field, stm, ty, var::Var};

pub type Dec = Box<Dec_>;
pub type DecList = Vec<Dec>;

#[derive(Clone, Debug)]
pub struct Dec_ {
    pub pos: (usize, usize),
    pub data: DecData,
}

#[derive(Clone, Debug)]
pub enum DecData {
    Func(String, field::FieldList, field::FieldList, stm::Stm),
    Oper(String, field::FieldList, field::FieldList, stm::Stm),
    JsImport(String, field::FieldList, field::FieldList, String, String),
    JsExport(String, String),

    Var(Var, ty::Type, exp::ASTExp),
    Class(String, ClassMemberList, Vec<String>),
    Template(Dec, Vec<String>),
    None,
}

impl Dec_ {
    pub fn func_dec(
        pos: (usize, usize),
        name: String,
        params: field::FieldList,
        result: field::FieldList,
        body: stm::Stm,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Func(name, params, result, body),
        })
    }
    pub fn oper_dec(
        pos: (usize, usize),
        op: String,
        params: field::FieldList,
        result: field::FieldList,
        body: stm::Stm,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Oper(op, params, result, body),
        })
    }
    pub fn js_import_dec(
        pos: (usize, usize),
        name: String,
        params: field::FieldList,
        result: field::FieldList,
        module: String,
        id: String,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::JsImport(name, params, result, module, id),
        })
    }
    pub fn js_export_dec(pos: (usize, usize), name: String, export_name: String) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::JsExport(name, export_name),
        })
    }
    pub fn var_dec(pos: (usize, usize), var: Var, ty: ty::Type, init: exp::ASTExp) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Var(var, ty, init),
        })
    }
    pub fn class_dec(
        pos: (usize, usize),
        name: String,
        class_members: ClassMemberList,
        inheritance: Vec<String>,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Class(name, class_members, inheritance),
        })
    }
    pub fn template_dec(pos: (usize, usize), dec: Dec, ty_params: Vec<String>) -> Dec {
        Box::new(Dec_ {
            pos,
            data: DecData::Template(dec, ty_params),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MemberSpecifier {
    Public,
    Private,
}

#[derive(Clone, Debug)]
pub struct ClassMember_ {
    pub specifier: MemberSpecifier,
    pub dec: Dec,
}

pub type ClassMember = Box<ClassMember_>;
pub type ClassMemberList = Vec<ClassMember>;

pub trait MemberList {
    fn new_list(declist: Vec<Dec>, specifier: MemberSpecifier) -> ClassMemberList;
    fn append_list(&mut self, declist: ClassMemberList);
}

impl MemberList for Vec<ClassMember> {
    fn new_list(declist: Vec<Dec>, specifier: MemberSpecifier) -> ClassMemberList {
        let mut new_list: ClassMemberList = vec![];
        for dec in declist {
            new_list.push(Box::new(ClassMember_ { specifier, dec }));
        }
        new_list
    }
    fn append_list(&mut self, declist: ClassMemberList) {
        for dec in declist {
            self.push(dec);
        }
    }
}
