use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct TemplateFile {
    pub path: PathBuf,
    pub content: String,
}

#[derive(Debug)]
pub struct TemplateDirectory {
    pub path: PathBuf,
    pub entries: Vec<TemplateEntry>,
    pub poopfile: Option<String>,
}

#[derive(Debug)]
pub enum TemplateEntry {
    File(TemplateFile),
    Directory(TemplateDirectory),
}

pub fn parse_directory(path: &PathBuf) -> TemplateDirectory {
    let entities = fs::read_dir(path).expect("failed to read directory");

    let mut directory = TemplateDirectory {
        path: PathBuf::from(path.file_name().unwrap()),
        entries: Vec::new(),
        poopfile: None,
    };

    for entry in entities {
        let entry_path = entry.unwrap().path();
        let entry_name = entry_path.file_name().unwrap();

        println!("{:?}", entry_name);

        if entry_path.is_dir() {
            let template_directory = parse_directory(&entry_path);

            directory
                .entries
                .push(TemplateEntry::Directory(template_directory));
        } else {
            let content = fs::read_to_string(&entry_path).expect("failed to parse file content");

            // poopfile should not be included in entries
            if entry_name == "_poop.js" {
                directory.poopfile = Some(content);

                continue;
            }

            directory.entries.push(TemplateEntry::File(TemplateFile {
                path: PathBuf::from(entry_name),
                content,
            }))
        }
    }

    directory
}
