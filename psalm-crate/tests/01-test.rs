#[cfg(test)]
mod tests {

    use psalmer::{psalm_context};
    use serde::{Deserialize};

    #[test]
    fn test_functionality() {
        #[derive(serde::Deserialize)]
        pub struct PsalmInfo {
            pub id: String,
        }

        #[psalm_context]
        #[derive(Deserialize)]
        struct B {
            #[serde(flatten)]
            pub a: String,
        }

        let b = B {
            a: "text".to_owned(),
            info: Some(PsalmInfo {
                id: "my_id".to_owned(),
            }),
        };

        assert_eq!(b.info.unwrap().id, "my_id".to_owned());
    }
}
