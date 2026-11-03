use super::{BACK as BACK_MARK, Request};

fn reply_to(request: &(Request, bool)) -> env::PieToken {
    request.0.back
}

#[mold::contract(request = Request, response = system_api::loader::Said, mark = BACK_MARK, back = reply_to)]
pub struct Call;
