
// May move to serenity::builder::* on next update

use serenity::utils::builder::{CreateEmbed, CreateEmbedField, CreateEmbedAuthor, CreateEmbedFooter};
use serenity::utils::Colour;
use serde_json;
use serde_json::Error;

#[derive(Serialize, Deserialize, Debug)]
struct JsonEmbedField {
    pub title: Option<String>,
    pub value: Option<String>,
    pub inline: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonEmbedColor(pub i32);

#[derive(Serialize, Deserialize, Debug)]
struct JsonEmbedAuthor {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub url: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonEmbedImage {
    pub url: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonEmbedFooter {
    pub url: Option<String>,
    pub text: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonEmbedThumbnail(pub JsonEmbedImage);

#[derive(Serialize, Deserialize, Debug)]
struct JsonEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<JsonEmbedColor>,
    pub fields: Option<Vec<JsonEmbedField>>,
    pub author: Option<JsonEmbedAuthor>,
    pub image: Option<JsonEmbedImage>,
    pub timestamp: Option<String>,
    pub footer: Option<JsonEmbedFooter>
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct JsonToDiscordEmbedObject;

#[allow(dead_code)]
impl JsonToDiscordEmbedObject {
    pub fn new() -> JsonToDiscordEmbedObject {
        JsonToDiscordEmbedObject{}
    }

    pub fn parse(&self, s: &str) -> Result<CreateEmbed, Error> {
        let psd: JsonEmbed = serde_json::from_str(s)?;
        let mut xr: CreateEmbed = CreateEmbed::default();
        if let Some(title) = psd.title {
            xr = xr.title(title.as_str());
        }

        if let Some(desc) = psd.description {
            xr = xr.description(desc.as_str());
        }

        if let Some(color) = psd.color {
            let tcolor = Colour::from(color.0);
            xr = xr.colour(tcolor);
        }

        if let Some(fields) = psd.fields {
            for field in fields {
                xr = xr.field(move |_| {
                    let mut field_builder: CreateEmbedField = CreateEmbedField::default();
                    if let Some(ftitle) = field.title { field_builder = field_builder.name(ftitle.as_str()); }
                    if let Some(fvalue) = field.value { field_builder = field_builder.value(fvalue.as_str()); }
                    if let Some(finline) = field.inline { field_builder = field_builder.inline(finline); }
                    field_builder
                });
            }
        }

        if let Some(author) = psd.author {
            xr = xr.author(move |_| {
                let mut author_builder: CreateEmbedAuthor = CreateEmbedAuthor::default();
                if let Some(aname) = author.name { author_builder = author_builder.name(aname.as_str()); }
                if let Some(aicon) = author.icon { author_builder = author_builder.icon_url(aicon.as_str()); }
                if let Some(aurl) = author.url { author_builder = author_builder.url(aurl.as_str()); }
                author_builder
            });
        }

        if let Some(image) = psd.image {
            if let Some(iurl) = image.url {
                xr = xr.image(iurl.as_str())
            }
        }

        if let Some(tstamp) = psd.timestamp {
            xr = xr.timestamp(tstamp);
        }

        if let Some(footer) = psd.footer {
            xr = xr.footer(move |_| {
                let mut footer_builder: CreateEmbedFooter = CreateEmbedFooter::default();
                if let Some(furl) = footer.url { footer_builder = footer_builder.icon_url(furl.as_str()); }
                if let Some(ftext) = footer.text { footer_builder = footer_builder.text(ftext.as_str()); }
                footer_builder
            });
        }

        Ok(xr)
    }
}
