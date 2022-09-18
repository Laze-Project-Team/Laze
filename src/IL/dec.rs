use super::{stm, util};

enum Dec {
    Func(String, Vec<util::Type>, Vec<util::Type>, stm::Stm),
}
