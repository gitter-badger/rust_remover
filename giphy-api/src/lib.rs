static API_SEARCH_URL: &'static str = "https://api.giphy.com/v1/gifs/search?api_key={api_key}&q={query}&limit={limit}&offset={offset}&rating=G&lang=en";
static API_RANDOM_URL: &'static str = "https://api.giphy.com/v1/gifs/random?api_key={api_key}&tag={tag}&rating=G";
static API_GETIMG_URL: &'static str = "https://api.giphy.com/v1/gifs/{gif_id}?api_key={api_key}";

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_jsob;
extern crate url;
extern crate reqwest;

mod model;

pub struct GiphyConnector {
    api_key: String
}

impl GiphyConnector {
    pub fn new(api_key: String) -> GiphyConnector {
        GiphyConnector {
            api_key: api_key
        }
    }
    /// Searches for the given term and returns an image.
    pub fn search(term: &str, resultType: model::GifResultType, limit: Option<u32>, offset: Option<u32>) -> Result<model::GiphyObject {

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
