use korrecte::linters::LintSpecLoader;
use std::io::*;
use std::fs::{File, OpenOptions};
use std::io::Read;

fn main() {
    let spec_loader = LintSpecLoader::new().unwrap();
    let mut buffer = Vec::new();
    buffer.push(["Name", "Group", "Description", "References"].join("|"));
    buffer.push(["---", "---", "---", "---"].join("|"));

    let mut lint_content = Vec::new();
    for ls in spec_loader.all().values() {
        let content = [
            ls.name.clone(),
            ls.group.to_string(),
            ls.description.clone(),
            ls.references.join("<br>"),
        ];

        lint_content.push((ls.name.clone(), content.join("|")));
    }

    lint_content.sort_by(|a, b| a.0.cmp(&b.0));
    buffer.extend(lint_content.into_iter().map(|p| p.1));

    let new_readme = replace_readme(&buffer.join("\n"))
        .expect("Readme could be replaced");
    store_new_readme(&new_readme)
        .expect("Readme could be written");

}

fn replace_readme(lints_info: &str) -> Result<String> {
    let mut file = File::open("../README.md")?;
    let mut current_content = String::new();
    file.read_to_string(&mut current_content)?;

    let mut new_content  = String::new();
    let prelude_pos = current_content.find("## Current lints")
        .expect("Current lints string should be present on README.md");
    let finale_pos = current_content.find("## Roadmap ideas")
        .expect("Roadmap ideas string should be present on readme");

    new_content.push_str(&current_content[..prelude_pos]);
    new_content.push_str("## Current lints \n\n");
    new_content.push_str(lints_info);
    new_content.push_str(&current_content[finale_pos-"## Roadmap ideas".len()..]);

    Ok(new_content)
}

fn store_new_readme(content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("../README.md")?;
    file.write_all(content.as_bytes())?;

    Ok(())
}