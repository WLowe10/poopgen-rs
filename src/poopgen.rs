use crate::parse::{parse_directory, TemplateDirectory, TemplateEntry, TemplateFile};
use std::{fs, path::PathBuf};

fn process_template_entry(template_entry: &TemplateEntry, parent_dest: &PathBuf) {
    match template_entry {
        TemplateEntry::Directory(template_directory) => {
            process_template_directory(&template_directory, parent_dest);
        }
        TemplateEntry::File(template_file) => {
            process_template_file(&template_file, parent_dest);
        }
    }
}

fn process_template_directory(template_directory: &TemplateDirectory, parent_dest: &PathBuf) {
    let dir_path = parent_dest.join(&template_directory.path);

    // if let Some(poopfile_content) = &template_directory.poopfile {}

    // println!("pooping dir {:?}", dir_path);

    fs::create_dir_all(&dir_path).expect("Failed to poop dir");

    for entry in &template_directory.entries {
        process_template_entry(&entry, &dir_path);
    }
}

fn process_template_file(template_file: &TemplateFile, parent_dest: &PathBuf) {
    let file_path = parent_dest.join(&template_file.path);

    // println!("pooping file {:?}", file_path);

    std::fs::write(&file_path, &template_file.content).expect("failed to poop file");
}

pub fn poopgen(template_path: &str, dest_path: &str) {
    let template_path = PathBuf::from(template_path);
    let dest_path = PathBuf::from(dest_path);
    let mut template_directory = parse_directory(&template_path);

    // remove the name of the template dir from the template path
    template_directory.path = PathBuf::new();

    println!("{:#?}", template_directory);

    process_template_directory(&template_directory, &dest_path);
}
