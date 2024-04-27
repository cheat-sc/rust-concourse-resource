use crate::resource::Resource;
use serde::{
	de::DeserializeOwned,
	Deserialize,
	Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PutResource<P, S> {
	pub source: S,
	pub params: P,
}

impl<P, S> Resource for PutResource<P, S>
where
	P: Serialize + DeserializeOwned,
	S: Serialize + DeserializeOwned,
{
}
