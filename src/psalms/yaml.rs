use serde::Deserialize;

use crate::{Psalm, psalms::deacons::file::FileDeacon, worship::Worship};

use super::{deacons::file::FileDestination, PsalmOutput, PsalmInfo};

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

}

#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct YamlContext {
    file: FileDestination,

    path: String,
    
    r#override: String
}

impl Psalm<YamlContext> for YamlPsalm {

    fn invoke(context: &YamlContext, worship: &Worship) -> Result<PsalmOutput, String> {

        let file_deacon = FileDeacon::spawn(&context.file, worship);

        let contents = YamlPsalm::r#override(&file_deacon.load()?, &context.r#override, &context.path);

        file_deacon.write(&contents);

        Ok(PsalmOutput::empty(context.info.clone()))
    }
}

pub struct YamlPsalm {}

impl YamlPsalm {
    fn r#override(contents: &str, yaml_string: &str, path: &str) -> String {

        let parsed_input: Result<serde_yaml::Value, _> = serde_yaml::from_str(contents);
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
                Ok(index) => {
                    current_value = &mut current_value[index]
                },
                Err(_) => {
                    current_value = &mut current_value[sub_path]
                }
            };

            println!("current value: {:?}", current_value);
        }

        current_value[last.unwrap()] = parsed_appendix.unwrap();
        serde_yaml::to_string(&root).unwrap()

    }
}