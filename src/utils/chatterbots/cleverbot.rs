#[allow(dead_code)]
static API_BASE_URL: &'static str = "https://www.cleverbot.com/getreply";

use std::collections::HashMap;
use reqwest;
use serde_json::Error;
use serde_json;
use std::io::Read;
use url::form_urlencoded;

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


        let url = API_BASE_URL.to_owned() + "?" + params_to_www_form_url_encoded(&params).as_str();

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

fn params_to_www_form_url_encoded<'a>(params: &'a HashMap<&str,&str>) -> String {
    let mut encoder = form_urlencoded::Serializer::new(String::new());
    for (name, val) in params {
        encoder.append_pair(name, val);
    }
    encoder.finish()
}

#[cfg(tests)]
mod tests {
    #[test]
    fn params_to_www_form_url_encoded_test() {
        // Empty Test
        let v = HashMap::new();
        assert_eq!(params_to_www_form_url_encoded(&v), "");
        // Create DummyArray
        let v: HashMap<&str, &str> = [
            ("param", "data"),
            ("qubit_dfu", "meta")
        ].iter().cloned().collect();

        // Create & split testing vectors
        let lf = params_to_www_form_url_encoded(&v);
        let rf = String::from("param=data&qubit_dfu=meta");
        let left: Vec<&str> = lf.split('&').collect();
        let right: Vec<&str> = rf.split('&').collect();

        // Compare Vectors
        for t in right {
            if !left.contains(&t) {
                assert!(false, format!("Missing {}", t))
            }
        }
    }
}