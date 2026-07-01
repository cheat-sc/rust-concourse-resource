use crate::resource;
use serde::{
	de::DeserializeOwned,
	Deserialize,
	Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Resource<S, V> {
	pub version: Option<V>,
	pub source: S,
}

impl<S, V> resource::Resource for Resource<S, V>
where
	S: Serialize + DeserializeOwned,
	V: Serialize + DeserializeOwned,
{
}

#[derive(Debug, Serialize, Default)]
pub struct VersionBuilder<V> {
	versions: Vec<V>,
}

impl<V> VersionBuilder<V>
where
	V: Serialize + Default,
{
	pub fn version(mut self, version: V) -> Self {
		self.versions.push(version);
		self
	}

	pub fn build(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string_pretty(self)
	}
}
