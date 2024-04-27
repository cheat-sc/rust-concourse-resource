use rstest::rstest;
use rust_concourse_resource::{
	put::PutResource,
	resource::Resource,
};
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
struct DummySource {
	param1: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
struct DummyVersion {}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
struct DummyParameters {
	param1: String,
}

#[rstest]
#[case::empty("", Err("EOF while parsing a value at line 1 column 0"))]
#[case::no_params(
	r#"
	{
		"source": {
			"param1": "test"
		}
	}
	"#,
	Ok(
		PutResource {
			params: None,
			source: DummySource {
				param1: "test".to_owned(),
			},
		}
	),
)]
#[case::with_params(
	r#"
	{
		"source": {
			"param1": "test"
		},
		"params": {
			"param1": "test"
		}
	}
	"#,
	Ok(PutResource {
		source: DummySource {
			param1: "test".to_owned(),
		},
		params: Some(DummyParameters {
			param1: "test".to_owned(),
		}),
	}),
)]
#[case::invalid_type(
	r#"
	{
		"source": {
			"param1": "test"
		},
		"params": {
			"param1": 10
		}
	}
	"#,
	Err("invalid type: integer `10`, expected a string at line 8 column 0")
)]
#[case::invalid_param(
	r#"
	{
		"source": {
			"param1": "test"
		},
		"params": {
			"param1": "test"
		},
		"param2": 10
	}
	"#,
	Err("unknown field `param2`, expected `source` or `params` at line 9 column 11")
)]
fn test_put_resource(
	#[case] input: &str,
	#[case] expect: Result<PutResource<Option<DummyParameters>, DummySource>, &str>,
) {
	let put = PutResource::<Option<DummyParameters>, DummySource>::from_reader(input.as_bytes());

	match (put, expect) {
		(Ok(actual), Ok(expect)) => assert_eq!(actual, expect),
		(Err(actual), Err(expect)) => assert_eq!(actual.to_string(), expect),
		(actual, expect) => panic!("{:?} != {:?}", actual, expect),
	}
}
