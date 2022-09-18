// pub mod IL;
pub mod ast;
// pub mod semantic;
pub mod parser;

use parser::peg_parser::*;

fn main() {
    // match init_peg_parser().get("Rules") {
    //     Some(matcher) => {
    //         let s = matcher("Test = \"hello\" [㐀-龯ぁ-んァ-ヶa-zA-Z_ー] \"!\"\nTest = \"hello\" [㐀-龯ぁ-んァ-ヶa-zA-Z_ー] \"!\"\n");
    //         match s {
    //             Some(s) => {
    //                 println!("Parsed String: \n{}\n Remaning String: {}", s.0, s.1);
    //             }
    //             None => {
    //                 println!("Could not parse.");
    //             }
    //         }
    //     }
    //     None => {
    //         println!("Could not find Rule.");
    //     }
    // }
}
