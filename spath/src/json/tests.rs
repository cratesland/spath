use crate::{manifest_dir, Value};
use googletest::assert_that;
use googletest::matchers::eq;
use insta::assert_snapshot;

use serde_json::Value as JsonValue;

fn assert_testdata_identical(path: &str) -> String {
    let path = manifest_dir().join("testdata").join(path);
    let literal = std::fs::read_to_string(&path).unwrap();

    let json_value = serde_json::from_str::<JsonValue>(&literal).unwrap();
    let value = Value::from(json_value.clone());
    assert_that!(json_value, eq(&JsonValue::from(value.clone())));

    format!("{:?}", value)
}

#[test]
fn test_rfc_9535_example_convertion() {
    // §1.5 Figure 1
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-1.json"));
    // §2.3.1.3 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-2.json"));
    // §2.3.2.3 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-3.json"));
    // §2.3.3.3 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-4.json"));
    // §2.3.4.3 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-5.json"));
    // §2.3.5.3 Example 1
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-6.json"));
    // §2.3.5.3 Example 2
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-7.json"));
    // §2.5.1.3 Example 1
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-8.json"));
    // §2.5.2.3 Example 2
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-9.json"));
    // §2.6.1 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-10.json"));
}
