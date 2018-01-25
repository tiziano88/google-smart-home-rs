use device::Device;
use futures::Future;
use futures::stream::Stream;
use google_actions::{ActionRequest, ActionRequestInput, ExecuteResponseCommand, Name, Params,
                     SyncResponseDevice, SyncResponseDeviceAttributes};
use hyper::{Client, Method, Request};
use tokio_core::reactor::Core;
use serde_json;

pub struct Proxy {
    pub id: String,
    pub target_url: String,
}

impl Device for Proxy {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn sync(&self) -> Option<SyncResponseDevice> {
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());
        let uri = self.target_url.parse().unwrap();

        let mut req = Request::new(Method::Post, uri);
        let action = ActionRequest {
            request_id: "lksdjflksdjkl".to_string(),
            inputs: vec![
                ActionRequestInput {
                    intent: "action.devices.SYNC".to_string(),
                    payload: Option::None,
                },
            ],
        };
        req.set_body(serde_json::to_string(&action).unwrap());

        let work = client.request(req).and_then(|res| {
            res.body().concat2().and_then(move |body| {
                let r = serde_json::from_slice::<SyncResponseDevice>(&body).unwrap();
                Ok(r)
            })
        });
        let res = core.run(work).unwrap();
        Option::None
    }

    fn query(&self) -> Option<Params> {
        Option::None
    }

    fn execute(&mut self, params: &Params) -> Option<ExecuteResponseCommand> {
        Option::None
    }
}
