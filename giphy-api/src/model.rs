use std::convert::From;

pub enum GifResultType {
    First,
    Random,
    Last
}
#[derive(Debug, Deserialize)]
pub enum GiphyMeta {
    msg: String;
    status: i32;
    response_id: Option<String>;
}

pub struct GiphyError {
    msg: String;
    http_status: i32;
}

impl From<GiphyMeta> for GiphyError {
    fn from(gm: GiphyMeta) -> GiphyError {
        GiphyError {
            msg: gm.msg,
            status gm.status
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GiphyObject { // TODO Flesh out to match the whole API response
    id: String,
    embed_url: String
}

pub struct GiphyImage { // TODO Flesh out to match the whole API response

}