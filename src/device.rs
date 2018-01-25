use google_actions::{ExecuteResponseCommand, Params, SyncResponseDevice};

pub trait Device: Send + Sync {
    fn id(&self) -> String;
    fn sync(&self) -> Option<SyncResponseDevice>;
    fn query(&self) -> Option<Params>;
    fn execute(&mut self, &Params) -> Option<ExecuteResponseCommand>;
}
