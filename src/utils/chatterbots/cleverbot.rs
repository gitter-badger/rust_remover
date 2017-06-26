#[allow(dead_code)]
static API_BASE_URL: &'static str = "https://www.cleverbot.com/getreply";

use utils;
use std::collections::HashMap;
use reqwest;
use serde_json::Error;
use serde_json;
use std::io::Read;

#[derive(Debug)]
pub struct Cleverbot {
    cbsession: CleverbotSession,
    api_key: String,
    sesscs: String
}
#[allow(dead_code)]
impl Cleverbot {
    pub fn new(api_key: String) -> Cleverbot {
        Cleverbot {
            cbsession: CleverbotSession::new(),
            sesscs: String::new(),
            api_key: api_key
        }
    }

    pub fn think(&mut self, text: String) -> Result<String, String> {
        let mut params: HashMap<&str, &str> = [
            ("key", self.api_key.as_str()),
            ("input", text.as_str())
        ].iter().cloned().collect();

        self.sesscs = self.cbsession.cs.clone();
        if !self.sesscs.is_empty() {
            params.insert("cs", self.sesscs.as_str());
        }


        let url = API_BASE_URL.to_owned() + "?" + utils::http_utils::params_to_www_form_url_encoded(&params).as_str();

        let mut response = reqwest::get(url.as_str()).unwrap();

        if !response.status().is_success() {
            return Err(String::from("The response did not return an 200"));
        }

        let mut rsp = String::new();
        let _ = response.read_to_string(&mut rsp);

        if let Ok(new_session) = Cleverbot::get_next_session(rsp) {
            self.cbsession = new_session;
            Ok(self.cbsession.output.clone())
        } else {
            return Err(String::from("JSON PARSE ERR !"))
        }
    }

    fn get_next_session(json: String) -> Result<CleverbotSession, Error> {
        let session: CleverbotSession = serde_json::from_str(json.as_str())?;
        Ok(session)
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct CleverbotSession {
    pub cs: String,
    pub interaction_count: String,
    pub input: String,
    pub output: String,
    pub conversation_id: String,
}


impl CleverbotSession {
    pub fn new() -> CleverbotSession {
        CleverbotSession {
            cs: String::new(),
            interaction_count: String::new(),
            input: String::new(),
            output: String::new(),
            conversation_id: String::new()
        }
    }
}

 // Stubs
/*
 pub struct Cleverbot;

 impl Cleverbot {
     pub fn new(api_key: String) -> Cleverbot {
         Cleverbot{}
     }
     pub fn think(&mut self, text: String) -> Result<String, String> {
         if (text.len() == 0) {
             return Err(String::from("This is the Error Method stub of Cleverbot"));
         }
         Ok(String::from("This is the Ok Method stub of Cleverbot"))
     }
 }*/