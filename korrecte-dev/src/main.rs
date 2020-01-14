use korrecte::linters::LintCollection;

fn main() {
    let config = korrecte::config::Config::default();
    let lints = LintCollection::all(config);

    for l in lints {
        ;
    }
}
