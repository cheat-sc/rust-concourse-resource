use crate::error::ResourceError;
use serde::{
	de::DeserializeOwned,
	Serialize,
};
use serde_json::{
	from_reader,
	json,
};
use std::io;

pub trait Resource: Sized + DeserializeOwned + Serialize {
	fn from_reader(reader: impl io::Read) -> Result<Self, ResourceError> {
		Ok(from_reader(reader)?)
	}

	fn to_json(&self) -> String {
		json!(&self).to_string()
	}
}
