pub type Result<T> = std::result::Result<T, String>;

pub trait Ordering {
    fn get_keys(&self) -> Vec<Option<String>>;
    // fn get_ordering_key_of_posision(&self, i: usize) -> Result<Option<String>>;
    fn set_ordering_key_of_posision(&mut self, i: usize, key: Option<String>) -> Result<()>;
    // fn get_posision_of_id(&self, id: i32) -> Result<usize>;
    fn get_ordering_key_of_id(&self, id: i32) -> Option<Option<String>>;
    fn new_ordering_finished(&self);

    fn get_key_pos_of_id(&self, id: i32) -> Option<(String, usize)> {
        let ordering_key = self.get_ordering_key_of_id(id)??;
        let pos = self
            .get_keys()
            .iter()
            .position(|key| *key == Some(ordering_key.clone()))?;
        Some((ordering_key, pos))
    }

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

    fn get_new_ordering_key(&self, after_id: Option<i32>) -> String {
        // after_id: None: canculate based on adding new key after the last item
        // after_id: Some: canculate based on adding new key after the provided id
        let last_key = self
            .get_keys()
            .last()
            .map(|x| x.to_owned())
            .unwrap_or(None)
            .unwrap_or("".to_string());
        if let Some(id) = after_id {
            self.generate_key_for_after_id(id).unwrap_or("".into())
        } else {
            midstring::mid_string(&last_key, "")
        }
    }

    fn generate_key_for_after_id(&self, id: i32) -> Result<String> {
        let (key, pos) = self
            .get_key_pos_of_id(id)
            .ok_or("invalid id or key".to_string())?;
        let next_key = self
            .get_keys()
            .get(pos + 1)
            .map(|x| x.to_owned())
            .unwrap_or(Some(String::new()))
            .ok_or("invalid_key".to_string())?
            .clone();
        Ok(midstring::mid_string(&key, &next_key))
    }

    fn generate_key_for_move_up_with_id(&mut self, id: i32) -> Result<String> {
        let key = self
            .get_ordering_key_of_id(id)
            .ok_or("invalid id".to_string())?
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
            .get_ordering_key_of_id(id)
            .ok_or("invalid id".to_string())?
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

    fn generate_key_for_reordering_item_index(
        &self,
        src_index: usize,
        dest_index: usize,
    ) -> Result<String> {
        let keys = self.get_keys();

        // Validate indices
        if src_index >= keys.len() || dest_index >= keys.len() {
            return Err("Index out of bounds".into());
        }
        if src_index == dest_index {
            return Err("Source and destination indices are the same".into());
        }

        if dest_index == 0 {
            // Moving to the very top
            let next_key = keys[0].as_ref().ok_or("Invalid key at index 0")?;
            return Ok(midstring::mid_string("", next_key));
        }

        if dest_index == keys.len() - 1 {
            // Moving to the very bottom
            let prev_key = keys[dest_index]
                .as_ref()
                .ok_or("Invalid key at last index")?;
            return Ok(midstring::mid_string(prev_key, ""));
        }

        // For all other cases
        let prev_key;
        let next_key;
        if dest_index < src_index {
            prev_key = keys[dest_index - 1]
                .as_ref()
                .ok_or("Invalid previous key")?;
            next_key = keys[dest_index].as_ref().ok_or("Invalid next key")?;
        } else {
            prev_key = keys[dest_index].as_ref().ok_or("Invalid previous key")?;
            next_key = keys[dest_index + 1].as_ref().ok_or("Invalid next key")?;
        }

        Ok(midstring::mid_string(prev_key, next_key))
    }

    // fn generate_key_for_reordering_item_index(
    //     &mut self,
    //     src_index: usize,
    //     dest_index: usize,
    // ) -> Result<String> {
    //     // generate new key and update
    //     let prev_key;
    //     let next_key;
    //     let keys = self.get_keys();
    //     let dest_key = keys.get(dest_index)??;
    //     let before_dest_key = keys.get(dest_index - 1).unwrap_or("".to_string().clone());
    //     let after_dest_key = keys.get(dest_index + 1).unwrap_or("".to_string().clone());
    //     if dest_index < src_index {
    //         // moving up
    //         Ok(midstring::mid_string(before_dest_key, dest_key))
    //     } else if dest_index > src_index {
    //         // moving down
    //         Ok(midstring::mid_string(dest_key, after_dest_key))
    //     } else {
    //         Err("failed generating reordering item index key".into())
    //     }
    // }
}
