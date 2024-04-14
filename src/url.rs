use crate::error::ResourceError;
use serde_json::{
	Map,
	Value,
};
use std::env;

fn set_parameters(instance_vars: &Map<String, Value>, parent: Option<&String>) -> String {
	let mut params = Vec::<String>::new();

	/* NOTE: instance vars always are dictionary */
	for (key, value) in instance_vars.iter() {
		let param = if let Some(parent) = parent {
			format!("{}.{}", parent, key)
		} else {
			key.to_owned()
		};

		if let Some(obj) = value.as_object() {
			params.push(set_parameters(obj, Some(&param)))
		} else {
			params.push(format!("vars.{}={}", &param, &value).replace('\"', "%22"));
		}
	}

	let mut res = params.join("&");

	if !params.is_empty() && parent.is_none() {
		res.insert(0, '?');
	}

	res
}

pub fn build_concourse_url() -> Result<String, ResourceError> {
	Ok(format!(
		"{}/teams/{}/pipelines/{}/jobs/{}/builds/{}{}",
		env::var("ATC_EXTERNAL_URL").map_err(|err| ResourceError::VarError {
			name: "ATC_EXTERNAL_URL".to_owned(),
			source: err
		})?,
		env::var("BUILD_TEAM_NAME").map_err(|err| ResourceError::VarError {
			name: "BUILD_TEAM_NAME".to_owned(),
			source: err
		})?,
		env::var("BUILD_PIPELINE_NAME").map_err(|err| ResourceError::VarError {
			name: "BUILD_PIPELINE_NAME".to_owned(),
			source: err
		})?,
		env::var("BUILD_JOB_NAME").map_err(|err| ResourceError::VarError {
			name: "BUILD_JOB_NAME".to_owned(),
			source: err
		})?,
		env::var("BUILD_NAME").map_err(|err| ResourceError::VarError {
			name: "BUILD_NAME".to_owned(),
			source: err
		})?,
		if let Ok(instance_vars) = env::var("BUILD_PIPELINE_INSTANCE_VARS") {
			if let Ok(instance_vars) = serde_json::from_str(instance_vars.as_str()) {
				set_parameters(&instance_vars, None)
			} else {
				"".to_owned()
			}
		} else {
			"".to_owned()
		},
	))
}

#[cfg(test)]
mod tests {
	use super::*;
	use envtestkit;
	use rstest::rstest;
	use serde_json::json;

	#[rstest]
	#[case::empty(json!({}), "")]
	#[case::boolean(json!({ "a": true }), "?vars.a=true")]
	#[case::integer(json!({ "a": 0 }), "?vars.a=0")]
	#[case::string(json!({ "a": "s" }), "?vars.a=%22s%22")]
	#[case::nested(
		json!({
			"a": {
				"a": 0,
				"b": true
			}
		}),
		"?vars.a.a=0&vars.a.b=true",
	)]
	#[case::nested_nested(
		json!({
			"a": {
				"a": 0,
				"b": {
					"c": 0,
					"d": {
						"e": 0
					}
				}
			}
		}),
		"?vars.a.a=0&vars.a.b.c=0&vars.a.b.d.e=0",
	)]
	#[case::complex(
		json!({
			"a": 0,
			"b": {
				"a": 0,
				"b": true
			},
			"c": "0-0"
		}),
		"?vars.a=0&vars.b.a=0&vars.b.b=true&vars.c=%220-0%22",
	)]
	fn check_set_parameters(#[case] json: Value, #[case] expect: &str) {
		assert_eq!(set_parameters(json.as_object().unwrap(), None), expect);
	}

	#[rstest]
	#[case::empty(None, "http://localhost/teams/test/pipelines/test/jobs/test/builds/1")]
	#[case::empty(Some(json!({})), "http://localhost/teams/test/pipelines/test/jobs/test/builds/1")]
	#[case::boolean(
		Some(json!({ "a": true })),
		"http://localhost/teams/test/pipelines/test/jobs/test/builds/1?vars.a=true",
	)]
	#[case::integer(
		Some(json!({ "a": 0 })),
		"http://localhost/teams/test/pipelines/test/jobs/test/builds/1?vars.a=0",
	)]
	#[case::string(
		Some(json!({ "a": "s" })),
		"http://localhost/teams/test/pipelines/test/jobs/test/builds/1?vars.a=%22s%22",
	)]
	#[case::nested(
		Some(json!({
			"a": {
				"a": 0,
				"b": true
			}
		})),
		"http://localhost/teams/test/pipelines/test/jobs/test/builds/1?vars.a.a=0&vars.a.b=true",
	)]
	#[case::nested_nested(
		Some(json!({
			"a": {
				"a": 0,
				"b": {
					"c": 0,
					"d": {
						"e": 0
					}
				}
			}
		})),
		"http://localhost/teams/test/pipelines/test/jobs/test/builds/1?vars.a.a=0&vars.a.b.c=0&vars.a.b.d.e=0",
	)]
	#[case::complex(
		Some(json!({
			"a": 0,
			"b": {
				"a": 0,
				"b": true
			},
			"c": "0-0"
		})),
		"http://localhost/teams/test/pipelines/test/jobs/test/builds/1?vars.a=0&vars.b.a=0&vars.b.b=true&vars.c=%220-0%22",
	)]
	fn check_url_builder(#[case] instance_vars: Option<Value>, #[case] expect: String) {
		struct Var<'a> {
			key: &'a str,
			value: &'a str,
		}

		let _lock = envtestkit::lock::lock_test();
		let _vars = [
			Var {
				key: "ATC_EXTERNAL_URL",
				value: "http://localhost",
			},
			Var {
				key: "BUILD_TEAM_NAME",
				value: "test",
			},
			Var {
				key: "BUILD_PIPELINE_NAME",
				value: "test",
			},
			Var {
				key: "BUILD_JOB_NAME",
				value: "test",
			},
			Var {
				key: "BUILD_NAME",
				value: "1",
			},
		]
		.map(|v| envtestkit::set_env(v.key.into(), v.value));
		let _var =
			instance_vars.map(|json| envtestkit::set_env("BUILD_PIPELINE_INSTANCE_VARS".into(), json.to_string()));

		let res = build_concourse_url();
		assert!(res.is_ok());
		assert_eq!(res.unwrap(), expect);
	}
}
