//! Interfaces with PubChem's PUG REST API.
//!
//! https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest

use std::{fmt, marker::PhantomData};

use reqwest::{blocking::Client as HttpClient, header::HeaderMap, Result};
use serde_json::{Map as JsonMap, Value as JsonValue};
use url::{PathSegmentsMut, Url};

pub struct Client {
    http: HttpClient,
    url: Url,
}

pub struct Request<'a, Domain> {
    client: &'a Client,
    url: Url,
    _marker: PhantomData<&'a Domain>,
}

// Marker types
pub struct CompoundDomain;
pub struct Name;

#[derive(Debug)]
pub enum CompoundStringProperty {
    IUPACName,
}
pub use CompoundStringProperty::*;

pub trait Identifier<Domain> {
    fn fmt_path(&self, path: &mut PathSegmentsMut);
}

pub trait CompoundProperty {
    type Value;

    fn fmt_path(&self, path: &mut PathSegmentsMut) {
        let mut s = String::new();
        self.fmt(&mut s).unwrap();
        path.push(&s);
    }

    fn fmt(&self, f: &mut dyn fmt::Write) -> fmt::Result;

    fn extract(&self, map: &JsonMap<String, JsonValue>) -> Self::Value;
}

impl Client {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        Ok(Client {
            http: HttpClient::builder().default_headers(headers).build()?,
            url: "https://pubchem.ncbi.nlm.nih.gov/rest/pug".parse().unwrap(),
        })
    }

    fn request(&self) -> Request<()> {
        Request {
            client: self,
            url: self.url.clone(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Request<'a, T> {
    fn extend(mut self, f: impl FnOnce(&mut PathSegmentsMut)) -> Self {
        {
            let mut path = self.url.path_segments_mut().unwrap();
            f(&mut path);
        }

        self
    }

    fn cast<U>(self) -> Request<'a, U> {
        Request {
            client: self.client,
            url: self.url,
            _marker: PhantomData,
        }
    }
}

impl Client {
    pub fn compound<ID: Identifier<CompoundDomain>>(&self, id: ID) -> Request<CompoundDomain> {
        self.request()
            .extend(|path| {
                path.push("compound");
                id.fmt_path(path);
            })
            .cast()
    }
}

impl Request<'_, CompoundDomain> {
    pub fn property<Prop: CompoundProperty>(self, prop: Prop) -> Result<Prop::Value> {
        #[derive(serde::Deserialize)]
        struct Res {
            #[serde(rename = "PropertyTable")]
            prop_table: PropTable,
        }

        #[derive(serde::Deserialize)]
        struct PropTable {
            #[serde(rename = "Properties")]
            props: Vec<JsonMap<String, JsonValue>>,
        }

        let req = self.extend(|path| {
            path.push("property");
            prop.fmt_path(path);
        });
        let res: Res = req.client.http.get(req.url).send()?.json()?;

        let value = prop.extract(&res.prop_table.props[0]);
        Ok(value)
    }
}

impl Identifier<CompoundDomain> for (Name, &str) {
    fn fmt_path(&self, path: &mut PathSegmentsMut) {
        let (_, name) = self;
        path.push("name").push(name);
    }
}

impl CompoundProperty for CompoundStringProperty {
    type Value = String;

    fn fmt(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "{self:?}")
    }

    fn extract(&self, map: &JsonMap<String, JsonValue>) -> Self::Value {
        let key = format!("{self:?}");
        map[&key].as_str().unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_iupac_name() {
        let client = Client::new().unwrap();

        let iupac_name = client
            .compound((Name, "Caffeine"))
            .property(IUPACName)
            .unwrap();

        assert_eq!(iupac_name, "1,3,7-trimethylpurine-2,6-dione");
    }
}
