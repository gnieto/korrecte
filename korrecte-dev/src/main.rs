use korrecte::linters::LintSpecLoader;

fn main() {
    let spec_loader = LintSpecLoader::new().unwrap();
    let mut buffer = Vec::new();
    buffer.push(["Name", "Group", "Description", "References"].join("|"));
    buffer.push(["---", "---", "---", "---"].join("|"));

    for ls in spec_loader.all().values() {
        buffer.push(
            [
                ls.name.clone(),
                ls.group.to_string(),
                ls.description.clone(),
                ls.references.join("<br>"),
            ]
            .join("|"),
        )
    }

    println!("{}", buffer.join("\n"));
}
