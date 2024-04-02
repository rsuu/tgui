use crate::*;

use prost::Message;

#[derive(Debug)]
pub struct Any {}

impl Activity {
    pub fn send_any<I, O>(&self, req: I) -> Res<O>
    where
        I: Message + Default,
        O: Message + Default,
        items::method::Method: From<I>,
    {
        self.sr(items::method::Method::from(req))
    }
}
