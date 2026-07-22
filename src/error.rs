use std::{
	borrow::Cow,
	env,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResourceError {
	#[error(transparent)]
	SerdeError(#[from] serde_json::Error),

	#[error("`{name}` is not set")]
	VarError {
		name: Cow<'static, str>,
		#[source]
		source: env::VarError,
	},
}
