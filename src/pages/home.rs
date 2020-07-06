use crate::GeneratedPage;

#[derive(Default)]
pub struct HomePage {}

impl GeneratedPage for HomePage {
    fn get_route(&self) -> String {
        return "/".to_string()
    }
}
