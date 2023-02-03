mod user_service {
    use jsonrpsee::{
        core::{RpcResult as Result, async_trait},
        proc_macros::rpc,
    };
    ///Server trait implementation for the `UserServiceTrait` RPC API.
    pub trait UserServiceTraitServer: Sized + Send + Sync + 'static {
        fn user_name(&self) -> Result<String>;
        ///Collects all the methods and subscriptions defined in the trait and adds them into a single `RpcModule`.
        fn into_rpc(self) -> jsonrpsee::RpcModule<Self> {
            let mut rpc = jsonrpsee::RpcModule::new(self);
            {
                let res = rpc
                    .register_method(
                        "useraa_userName",
                        |params, context| { context.user_name() },
                    );
                if true {
                    if !res.is_ok() {
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[
                                    "RPC macro method names should never conflict, this is a bug, please report it.",
                                ],
                                &[],
                            ),
                        )
                    }
                }
            }
            rpc
        }
    }
    pub struct UserService;
    impl UserServiceTraitServer for UserService {
        fn user_name(&self) -> Result<String> {
            Ok("hello".to_string())
        }
    }
}
