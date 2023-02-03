use jsonrpsee::{core::{RpcResult as Result, async_trait}, proc_macros::rpc};

#[rpc(server, namespace = "useraa")]
#[async_trait]
pub trait UserServiceTrait {
    #[method(name = "userName")]
    fn user_name(&self) -> Result<String>;
}

pub struct UserService;

#[async_trait]
impl UserServiceTraitServer for UserService {
    fn user_name(&self) -> Result<String> {
        Ok("hello".to_string())
    }
}
