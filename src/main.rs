extern crate nfd;
extern crate serde;
extern crate quick_xml;
#[macro_use]
extern crate serde_derive;
extern crate regex;

use dialog as PromptDialog;
use dialog::DialogBox;
use itertools::Itertools;
use nfd::{Response as FileDialogResponse, DialogType, DialogBuilder};
use rayon::prelude::*;
use regex::Regex;

use crate::xml::{Audit, audit_from_xml_file, audit_to_xml, generate_xml_header};
use std::fs::File;
use std::io::Write;

mod xml;
mod interceptor;

fn main() {
    let main_file = select_main_file().unwrap();
    let interception_files = select_interception_files().unwrap();

    let mut main_audit = audit_from_xml_file(main_file.clone());
    let interception_audits = generate_audits_from_files(interception_files);

    let duplicated_document_numbers = get_duplicated_document_numbers(&main_audit, interception_audits);

    let remove_duplication_choice = PromptDialog::Question::new(
        format!(
            "There are ({:?}) document numbers should be removed: {}",
            duplicated_document_numbers.len(),
            duplicated_document_numbers.chunks(6).map(|chunk| chunk.join(",")).join("\n"),
        ))
        .title("Remove Duplicate documents")
        .show()
        .expect("Unable to show Dialog . . .");

    if remove_duplication_choice != dialog::Choice::Yes {
        return;
    }

    interceptor::remove_duplicated_numbers(&mut main_audit, duplicated_document_numbers);
    main_audit.recalculate_returns();

    let audit_xml = format!(
        "{}\n{}",
        generate_xml_header(String::from("1.0"), String::from("WINDOWS-1251")),
        audit_to_xml(main_audit)
    );

    let file_name_regex = Regex::new(r"(?P<file_name>[\w\d-]+\.xml$)").unwrap();
    let main_file_name = file_name_regex.captures(main_file.as_str())
        .and_then(|capture| {
            capture.name("file_name").map(|file_name| file_name.as_str())
        }).unwrap();

    let file_save_location = select_save_location();
    let mut file_handle = File::create(
        format!(
            "{}/{}_clean.xml",
            file_save_location,
            main_file_name.replace(".xml", "")
        )
    ).unwrap();
    file_handle.write_all(audit_xml.as_bytes()).unwrap();
}

fn select_main_file() -> Result<String, ()> {
    let filter = Some("xml");
    let result = nfd::open_file_dialog(filter, None).unwrap_or_else(|e| {
        panic!("{}", e)
    });

    match result {
        FileDialogResponse::Okay(file_path) => Ok(file_path),
        FileDialogResponse::OkayMultiple(_) => Err(()),
        FileDialogResponse::Cancel => Err(())
    }
}

fn select_interception_files() -> Result<Vec<String>, ()> {
    let filter = Some("xml");
    let result = nfd::open_file_multiple_dialog(filter, None).unwrap_or_else(|e| {
        panic!("{}", e)
    });

    match result {
        FileDialogResponse::Okay(file) => Ok(vec![file]),
        FileDialogResponse::OkayMultiple(files) => Ok(files),
        FileDialogResponse::Cancel => Err(())
    }
}

fn generate_audits_from_files(files: Vec<String>) -> Vec<Audit> {
    files.into_par_iter().map(audit_from_xml_file).collect()
}

fn get_duplicated_document_numbers(main_audit: &Audit, interception_audits: Vec<Audit>) -> Vec<String> {
    let result: Vec<String> = interception_audits.into_par_iter()
        .flat_map(|audit| interceptor::intercept_audit_document_numbers(main_audit, &audit))
        .collect();

    result.into_iter().unique().collect()
}

fn select_save_location() -> String {
    let response = DialogBuilder::new(DialogType::PickFolder)
        .default_path(".")
        .open()
        .unwrap();

    match response {
        FileDialogResponse::Okay(location) => location,
        _ => return String::from("")
    }
}
