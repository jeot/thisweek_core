// https://en.wikipedia.org/wiki/Unix_time
// https://en.wikipedia.org/wiki/January_1970#January_1,_1970_(Thursday)
// https://en.wikipedia.org/wiki/Leap_second
// https://www.time.ir/

use ptime;
use time::Timespec;
use serde::Serialize;

use crate::db;

#[derive(Serialize)]
pub struct WeekStateJs {
    week_title: String,
    today_title: String,
    goals: Vec<Goal>,
}

#[derive(Debug, Serialize, Clone)]
struct Goal {
    id: i32,
    text: String,
    done: bool,
}

impl Goal {
    fn from(text: &str) -> Self {
        Self {
            id: 0,
            text: String::from(text),
            done: false,
        }
    }
}

pub struct WeekState {
    reference: i64,
    goals: Vec<Goal>,
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
            Ok(Some((goals, notes))) => {
                println!("db read success. week data exists.");
                Self {
                    reference,
                    goals: goals.iter()
                        .map(|s| Goal::from(s))
                        .collect(),
                }
            },
            Ok(None) => {
                println!("db read success. no week data.");
                Self {
                    reference,
                    goals: vec![],
                }
            },
            Err(err) => {
                println!("db read failed. err: {}", err);
                Self {
                    reference,
                    goals: vec![],
                }
            },
        }
    }

    pub fn dummy() -> Self {
        Self {
            reference: Self::calculate_reference_based_on_saturday(&ptime::now()),
            goals: vec![
                Goal::from("اینجا خیلی کارا میشه"),
                Goal::from("t سلام"),
                Goal::from("xاین کار دیگه خیلی واجبه!!"),
                Goal::from("این کار دیگه خیلی واجبه!!"),
                Goal::from("first goal"),
                Goal::from("this must be done!")],
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

    pub fn next(&mut self) {
        self.reference = self.reference + 1;
    }

    pub fn previous(&mut self) {
        self.reference = self.reference - 1;
    }

    pub fn current(&mut self) {
        self.reference = Self::calculate_reference_based_on_saturday(&ptime::now());
    }

    pub fn first_day(&self) -> ptime::Tm {
        Self::calculate_first_last_week_day_based_on_saturday(self.reference).0
    }

    pub fn last_day(&self) -> ptime::Tm {
        Self::calculate_first_last_week_day_based_on_saturday(self.reference).1
    }

    pub fn today_title(&self) -> String {
        let today = ptime::now();
        today.to_string("E d MMM yyyy")
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
            week_title: self.week_title(),
            today_title: self.today_title(),
            goals: self.goals.clone(),
        }
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
