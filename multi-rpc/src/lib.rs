pub trait ServiceFactory {
    fn register<S: Service>(&self, svc:S ) -> Result<(), ()>;
    fn call() -> Result<Vec<u8>, ()>;
}


pub trait Service {

}


