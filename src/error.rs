use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResourceError {
	#[error(transparent)]
	SerdeError(#[from] serde_json::Error),

	#[error("`{name}` is not set")]
	VarError {
		name: String,
		#[source]
		source: env::VarError,
	},
}
