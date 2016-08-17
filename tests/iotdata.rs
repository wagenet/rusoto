#![cfg(feature = "iot-data")]

extern crate rusoto;
extern crate env_logger;
#[macro_use]
extern crate log;
use rusoto::iotdata::{IotDataPlaneClient, GetThingShadowRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_get_thing_shadow() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = IotDataPlaneClient::new(credentials, Region::UsEast1);
    let mut request = GetThingShadowRequest::default();
    request.thing_name = "iotbutton_G030JF050464C6PF".to_string();

    debug!("BLAH BLAH BLAH {:#?}", client.get_thing_shadow(&request).unwrap());
}
