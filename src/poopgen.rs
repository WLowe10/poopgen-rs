use crate::parse::{parse_directory, TemplateDirectory, TemplateEntry, TemplateFile};
use handlebars::Handlebars;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

fn process_template_entry(
    template_entry: &TemplateEntry,
    parent_dest: &Path,
    data: &HashMap<String, String>,
) {
    match template_entry {
        TemplateEntry::Directory(template_directory) => {
            process_template_directory(template_directory, parent_dest, data);
        }
        TemplateEntry::File(template_file) => {
            process_template_file(template_file, parent_dest, data);
        }
    }
}

fn process_template_directory(
    template_directory: &TemplateDirectory,
    parent_dest: &Path,
    data: &HashMap<String, String>,
) {
    let dir_path = parent_dest.join(&template_directory.path);

    // if let Some(poopfile_content) = &template_directory.poopfile {}

    // println!("pooping dir {:?}", dir_path);

    fs::create_dir_all(&dir_path).expect("Failed to poop dir");

    for entry in &template_directory.entries {
        process_template_entry(entry, &dir_path, data);
    }
}

fn process_template_file(
    template_file: &TemplateFile,
    parent_dest: &Path,
    data: &HashMap<String, String>,
) {
    let file_path = parent_dest.join(&template_file.path);
    let mut content = template_file.content.clone();

    if template_file.is_template {
        let handlebars = Handlebars::new();

        content = handlebars
            .render_template(&content, data)
            .expect("failed to render handlebars template");
    }

    // println!("pooping file {:?}", file_path);

    std::fs::write(&file_path, content).expect("failed to poop file");
}

pub fn poopgen(template_path: &str, dest_path: &str, data: &HashMap<String, String>) {
    let template_path = PathBuf::from(template_path);
    let dest_path = PathBuf::from(dest_path);
    let mut template_directory = parse_directory(&template_path);

    // remove the name of the template dir from the template path
    template_directory.path = PathBuf::new();

    // println!("{:#?}", template_directory);

    process_template_directory(&template_directory, &dest_path, data);
}
