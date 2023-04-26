use super::error::{CompileError, CompileError_};

pub fn handle_error(errors: Vec<CompileError>) -> bool {
    let mut error_result = false;
    for error in errors {
        error.print();
        if let CompileError_::Error(_, _) = *error {
            error_result = true;
        }
    }
    return error_result;
}
