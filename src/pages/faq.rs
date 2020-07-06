use crate::GeneratedPage;

#[derive(Default)]
pub struct FAQPage {}

impl GeneratedPage for FAQPage {
    fn get_route(&self) -> String {
        return "/faq".to_string()
    }
}
