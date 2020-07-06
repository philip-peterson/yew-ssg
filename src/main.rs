mod pages;
use pages::*;
use std::rc::Rc;

// These are defined by yew-ssg
pub trait GeneratedPage {
    fn get_route(&self) -> String;
}

trait Site {
    fn generate_pages() -> Box<[Rc<dyn GeneratedPage>]>;
}

// An example definition of a SSG website is below:
struct MySite {}

impl Site for MySite {
    fn generate_pages() -> Box<[Rc<dyn GeneratedPage>]> {
        let all_pages: Vec<Rc<dyn GeneratedPage>> = vec![
            Rc::new(HomePage {}),
            Rc::new(ContactPage {}),
            Rc::new(FAQPage {}),
        ];
        all_pages.into_boxed_slice()
    }
}