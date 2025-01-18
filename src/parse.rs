use std::{
    fs,
    path::{Path, PathBuf},
};

const POOPFILE_NAME: &str = "_poop.js";
const TEMPLATE_FILE_SUFFIX: &str = ".hbs";

#[derive(Debug)]
pub struct TemplateFile {
    pub path: PathBuf,
    pub content: String,
    pub is_template: bool,
}

#[derive(Debug)]
pub struct TemplateDirectory {
    pub path: PathBuf,
    pub poopfile: Option<String>,
    pub entries: Vec<TemplateEntry>,
}

#[derive(Debug)]
pub enum TemplateEntry {
    File(TemplateFile),
    Directory(TemplateDirectory),
}

pub fn parse_directory(path: &Path) -> TemplateDirectory {
    let entities = fs::read_dir(path).expect("failed to read directory");

    let mut directory = TemplateDirectory {
        path: PathBuf::from(path.file_name().unwrap()),
        entries: Vec::new(),
        poopfile: None,
    };

    for entry in entities {
        let entry_path = entry.unwrap().path();
        let mut entry_name = entry_path.file_name().unwrap().to_str().unwrap().to_owned();

        if entry_path.is_dir() {
            let template_directory = parse_directory(&entry_path);

            directory
                .entries
                .push(TemplateEntry::Directory(template_directory));
        } else {
            let content = fs::read_to_string(&entry_path).expect("failed to parse file content");
            let mut is_template = false;

            // poopfile should not be included in entries.
            if entry_name == POOPFILE_NAME {
                directory.poopfile = Some(content);

                continue;
            }

            if entry_name.ends_with(TEMPLATE_FILE_SUFFIX) {
                is_template = true;
                entry_name = entry_name.replace(TEMPLATE_FILE_SUFFIX, "");
            }

            directory.entries.push(TemplateEntry::File(TemplateFile {
                path: PathBuf::from(entry_name),
                content,
                is_template,
            }))
        }
    }

    directory
}
