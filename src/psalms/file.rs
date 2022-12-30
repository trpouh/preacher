use crate::psalms::deacons::file::template::Templating;
use crate::psalms::prelude::{core::*, deacons::*};

pub struct FilePsalm {}

#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct FileContext {
    source: FileSource,
    target: FileDestination,
    template: Option<Templating>,
}

impl Psalm<FileContext> for FilePsalm {
    fn invoke(context: &FileContext, worship: &Worship, vars: &PsalmVars) -> PsalmOutput {
        let file_source = context.source.to_deacon(worship, vars);
        if let Err(err) = file_source {
            return PsalmOutput::failed(context.info.clone(), err);
        }

        let f = file_source.unwrap();
        let source_path = f.get_path();

        let file_destination = context.target.to_deacon(worship);

        if let Err(err) = file_destination {
            return PsalmOutput::failed(context.info.clone(), err);
        }

        let d = file_destination.unwrap();
        let destination_path = d.path();

        let result = std::fs::copy(source_path, destination_path)
            .map_err(|err| err.to_string())
            .map(|_| "copy was OK".to_owned());

        PsalmOutput::simple_from_result(context.info.clone(), result)
    }
}
