use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SyncResponseDevice {
    pub id: String,
    #[serde(rename = "type")] pub type_: String,
    pub name: Name,
    pub traits: Vec<String>,
    #[serde(rename = "willReportState")] pub will_report_state: bool,
    #[serde(rename = "roomHint")]
    #[serde(skip)]
    pub room_hint: Option<String>,
    #[serde(rename = "structureHint")]
    #[serde(skip)]
    pub structure_hint: Option<String>,
    #[serde(rename = "deviceInfo")]
    #[serde(skip)]
    pub device_info: Option<DeviceInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes:
        Option<SyncResponseDeviceAttributes>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct SyncResponseDeviceAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sceneReversible")]
    pub scene_reversible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "availableThermostatModes")]
    pub available_thermostat_modes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thermostatTemperatureUnit")]
    pub thermostat_temperature_unit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Name {
    #[serde(rename = "defaultName")]
    #[serde(skip)]
    pub default_name: Vec<String>,
    pub name: Option<String>,
    #[serde(skip)] pub nicknames: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    pub manifacturer: String,
    pub model: String,
    #[serde(rename = "hwVersion")] pub hw_version: String,
    #[serde(rename = "swVersion")] pub sw_version: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SyncResponse {
    #[serde(rename = "requestId")] pub request_id: String,
    pub payload: SyncResponsePayload,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SyncResponsePayload {
    #[serde(rename = "agentUserId")] pub agent_user_id: String,
    pub devices: Vec<SyncResponseDevice>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QueryResponse {
    #[serde(rename = "requestId")] pub request_id: String,
    pub payload: QueryResponsePayload,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QueryResponsePayload {
    pub devices: BTreeMap<String, Params>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Command {
    pub devices: Vec<RequestDevice>,
    pub execution: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RequestDevice {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Execution {
    pub command: String,
    pub params: Params,
}

// TODO: Imple From and To for specific Device instances.
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")] pub online: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub brightness: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")] pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thermostatTemperatureAmbient")]
    pub thermostat_temperature_ambient: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thermostatHumidityAmbient")]
    pub thermostat_humidity_ambient: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thermostatTemperatureSetpoint")]
    pub thermostat_temperature_setpoint: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thermostatTemperatureSetpointLow")]
    pub thermostat_temperature_setpoint_low: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thermostatTemperatureSetpointHigh")]
    pub thermostat_temperature_setpoint_high: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thermostatMode")]
    pub thermostat_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub deactivate: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Color {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<u64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spectrumRGB")]
    pub spectrum_rgb: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExecuteResponse {
    #[serde(rename = "requestId")] pub request_id: String,
    pub payload: ExecuteResponsePayload,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExecuteResponsePayload {
    #[serde(rename = "errorCode")] pub error_code: Option<String>,
    #[serde(rename = "debugString")] pub debug_string: Option<String>,
    pub commands: Vec<ExecuteResponseCommand>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExecuteResponseCommand {
    pub ids: Vec<String>,
    pub status: String,
    pub states: Params,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ActionRequest {
    #[serde(rename = "requestId")] pub request_id: String,
    pub inputs: Vec<ActionRequestInput>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ActionRequestInput {
    pub intent: String,
    pub payload: Option<ActionRequestPayload>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ActionRequestPayload {
    #[serde(default)] pub devices: Vec<RequestDevice>,
    #[serde(default)] pub commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AuthResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[test]
fn test_sync_request() {
    let json_req = r#"
{
  "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
  "inputs": [{
    "intent": "action.devices.SYNC"
  }]
}
"#;
    let parsed_req: ActionRequest = serde_json::from_str(&json_req).unwrap();
    let expected_req = ActionRequest {
        request_id: "ff36a3cc-ec34-11e6-b1a0-64510650abcf".to_string(),
        inputs: vec![
            ActionRequestInput {
                intent: "action.devices.SYNC".to_string(),
                payload: None,
            },
        ],
    };
    assert_eq!(expected_req, parsed_req);
}

#[test]
fn test_query_request() {
    let json_req = r#"
{
  "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
  "inputs": [{
    "intent": "action.devices.QUERY",
    "payload": {
      "devices": [{
        "id": "123",
        "customData": {
          "fooValue": 74,
          "barValue": true,
          "bazValue": "foo"
        }
      },{
        "id": "456",
        "customData": {
          "fooValue": 12,
          "barValue": false,
          "bazValue": "bar"
        }
      }]
    }
  }]
}
"#;
    let parsed_req: ActionRequest = serde_json::from_str(&json_req).unwrap();
    let expected_req = ActionRequest {
        request_id: "ff36a3cc-ec34-11e6-b1a0-64510650abcf".to_string(),
        inputs: vec![
            ActionRequestInput {
                intent: "action.devices.QUERY".to_string(),
                payload: Some(ActionRequestPayload {
                    devices: vec![
                        RequestDevice {
                            id: "123".to_string(),
                        },
                        RequestDevice {
                            id: "456".to_string(),
                        },
                    ],
                    commands: vec![],
                }),
            },
        ],
    };
    assert_eq!(expected_req, parsed_req);
}


#[test]
fn test_execute_request() {
    let json_req = r#"
{
  "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
  "inputs": [{
    "intent": "action.devices.EXECUTE",
    "payload": {
      "commands": [{
        "devices": [{
          "id": "123",
          "customData": {
            "fooValue": 74,
            "barValue": true,
            "bazValue": "sheepdip"
          }
        },{
          "id": "456",
          "customData": {
            "fooValue": 36,
            "barValue": false,
            "bazValue": "moarsheep"
          }
        }],
        "execution": [{
          "command": "action.devices.commands.OnOff",
          "params": {
            "on": true
          }
        }]
      }]
    }
  }]
}
"#;
    let parsed_req: ActionRequest = serde_json::from_str(&json_req).unwrap();
    let expected_req = ActionRequest {
        request_id: "ff36a3cc-ec34-11e6-b1a0-64510650abcf".to_string(),
        inputs: vec![
            ActionRequestInput {
                intent: "action.devices.EXECUTE".to_string(),
                payload: Some(ActionRequestPayload {
                    devices: vec![],
                    commands: vec![
                        Command {
                            devices: vec![
                                RequestDevice {
                                    id: "123".to_string(),
                                },
                                RequestDevice {
                                    id: "456".to_string(),
                                },
                            ],
                            execution: vec![
                                Execution {
                                    command: "action.devices.\
                                              commands.OnOff"
                                        .to_string(),
                                    params: Params { on: true },
                                },
                            ],
                        },
                    ],
                }),
            },
        ],
    };
    assert_eq!(expected_req, parsed_req);
}
