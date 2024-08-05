pub trait Ordering {
    fn get_keys(&self) -> Vec<Option<String>>;
    fn get_ordering_key_mut_iter(&mut self) -> &Option<String>;

    fn needs_reordering(&self) -> bool {
        let mut fix = false;
        for item_key in self.get_keys() {
            if item_key.is_none() {
                fix = true;
                break;
            }
        }
        fix
    }

    fn new_ordering(&mut self) {
        println!("reordering all items...");
        let mut top = String::from("");
        let bot = String::from("");
        let kesy_iter = self.get_ordering_key_mut_iter();
        for old_key in kesy_iter {
            let new_key = midstring::mid_string(&top, &bot);
            *old_key = Some(new_key.clone());
            top = new_key;
        }
    }

    fn check_and_fix_ordering(&mut self) {
        let fix = self.needs_reordering();
        if fix {
            // println!("fixing some invalid ordering keys...");
            // println!("items before ordering: {:?}", self);
            self.new_ordering();
            // println!("items after ordering: {:?}", self);
        } else {
            // println!("ordering seems ok");
        }
    }
}
