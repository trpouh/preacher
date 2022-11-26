use serde::Deserialize;

use crate::{psalms::deacon::FileDeacon, worship::Worship, Psalm};

use super::deacon::FileDestination;

#[cfg(test)]
mod tests {

    use crate::psalms::{
        yaml::{YamlDeacon, YamlDeaconContext}
    };

    #[test]
    fn override_root() {
        let source = r#"
            foo: bar
        "#;

        let target = r#"
            bar: foo
        "#;

        let result = YamlDeacon::modify_yaml(YamlDeaconContext {
            appendix: target,
            source: source,
            yaml_path: Some("$"),
        });

        assert_eq!(result, target);
    }

    #[test]
    fn override_root_no_path() {
        let source = r#"
            foo: bar
        "#;

        let target = r#"
            bar: foo
        "#;

        let result = YamlDeacon::modify_yaml(YamlDeaconContext {
            appendix: target,
            source: source,
            yaml_path: None,
        });

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

        let result = YamlDeacon::modify_yaml(YamlDeaconContext {
            appendix: target,
            source: source,
            yaml_path: Some("$.obj"),
        });

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

        let result = YamlDeacon::modify_yaml(YamlDeaconContext {
            appendix: target,
            source: source,
            yaml_path: Some("$.foo.bar"),
        });

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

        let result = YamlDeacon::modify_yaml(YamlDeaconContext {
            appendix: target,
            source: source,
            yaml_path: Some("$.list"),
        });

        assert_eq!(result, expected);
    }
}

#[derive(Debug, Deserialize)]
pub struct YamlContext {
    file: FileDestination,
    yaml_path: Option<String>,
    yaml_string: String,
}

pub struct YamlPsalm {}

struct YamlDeaconContext<'a> {
    source: &'a str,
    appendix: &'a str,
    yaml_path: Option<&'a str>,
}

struct YamlDeacon {}

impl YamlDeacon {
    fn get_sanitized_path<'a>(yaml_path: &'a str) -> Vec<&'a str> {
        let mut paths: Vec<&str> = yaml_path.split(".").collect();

        if paths.starts_with(&["$"]) {
            paths.remove(0);
        }

        paths
    }

    fn modify_yaml(context: YamlDeaconContext) -> String {
        let source_unparsed: String = context.source.to_owned();

        let source: Result<serde_yaml::Value, _> = serde_yaml::from_str(context.source);
        let appendix: Result<serde_yaml::Value, _> = serde_yaml::from_str(context.appendix);

        if let Err(err) = source {
            println!("Error parsing yaml content: {}", err.to_string());
            return source_unparsed;
        }

        if let Err(err) = appendix {
            println!("Error parsing yaml content: {}", err.to_string());
            return source_unparsed;
        }

        let mut path = YamlDeacon::get_sanitized_path(context.yaml_path.unwrap_or("$"));
        let last = path.pop();

        if let None = last {
            return context.appendix.to_owned();
        }

        let mut root = source.unwrap();
        let mut current_value = &mut root;

        for sub_path in path {
            let numeric_sub = sub_path.parse::<usize>();

            match numeric_sub {
                Ok(index) => current_value = &mut current_value[index],
                Err(_) => current_value = &mut current_value[sub_path],
            };

            println!("current value: {:?}", current_value);
        }

        current_value[last.unwrap()] = appendix.unwrap();
        serde_yaml::to_string(&root).unwrap()
    }
}

impl Psalm<YamlContext> for YamlPsalm {
    fn invoke(context: &YamlContext, worship: &Worship) -> Result<String, String> {
        let file_deacon = FileDeacon::spawn(&context.file, &worship);

        // &file_deacon.load()?, &context.r#override, &context.yaml_path

        let contents = YamlDeacon::modify_yaml(YamlDeaconContext {
            source: &file_deacon.load()?,
            appendix: &context.yaml_string,
            yaml_path: context.yaml_path.as_ref().map(|x| &**x),
        });

        if let Err(err) = file_deacon.write(&contents) {
            return Err(format!("Couldn't write to file: {}", err.to_owned()));
        }

        Ok("OK".to_owned())
    }
}
