pub type Result<T> = std::result::Result<T, String>;

pub trait Ordering {
    fn get_keys(&self) -> Vec<Option<String>>;
    // fn get_ordering_key_of_posision(&self, i: usize) -> Result<Option<String>>;
    fn set_ordering_key_of_posision(&mut self, i: usize, key: Option<String>) -> Result<()>;
    // fn get_posision_of_id(&self, id: i32) -> Result<usize>;
    fn get_ordering_key_of_id(&self, id: i32) -> Result<Option<String>>;
    fn new_ordering_finished(&self);

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
        let mut i: usize = 0;
        loop {
            let new_key = midstring::mid_string(&top, &bot);
            if let Err(_) = self.set_ordering_key_of_posision(i, Some(new_key.clone())) {
                break;
            }
            i += 1;
            top = new_key;
        }
        self.new_ordering_finished();
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

    fn get_new_ordering_key(&self) -> String {
        // canculate based on adding new key after the last item
        let keys = self.get_keys();
        let last_key = keys.last();
        if let Some(Some(key)) = last_key {
            midstring::mid_string(key, "")
        } else {
            midstring::mid_string("", "")
        }
    }

    fn generate_key_for_move_up_with_id(&mut self, id: i32) -> Result<String> {
        let key = self
            .get_ordering_key_of_id(id)?
            .ok_or("invalid key".to_string())?;
        self.generate_key_for_move_up_with_key(key)
    }

    fn generate_key_for_move_up_with_key(&mut self, key: String) -> Result<String> {
        // reordering logic:
        // get the ordering-keys of two previous items
        // generate new key and update
        let prev_key;
        let next_key;
        let keys = self.get_keys();
        if let Some(pos) = keys.iter().position(|k| *k == Some(key.clone())) {
            if pos == 0 {
                // already first item
                return Ok(keys[pos].clone().unwrap_or("".to_string()));
            } else if pos == 1 {
                prev_key = "".to_string();
                next_key = keys[pos - 1].clone().unwrap_or("".to_string());
            } else {
                // pos > 2
                prev_key = keys[pos - 2].clone().unwrap_or("".to_string());
                next_key = keys[pos - 1].clone().unwrap_or("".to_string());
            }
            Ok(midstring::mid_string(&prev_key, &next_key))
        } else {
            Err("invalid key".to_string())
        }
    }

    fn generate_key_for_move_down_with_id(&mut self, id: i32) -> Result<String> {
        let key = self
            .get_ordering_key_of_id(id)?
            .ok_or("invalid key".to_string())?;
        self.generate_key_for_move_down_with_key(key)
    }

    fn generate_key_for_move_down_with_key(&mut self, key: String) -> Result<String> {
        // reordering logic:
        // get the ordering-keys of two next items
        // generate new key and update
        let prev_key;
        let next_key;
        let keys = self.get_keys();
        let length = keys.len();
        if let Some(pos) = keys.iter().position(|k| *k == Some(key.clone())) {
            if pos == length - 1 {
                // already last item
                return Ok(keys[pos].clone().unwrap_or("".to_string()));
            } else if pos == length - 2 {
                prev_key = keys[pos + 1].clone().unwrap_or("".to_string());
                next_key = "".to_string();
            } else {
                // pos < length - 2
                prev_key = keys[pos + 1].clone().unwrap_or("".to_string());
                next_key = keys[pos + 2].clone().unwrap_or("".to_string());
            }
            Ok(midstring::mid_string(&prev_key, &next_key))
        } else {
            Err("invalid key".to_string())
        }
    }
}
