use std::cmp::Ordering;

use crate::ast::ast::ASTNode;
use crate::ast::dec::{DecData, DecList};
use crate::wasm::il::module::ModuleList;

use super::trans_dec::trans_dec;

pub fn trans_ast(tree: ASTNode) -> ModuleList {
    match tree {
        ASTNode::DecList(declist) => {
            let new_list = sort_declist(declist);
            let mut result_list: ModuleList = vec![];
            for dec in new_list {
                result_list.append(&mut trans_dec(dec));
            }
            result_list
        }
        _ => {
            panic!("The parsed ASTNode is not a declist.");
        }
    }
}

fn sort_declist(mut list: DecList) -> DecList {
    list.sort_by(|a, b| {
        if let DecData::JsImport(..) = a.data {
            return Ordering::Less;
        }
        if let DecData::JsImport(..) = b.data {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    });
    list
}
