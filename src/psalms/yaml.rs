use crate::psalms::prelude::{core::*, deacons::*};
#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct YamlContext {
    target: FileDestination,
    source: FileSource,
    yaml_path: Option<String>,
}

impl Psalm<YamlContext> for YamlPsalm {
    fn invoke(context: &YamlContext, worship: &Worship, vars: &PsalmVars) -> PsalmOutput {
        
        let file_deacon = FileDeacon::new(&context.target, worship);
        let source_deacon = context.source.to_deacon(worship, vars).unwrap();

        match file_deacon {
            Ok(deacon) => {

                let json_path = context.yaml_path.clone().unwrap_or_else(|| "$".to_owned());

                let map_yaml = |c: String| YamlPsalm::r#override(&c, &source_deacon.file_content().unwrap(), &json_path);

                let result = std::fs::read_to_string(deacon.path())
                    .map_err(|err| err.to_string())
                    .map(map_yaml)
                    .and_then(|contents| deacon.write(&contents));

                PsalmOutput::simple_from_result(context.info.clone(), result)
            }
            Err(err) => PsalmOutput::failed(context.info.clone(), err),
        }
    }
}

pub struct YamlPsalm {}

impl YamlPsalm {
    fn r#override(contents: &str, yaml_string: &str, path: &str) -> String {
        
        let parsed_input = if contents.is_empty() {
            Ok(serde_yaml::Value::default())
        } else {
            serde_yaml::from_str(contents)
        };

        let parsed_appendix: Result<serde_yaml::Value, _> = serde_yaml::from_str(yaml_string);

        if let Err(err) = parsed_input {
            println!("Error parsing yaml content: {}", err);
            return contents.to_owned();
        }

        let mut paths: Vec<&str> = path.split('.').collect();

        if paths.starts_with(&["$"]) {
            paths.remove(0);
        }

        let last = paths.pop();

        if last.is_none() {
            return yaml_string.to_owned();
        }

        let mut root = parsed_input.unwrap();
        let mut current_value = &mut root;

        for sub_path in paths {
            let numeric_sub = sub_path.parse::<usize>();

            match numeric_sub {
                Ok(index) => current_value = &mut current_value[index],
                Err(_) => current_value = &mut current_value[sub_path],
            };

            println!("current value: {:?}", current_value);
        }

        current_value[last.unwrap()] = parsed_appendix.unwrap();
        serde_yaml::to_string(&root).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::YamlPsalm;

    #[test]
    fn override_root() {
        let source = r#"
            foo: bar
        "#;

        let target = r#"
            bar: foo
        "#;

        let result = YamlPsalm::r#override(source, target, "$");
        assert_eq!(result, target);
    }

    #[test]
    fn override_sub() {
        let source = r#"
foo: bar
obj: test"#;

        let target = r#"
name: test
type: object"#;

        let expected = r#"foo: bar
obj:
  name: test
  type: object
"#;

        let result = YamlPsalm::r#override(source, target, "$.obj");
        assert_eq!(result, expected);
    }

    #[test]
    fn override_nested() {
        let source = r#"
foo:
  bar: a
obj: test"#;

        let target = r#"
name: test
type: object"#;

        let expected = r#"foo:
  bar:
    name: test
    type: object
obj: test
"#;

        let result = YamlPsalm::r#override(source, target, "$.foo.bar");
        assert_eq!(result, expected);
    }

    #[test]
    fn create_new() {
        let source = r#"a: b"#;

        let target = r#"[a,b]"#;

        let expected = r#"a: b
list:
- a
- b
"#;

        let result = YamlPsalm::r#override(source, target, "$.list");
        assert_eq!(result, expected);
    }

    #[test]
    fn empty_source() {
        let source = "";

        let target = r#"[a,b]"#;

        let expected = r#"list:
- a
- b
"#;

        let result = YamlPsalm::r#override(source, target, "$.list");
        assert_eq!(result, expected);
    }
}