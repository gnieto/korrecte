use crate::reporting::Finding;

pub trait View {
    fn render(&self, findings: &[Finding]);
}
