use serde::{/*Deserialize,*/ Serialize};
use serde_json::Value::Null;

#[derive(Serialize)]
pub struct RpcReq<P: Serialize > {
    id: &'static str,
    method: &'static str,
    jsonrpc: &'static str,
    params: P,
}

#[derive(Serialize)]
pub struct AuthParams {
    user: String,
    client: String,
    password: String,
}

impl AuthParams {
    pub fn new(user: &String, pw: &String, client: &Option<String>) -> AuthParams {
        AuthParams {
            //TODO maybe solve this without copy()
            user: user.clone(),
            //TODO add default value retrival over deamon startup
            //config instead of hardcoding it
            client: client.as_ref().unwrap_or(&String::from("untisd")).clone(),
            password: pw.clone(),
        }
    }
}

impl<P: Serialize> RpcReq<P> {
    pub fn new(ap: P, method: &'static str) -> Self {
        RpcReq {
            id: "ID",
            method: method,
            jsonrpc: "2.0",
            params: ap,
        }
    }
}

#[derive(Serialize)]
pub struct SimpleTimetableParams {
    pub id: i64,
    #[serde(rename = "type")]
    pub element_type: i64,
    #[serde(skip_serializing_if = "is_null")]
    pub startDate: Option<i64>,
    #[serde(skip_serializing_if = "is_null")]
    pub endDate: Option<i64>,
}

fn is_null<T: Serialize>(t: &T) -> bool {
    serde_json::to_value(t).unwrap_or(Null).is_null()
}

impl SimpleTimetableParams {
    pub fn new(id: i64, etype: i64, sdate: Option<i64>, edate: Option<i64>) -> Self {
        SimpleTimetableParams {
            id: id,
            element_type: etype,
            startDate: sdate,
            endDate: edate,
        }
    }
}
