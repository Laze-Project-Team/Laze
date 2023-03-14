use std::cmp::Ordering;

use crate::ast::ast::ASTNode;
use crate::ast::dec::{DecData, DecList};
use crate::wasm::il::module::ModuleList;

use super::semantic_param::SemanticParam;
use super::trans_dec::trans_dec;

pub fn trans_ast(tree: ASTNode) -> (ModuleList, i32) {
    match tree {
        ASTNode::DecList(declist) => {
            let new_list = sort_declist(declist);
            let mut semantic_param = SemanticParam::new();
            for dec in new_list {
                trans_dec(&dec, None, &mut semantic_param);
            }
            let mem_size = semantic_param.get_mem_size();
            (semantic_param.result_modlist, mem_size)
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
