use rstest::rstest;
use rust_concourse_resource::{
	check::{
		CheckResource,
		VersionBuilder,
	},
	resource::Resource,
};
use serde::{
	Deserialize,
	Serialize,
};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
struct DummySource {
	param1: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
struct DummyVersion {}

#[rstest]
#[case::no_version(
	r#"
	{
		"source": {
			"param1": "test"
		}
	}
	"#,
	Ok(CheckResource {
		source: DummySource{
			param1: "test".to_owned(),
		},
		version: None,
	})
)]
#[case::with_version(
	r#"
	{
		"source": {
			"param1": "test"
		},
		"version": {}
	}
	"#,
	Ok(CheckResource{
		source: DummySource{
			param1: "test".to_owned(),
		},
		version: Some(DummyVersion {}),
	}),
)]
#[case::no_source("{}", Err("missing field `source` at line 1 column 2"))]
#[case::invalid_param(
	r#"
	{
		"source": {
			"param1": "test"
		},
		"version": {},
		"foobar": 10,
	}
	"#,
	Err("unknown field `foobar`, expected `version` or `source` at line 7 column 11")
)]
fn check_get_resource(
	#[case] input: &'static str,
	#[case] expect: Result<CheckResource<DummySource, DummyVersion>, &str>,
) {
	let check = CheckResource::<DummySource, DummyVersion>::from_reader(input.as_bytes());

	match (check, expect) {
		(Ok(actual), Ok(expect)) => assert_eq!(actual, expect),
		(Err(actual), Err(expect)) => assert_eq!(actual.to_string(), expect),
		(actual, expect) => panic!("{:?} != {:?}", actual, expect),
	}
}

#[rstest]
fn test_version_builder() {
	let result = VersionBuilder::<DummyVersion>::default()
		.version(DummyVersion {})
		.version(DummyVersion {})
		.build();

	let expect = json!({
		"versions": [
			{},
			{}
		]
	});

	assert_eq!(result.unwrap(), serde_json::to_string_pretty(&expect).unwrap())
}
