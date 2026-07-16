use crate::resource;
use serde::{
	de::DeserializeOwned,
	Deserialize,
	Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request<V, S, P> {
	pub version: V,
	pub source: S,
	pub params: Option<P>,
}

impl<V, S, P> resource::Resource for Request<V, S, P>
where
	S: Serialize + DeserializeOwned,
	V: Serialize + DeserializeOwned,
	P: Serialize + DeserializeOwned,
{
}

pub use resource::Response;
