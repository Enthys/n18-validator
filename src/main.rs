extern crate quick_xml;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::Write;

use itertools::Itertools;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use rayon::prelude::*;
use regex::Regex;

use crate::xml::{Audit, audit_from_xml_file, audit_to_xml, generate_xml_header};

mod xml;
mod interceptor;

fn main() {
    let main_file = select_main_file();
    let interception_files = select_interception_files();

    let mut main_audit = audit_from_xml_file(main_file.clone());
    let interception_audits = generate_audits_from_files(interception_files);

    let duplicated_document_numbers = get_duplicated_document_numbers(&main_audit, interception_audits);

    let remove_duplication_choice = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Remove duplicate documents?")
        .set_text(&format!(
            "There are ({:?}) document numbers should be removed: {}",
            duplicated_document_numbers.len(),
            duplicated_document_numbers.chunks(7).map(|chunk| chunk.join(",")).join("\n"),
        ))
        .show_confirm()
        .unwrap();

    if remove_duplication_choice == false {
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
    let main_file_name = format!("{}_clean.xml", file_name_regex.captures(main_file.as_str())
        .and_then(|capture| {
            capture.name("file_name").map(|file_name| file_name.as_str())
        }).unwrap().replace(".xml", ""));

    let file_save_location = select_save_location(main_file_name.clone());

    let mut file_handle = File::create(file_save_location).unwrap();
    file_handle.write_all(audit_xml.as_bytes()).unwrap();
}

fn select_main_file() -> String {
    String::from(FileDialog::new()
        .add_filter("XML File", &["xml"])
        .show_open_single_file()
        .unwrap()
        .unwrap()
        .to_str()
        .unwrap()
    )
}

fn select_interception_files() -> Vec<String> {
    FileDialog::new()
        .add_filter("XML File", &["xml"])
        .show_open_multiple_file()
        .unwrap()
        .iter()
        .map(|path_buf| String::from(path_buf.to_str().unwrap()))
        .collect_vec()
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

fn select_save_location(default_file_name: String) -> String {
    String::from(FileDialog::new()
        .set_filename(default_file_name.as_str())
        .show_save_single_file()
        .unwrap()
        .unwrap()
        .to_str()
        .unwrap()
    )
}
