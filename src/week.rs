// https://en.wikipedia.org/wiki/Unix_time
// https://en.wikipedia.org/wiki/January_1970#January_1,_1970_(Thursday)
// https://en.wikipedia.org/wiki/Leap_second
// https://www.time.ir/

use ptime;
use time::Timespec;
use serde::Serialize;
use cuid2;

use crate::db;

#[derive(Serialize)]
pub struct WeekStateJs {
    today_title: String,
    week_title: String,
    goals: Vec<Element>,
    notes: Vec<Element>,
}

#[derive(Debug, Serialize, Clone)]
pub enum Element {
    Goal { id: String, text: String, done: bool },
    Note { id: String, text: String },
    // Event { id: String, text: String, /* TimePeriod */ }
}

impl Element {
    pub fn new_goal(text: String) -> Self {
        Element::Goal { id: cuid2::create_id(), text, done: false }
    }

    pub fn build_goal(id: String, text: String, done: bool) -> Self {
        Element::Goal { id, text, done }
    }

    pub fn new_note(text: String) -> Self {
        Element::Note { id: cuid2::create_id(), text }
    }

    pub fn build_note(id: String, text: String) -> Self {
        Element::Note { id, text }
    }
}

pub struct WeekState {
    pub reference: i64,
    pub elements: Vec<Element>
}

impl WeekState {
    pub fn new() -> Self {
        Self::from_ptime(&ptime::now())
    }

    pub fn from_ptime(persian_time: &ptime::Tm) -> Self {
        let reference = Self::calculate_reference_based_on_saturday(&persian_time);
        Self::from_week_reference(reference)
    }

    fn from_week_reference(reference: i64) -> Self {
        let db_result = db::read_week(reference);
        match db_result {
            Ok(Some(week)) => {
                println!("db read success. some week data.");
                week
            },
            Ok(None) => {
                println!("db read success. no week data.");
                Self {
                    reference,
                    elements: vec![],
                }
            },
            Err(err) => {
                println!("db read failed. err: {}", err);
                // println!("err kind: {}", err);
                Self {
                    reference,
                    elements: vec![],
                }
            },
        }
    }

    pub fn dummy() -> Self {
        Self {
            reference: Self::calculate_reference_based_on_saturday(&ptime::now()),
            elements: vec![
                Element::build_goal(cuid2::create_id(), "اینجا خیلی کارا میشه".to_string(), false),
                Element::build_goal(cuid2::create_id(), "t سلام".to_string(), false),
                Element::new_goal("xاین کار دیگه خیلی واجبه!!".to_string()),
                Element::new_goal("این کار دیگه خیلی واجبه!!".to_string()),
                Element::new_goal("first goal".to_string()),
                Element::build_goal(cuid2::create_id(), "this must be done!".to_string(), false),
                Element::new_note("this is a note!".to_string()),
            ]
        }
    }

    pub fn write_dummy_week_to_db() {
        println!("trying to write dummy week data...");
    }

    fn calculate_reference_based_on_saturday(date: &ptime::Tm) -> i64 {
        let seconds = date.to_timespec().sec;
        let days = seconds / 3600 / 24;
        // January 1, 1970 was Thursday
        // to get the next Saturday as reference, we reduce 2 days
        let days_with_saturday_reference = days - 2;
        days_with_saturday_reference / 7
    }

    fn calculate_first_last_week_day_based_on_saturday(reference: i64) -> (ptime::Tm, ptime::Tm) {
        // January 1, 1970 was Thursday
        // to get the Saturday, we add 2 days
        let seconds_shanbeh = (reference * 7 + 2) * 24 * 3600;
        let seconds_jomeh = seconds_shanbeh + (6 * 24 * 3600);
        let shanbeh = ptime::at(Timespec::new(seconds_shanbeh, 0,));
        let jomeh = ptime::at(Timespec::new(seconds_jomeh, 0,));
        (shanbeh, jomeh)
    }

    fn update_week_from_db(&mut self) {
        let week_result = db::read_week(self.reference);
        if let Ok(Some(week)) = week_result {
            if week.reference == self.reference {
                self.elements = week.elements;
            }
        } else if let Ok(None) = week_result {
            self.elements = vec![];
        } else {
            println!("read db failed");
        }
    }

    pub fn next(&mut self) {
        self.reference = self.reference + 1;
        self.update_week_from_db();
    }

    pub fn previous(&mut self) {
        self.reference = self.reference - 1;
        self.update_week_from_db();
    }

    pub fn current(&mut self) {
        self.reference = Self::calculate_reference_based_on_saturday(&ptime::now());
        self.update_week_from_db();
    }

    pub fn first_day(&self) -> ptime::Tm {
        Self::calculate_first_last_week_day_based_on_saturday(self.reference).0
    }

    pub fn last_day(&self) -> ptime::Tm {
        Self::calculate_first_last_week_day_based_on_saturday(self.reference).1
    }

    pub fn today_title(&self) -> String {
        let today = ptime::now();
        if today.tm_mday == 6 && today.tm_mon == 11 {
            today.to_string("E d MMM yyyy 🎉")
        } else {
            today.to_string("E d MMM yyyy")
        }
    }

    pub fn week_title(&self) -> String {
        let shanbeh = self.first_day();
        let jomeh = self.last_day();
        format!(
            "{} ... {}",
            shanbeh.to_string("E d MMM yyyy"),
            jomeh.to_string("E d MMM yyyy")
        )
    }

    pub fn week_state_js_object(&self) -> WeekStateJs {
        WeekStateJs {
            today_title: self.today_title(),
            week_title: self.week_title(),
            goals: self.elements.clone().into_iter().filter(|e| {
                if let Element::Goal{..} = e { true }
                else { false }
            }).collect(),
            notes: self.elements.clone().into_iter().filter(|e| {
                if let Element::Note{..} = e { true }
                else { false }
            }).collect(),
        }
    }

    pub fn add_new_goal(&mut self, text: String) {
        println!("adding a new goal: {text}");
        let goal = Element::new_goal(text);
        self.elements.push(goal);
        let _result = db::write_week(self);
    }

    pub fn toggle_goal_state(&mut self, goal_id: String) -> bool {
        println!("searching for goal id {goal_id}");
        let mut mut_iter = self.elements.iter_mut();
        // let mut mut_iter = self.elements.into_iter();
        let goal = mut_iter
            .find(|e| {
            if let Element::Goal {id,..} = e {
                if *id == goal_id { true }
                else { false }
            } else {
                false
            }});
        let mut final_done_value = false;
        match goal {
            Some(Element::Goal{done: d,..}) => {
                println!("found the goal, toggling the done parameter");
                *d = !(*d);
                final_done_value = *d;
            },
            _ => (),
        }
        let _result = db::write_week(self);
        println!("returning: {final_done_value}");
        return final_done_value;
    }
}


#[cfg(test)]
mod tests {
    use crate::week::WeekState;

    fn check_correct_reference_from_dates(dates: Vec<ptime::Tm>, expected_reference: i64) -> bool {
        println!("----");
        for pt in dates {
            let reference = WeekState::calculate_reference_based_on_saturday(&pt);
            let (first, last) = WeekState::calculate_first_last_week_day_based_on_saturday(reference);
            let date = pt.to_string("yyyy-MM-dd HH:mm:ss");
            let shanbeh = first.to_string("yyyy-MM-dd HH:mm:ss");
            let jomeh = last.to_string("yyyy-MM-dd HH:mm:ss");
            println!("ptime: {}, ref: {}, week: {} -> {}", date, reference, shanbeh, jomeh);
            if expected_reference != reference {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_find_week_period_with_ptime() {
        let mut pt_vec: Vec<ptime::Tm> = Vec::new();
        let pt = ptime::from_persian_components(1402, 11-1, 27, 23, 22, 11, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 11-1, 27, 23, 59, 36, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 11-1, 27, 23, 59, 59, 0).unwrap();
        pt_vec.push(pt);
        assert!(check_correct_reference_from_dates(pt_vec, 2823));

        let mut pt_vec: Vec<ptime::Tm> = Vec::new();
        let pt = ptime::from_persian_components(1402, 11-1, 28, 0, 0, 0, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 11-1, 28, 0, 0, 1, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 11-1, 28, 0, 0, 11, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 11-1, 28, 0, 1, 22, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 1, 12, 0, 0, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 4, 0, 0, 0, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 4, 23, 23, 23, 23).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 4, 23, 59, 23, 23).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 4, 23, 59, 59, 19993294).unwrap();
        pt_vec.push(pt);
        assert!(check_correct_reference_from_dates(pt_vec, 2824));

        let mut pt_vec: Vec<ptime::Tm> = Vec::new();
        let pt = ptime::from_persian_components(1402, 12-1, 5, 0, 0, 0, 0).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 5, 0, 0, 0, 1).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 5, 0, 0, 1, 1).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 5, 0, 1, 1, 1).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 5, 1, 1, 1, 1).unwrap();
        pt_vec.push(pt);
        let pt = ptime::from_persian_components(1402, 12-1, 6, 6, 6, 6, 6).unwrap();
        pt_vec.push(pt);
        assert!(check_correct_reference_from_dates(pt_vec, 2825));
    }
}
