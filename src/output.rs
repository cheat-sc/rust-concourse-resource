use crate::resource;
use serde::{
	de::DeserializeOwned,
	Deserialize,
	Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request<S, P> {
	pub source: S,
	pub params: Option<P>,
}

impl<S, P> resource::Resource for Request<S, P>
where
	S: Serialize + DeserializeOwned,
	P: Serialize + DeserializeOwned,
{
}

pub use resource::Response;
