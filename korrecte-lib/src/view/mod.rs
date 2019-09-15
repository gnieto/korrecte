use crate::reporting::Finding;

pub mod cli;

pub trait View {
    fn render(&self, findings: &[Finding]);
}
