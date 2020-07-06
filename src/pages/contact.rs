use crate::GeneratedPage;

#[derive(Default)]
pub struct ContactPage {}

impl GeneratedPage for ContactPage {
    fn get_route(&self) -> String {
        return "/contact".to_string()
    }
}
