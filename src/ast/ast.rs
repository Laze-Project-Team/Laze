use std::io::{stderr, Write};
use std::mem;

use peg_parser::{Parser, ParserData};

use crate::laze_parser::matcher::extract_ast;

use super::{
    dec::{self, ClassMemberList, Dec, DecData, DecList, Dec_},
    exp::{Exp, ExpData, ExpList, Exp_},
    field::{Field, FieldData, FieldList, Field_},
    ifelse::{IfElse, IfElseList},
    op::{Oper, OperList},
    stm::{Stm, StmData, StmList, Stm_},
    suffix::ExpSuffixList,
    ty::{Type, TypeData, TypeList, Type_},
    var::{Var, VarData, Var_},
};

pub type AST = dec::DecList;

#[derive(Clone, Debug)]
pub enum ASTNode {
    Dec(Dec),
    Stm(Stm),
    Exp(Exp),
    Type(Type),
    Field(Field),
    String(String),
    Var(Var),
    IfElse(IfElse),
    Op(Oper),
    DecList(DecList),
    StmList(StmList),
    ExpList(ExpList),
    FieldList(FieldList),
    TypeList(TypeList),
    StringList(Vec<String>),
    IfElseList(IfElseList),
    ExpSuffixList(ExpSuffixList),
    OperList(OperList),
    ClassMemberList(ClassMemberList),
    None,
}

impl ASTNode {
    pub fn get_var_data(self, pos: (usize, usize), name: &str, rule: &str) -> Var {
        if let ASTNode::Var(var) = self {
            var
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a declaration.");
            Box::new(Var_ {
                pos,
                data: VarData::None,
            })
        }
    }
    pub fn get_dec_data(self, pos: (usize, usize), name: &str, rule: &str) -> Dec {
        match self {
            ASTNode::Dec(dec) => dec,
            ASTNode::DecList(mut declist) => {
                if declist.len() == 1 {
                    let mut temp_dec = Box::new(Dec_ {
                        pos,
                        data: DecData::None,
                    });
                    mem::swap(&mut declist[0], &mut temp_dec);
                    temp_dec
                } else {
                    let _ = writeln!(stderr(), "{name} in {rule} is not a declaration.");
                    Box::new(Dec_ {
                        pos,
                        data: DecData::None,
                    })
                }
            }
            _ => {
                let _ = writeln!(stderr(), "{name} in {rule} is not a declaration.");
                Box::new(Dec_ {
                    pos,
                    data: DecData::None,
                })
            }
        }
    }
    pub fn get_declist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> DecList {
        if let ASTNode::DecList(declist) = self {
            declist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a declaration list.");
            vec![]
        }
    }
    pub fn get_stm_data(self, pos: (usize, usize), name: &str, rule: &str) -> Stm {
        match self {
            ASTNode::Stm(stm) => stm,
            ASTNode::StmList(mut stmlist) => {
                if stmlist.len() == 1 {
                    let mut temp_stm = Box::new(Stm_ {
                        pos,
                        data: StmData::None,
                    });
                    mem::swap(&mut stmlist[0], &mut temp_stm);
                    temp_stm
                } else {
                    let _ = writeln!(stderr(), "{name} in {rule} is not a statement.");
                    Box::new(Stm_ {
                        pos,
                        data: StmData::None,
                    })
                }
            }
            _ => {
                let _ = writeln!(stderr(), "{name} in {rule} is not a statement.");
                Box::new(Stm_ {
                    pos,
                    data: StmData::None,
                })
            }
        }
    }
    pub fn get_stmlist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> StmList {
        if let ASTNode::StmList(stmlist) = self {
            stmlist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a statement list.");
            vec![]
        }
    }
    pub fn get_exp_data(self, pos: (usize, usize), name: &str, rule: &str) -> Exp {
        match self {
            ASTNode::Exp(exp) => exp,
            ASTNode::ExpList(mut explist) => {
                let mut temp_exp = Box::new(Exp_ {
                    pos,
                    data: ExpData::None,
                });
                if explist.len() == 1 {
                    mem::swap(&mut explist[0], &mut temp_exp);
                } else if explist.len() == 0 {
                } else {
                    // let _ = writeln!(stderr(), "{name} in {rule} is not an expression.");
                    mem::swap(&mut explist[0], &mut temp_exp);
                }
                temp_exp
            }
            _ => {
                let _ = writeln!(stderr(), "{name} in {rule} is not an expression.");
                return Box::new(Exp_ {
                    pos,
                    data: ExpData::None,
                });
            }
        }
    }
    pub fn get_explist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> ExpList {
        if let ASTNode::ExpList(explist) = self {
            explist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not an expression list.");
            vec![]
        }
    }
    pub fn get_classmembers_data(
        self,
        _pos: (usize, usize),
        name: &str,
        rule: &str,
    ) -> ClassMemberList {
        if let ASTNode::ClassMemberList(members) = self {
            members
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not an class members list.");
            vec![]
        }
    }
    pub fn get_ty_data(self, pos: (usize, usize), name: &str, rule: &str) -> Type {
        if let ASTNode::Type(ty) = self {
            ty
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a type.");
            Box::new(Type_ {
                pos,
                data: TypeData::None,
            })
        }
    }
    pub fn get_tylist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> TypeList {
        if let ASTNode::TypeList(tylist) = self {
            tylist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a type.");
            vec![]
        }
    }
    pub fn get_field_data(self, pos: (usize, usize), name: &str, rule: &str) -> Field {
        if let ASTNode::Field(field) = self {
            field
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a field.");
            Box::new(Field_ {
                pos,
                data: FieldData::None,
            })
        }
    }
    pub fn get_oper_data(self, _pos: (usize, usize), name: &str, rule: &str) -> Oper {
        if let ASTNode::Op(oper) = self {
            oper
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a field.");
            Oper::None
        }
    }
    pub fn get_fieldlist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> FieldList {
        if let ASTNode::FieldList(fieldlist) = self {
            fieldlist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a fieldlist.");
            vec![]
        }
    }
    pub fn get_string_data(self, _pos: (usize, usize), name: &str, rule: &str) -> String {
        if let ASTNode::String(str) = self {
            str
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string.");
            "".to_string()
        }
    }
    pub fn get_stringlist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> Vec<String> {
        if let ASTNode::StringList(strlist) = self {
            strlist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string list.");
            vec![]
        }
    }
    pub fn get_suffixlist_data(
        self,
        _pos: (usize, usize),
        name: &str,
        rule: &str,
    ) -> ExpSuffixList {
        if let ASTNode::ExpSuffixList(suffixlist) = self {
            suffixlist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string list.");
            vec![]
        }
    }
    pub fn get_ifelselist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> IfElseList {
        if let ASTNode::IfElseList(ifelselist) = self {
            ifelselist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string list.");
            vec![]
        }
    }
    pub fn get_operlist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> OperList {
        if let ASTNode::OperList(oplist) = self {
            oplist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string list.");
            vec![]
        }
    }
}

impl ParserData for ASTNode {
    fn string(_: (usize, usize), str: String) -> Self {
        Self::String(str)
    }
    fn null() -> Self {
        Self::None
    }
    fn data(pos: (usize, usize), name: &str, parser: &mut Parser<Self>) -> Self {
        extract_ast(pos, name, parser)
    }
    fn is_null(&self) -> bool {
        if let Self::None = self {
            true
        } else {
            false
        }
    }
}
