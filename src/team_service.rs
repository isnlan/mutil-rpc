use jsonrpsee::{core::{RpcResult as Result, async_trait}, proc_macros::rpc};

#[rpc(server, namespace = "team")]
#[async_trait]
pub trait TeamServiceTrait {
    #[method(name = "teamName")]
    fn team_name(&self) -> Result<String>;
}

pub struct TeamService;

#[async_trait]
impl TeamServiceTraitServer for TeamService {
    fn team_name(&self) -> Result<String> {
        Ok("hello team".to_string())
    }
}
