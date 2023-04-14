use xmltree::Element;

pub trait ElemUtils {
    fn any_text(&self) -> Option<String>;

}

impl ElemUtils for Element {
    fn any_text(&self) -> Option<String> {
        if let Some(text) = self.get_text() {
            return Some(text.into_owned());
        }

        for child in &self.children {
            if let Some(elem) = child.as_element() {
                if let Some(text) = elem.any_text() {
                    return Some(text);
                }
            }
        }

        None
    }

    // fn any_text_mut(&mut self) -> Option<TextsMut> {
    //     if !self.text().is_empty() {
    //         return Some(self.texts_mut());
    //     }

    //     for child in self.children_mut() {
    //         if let Some(text) = child.any_text_mut() {
    //             return Some(text);
    //         }
    //     }

    //     None
    // }
}
