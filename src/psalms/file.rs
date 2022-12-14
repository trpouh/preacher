use crate::{psalms::PsalmInfo, worship::Worship};
use serde::Deserialize;

use super::{
    deacons::file::{destination::FileDestination, source::FileSource},
    Psalm, PsalmOutput,
};

pub struct FilePsalm {}

#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct FileContext {
    source: FileSource,
    target: FileDestination,
}

impl Psalm<FileContext> for FilePsalm {
    fn invoke(context: &FileContext, worship: &Worship) -> PsalmOutput {
        // can not simply copy, maybe downloaded first. put logic into file source/destination
        // file source -> get_path() (http download first -> file_source.check().get_path())

        // let SourceDeacon = file_source.check().get_path()
        // let DesinationDeacon = file_destination.check().get_path()

        // copy(source_deacon.get_path(), destination_deacon.get_path()

        // let new = file_destination.check()
        // let old = destination.get_content()
        // let differs = destination.get_content() != file_destination.check()

        let file_source = context.source.to_deacon(worship);
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

        let result = std::fs::copy(source_path, destination_path).map_err(|err|err.to_string()).map(|_| "copy was OK".to_owned());

        PsalmOutput::simple_from_result(context.info.clone(),result)
    }
}
