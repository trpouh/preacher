use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;
use tera::{Context, Tera};

#[derive(Deserialize, Debug)]
pub struct Templating {
    pub flavor: TemplatingLanguage,
}

impl Templating {
    pub fn to_deacon<'a>(&'a self, source: &'a PathBuf) -> TemplateDeacon<'a> {
        TemplateDeacon {
            templating: self,
            source,
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum TemplatingLanguage {
    J2,
    Handlebars,
}
pub struct TemplateDeacon<'a> {
    source: &'a PathBuf,
    templating: &'a Templating,
}

impl<'a> TemplateDeacon<'a> {
    pub fn template(&self, vars: &HashMap<String, String>) -> Result<String, String> {
        match &self.templating.flavor {
            TemplatingLanguage::J2 => {
                let tpl = "default";

                let context = Context::from_serialize(vars).unwrap();

                let mut tera = Tera::default();

                tera.add_template_file(self.source, Some(tpl))
                    .and_then(|_| tera.render(tpl, &context))
                    .map_err(|err| err.to_string())
            }
            TemplatingLanguage::Handlebars => {
                todo!("Yet to be implemented")
            }
        }
    }
}
