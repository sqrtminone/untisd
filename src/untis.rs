use crate::rpc::{AuthParams, RpcReq};
use futures::executor::block_on;
use reqwest::blocking::Client;
use anyhow::{Result,anyhow};
use hyper::header::{Headers, Cookie};

use serde::{Deserialize, Serialize};
use serde_json;

//structure that represents an Untis context, eg credentials
//and the server location as well as some session specific
//values
//#[derive(Serialize)]
pub struct Untis {
    //composed of school and server
    untis_url: String,
    //student/user login credentials
    //TODO DEBUG un-pub later
    pub user: String,
    pub password: String,
    //values for url generation and parsing less output messages
    school: String,
    server: String,
    pub session: Option<UntisSession>,
}

//TODO move to src/rpc.rs
#[derive(Deserialize)]
pub struct RpcRespone<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: T,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UntisSession {
    pub session_id: String,
    #[serde(rename = "klasseId")]
    pub class_id: i64,
    pub person_id: i64,
    pub person_type: i64,
}

impl Untis {
    pub fn new(school: &str, server: &str, user: &str, pw: &str) -> Self {
        Untis {
            untis_url: format!(
                           "https://{}.webuntis.com/WebUntis/jsonrpc.do?school={}",
                           server,
                           school
            ),
            user: user.to_owned(),
            password: pw.to_owned(),
            school: school.to_owned(),
            server: server.to_owned(),
            session: None,
        }
    }
    //TODO add organized information outputting like for
    //daemon stuff/logging 
    //HACKY info dumping function
    pub fn conntextion_info(self) {
        println!("url={}", self.untis_url);
    }

    pub fn login(&mut self) {
        let ap = AuthParams::new( &self.user,
                                  &self.password,
                                  &None);
        let req = RpcReq::new( ap, "authenticate");
        //println!("{}", serde_json::to_string(&req).unwrap());
        //TODO make more use of that its asyncronous
        let res = Client::new()
            .post(&self.untis_url)
            .json( &req )
            .send()
            .unwrap();

        let status = res.status();
        if status.is_success() {
            let res_str = res.text().unwrap();
            let res: RpcRespone<UntisSession> = serde_json::from_str(&res_str).unwrap();
            self.session = Some(res.result);
        }

    }

    pub fn issue_request_parameterized<T: Serialize>(&self,request: &'static str, params: T) -> Result<String> {
        if self.session.is_none() {
            //anyhow error
            return Err(anyhow!("no active session"));
        }

        let rpc_request = RpcReq::new(params, request);

        let mut cookie = Cookie::new();
        //cookie.append("JSESSIONID", self.session.unwrap().session_id.clone());

        let res = Client::new()
            .post(&self.untis_url)
            //TODO well is there a more integrated way to do this ?!
            .header("cookie", format!("JSESSIONID={}",self.session.as_ref().unwrap().session_id))
            .json( &rpc_request )
            .send()
            .unwrap();

        //println!("\n\njson = {:?}", serde_json::to_string(&rpc_request).unwrap());
        //println!("\n\nauth = {:?}", self.session.unwrap().session_id.clone());
        let tex = res.text()?;
        //println!("res -> {:?}", tex);
        Ok(tex)
    }
    
    pub fn issue_request(&self,request: &'static str) -> Result<String> {
        if self.session.is_none() {
            //anyhow error
            return Err(anyhow!("no active session"));
        }

        let rpc_request = RpcReq::new((), request);

        let mut cookie = Cookie::new();
        //cookie.append("JSESSIONID", self.session.unwrap().session_id.clone());

        let res = Client::new()
            .post(&self.untis_url)
            //TODO well is there a more integrated way to do this ?!
            .header("cookie", format!("JSESSIONID={}",self.session.as_ref().unwrap().session_id))
            .json( &rpc_request )
            .send()
            .unwrap();

        //println!("\n\njson = {:?}", serde_json::to_string(&rpc_request).unwrap());
        //println!("\n\nauth = {:?}", self.session.unwrap().session_id.clone());
        //println!("res -> {:?}", res.text().unwrap());
        Ok(res.text()?)
    }
}
