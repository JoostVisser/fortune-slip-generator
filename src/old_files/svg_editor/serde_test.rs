use serde_json::{json, Value};

fn main() {
    let json_str = r#"
        {
            "name": "example",
            "nested_obj": {
                "nested_prop": 42
            },
            "other_prop": {
                "some_value": "hello"
            }
        }
    "#;

    let mut json_data: Value = serde_json::from_str(json_str).unwrap();

    let nested_prop = json_data.pointer_mut("/nested_obj/nested_prop").unwrap();
    *nested_prop = json!(nested_prop.as_u64().unwrap() + 1);

    let nested_prop = &mut json_data["nested_obj"]["nested_prop"];
    *nested_prop = json!(nested_prop.as_u64().unwrap() + 1);

    let serialized = serde_json::to_string_pretty(&json_data).unwrap();
    println!("{}", serialized);
}

#[cfg(test)]
mod tests {
    use super::main;

    const SVG_EXAMPLE: &str = "<svg height='100' width='100'>
                                <tspan x='12' y='24'>Cat</tspan>
                               </svg>";

    #[test]
    fn test_json_serde() {
        let temp_file = test_utils::create_temp_file("temp.svg", SVG_EXAMPLE);

        main();
    }
}
