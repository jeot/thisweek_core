// https://en.wikipedia.org/wiki/Unix_time
// https://en.wikipedia.org/wiki/January_1970#January_1,_1970_(Thursday)
// https://en.wikipedia.org/wiki/Leap_second
// https://www.time.ir/

use ptime;
use time::Timespec;
use serde::Serialize;

use crate::db::read_week;

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
        let reference = Self::calculate_reference_based_on_saturday(&ptime::now());
        let x = read_week(reference);
        if let Ok(Some((goals, notes))) = x {
            Self {
                reference,
                goals: goals.iter()
                    .map(|s| Goal::from(s))
                    .collect(),
            }
        } else {
            Self {
                reference: reference,
                goals: vec![],
            }
        }
        // Self {
        //     reference: reference,
        //     goals: vec![
        //         Goal::from("اینجا خیلی کارا میشه"),
        //         Goal::from("t سلام"),
        //         Goal::from("xاین کار دیگه خیلی واجبه!!"),
        //         Goal::from("این کار دیگه خیلی واجبه!!"),
        //         Goal::from("first goal"),
        //         Goal::from("this must be done!")],
        // }
    }

    pub fn calculate_reference_based_on_saturday(date: &ptime::Tm) -> i64 {
        let seconds = date.to_timespec().sec;
        let days = seconds / 3600 / 24;
        // January 1, 1970 was Thursday
        // to get the next Saturday as reference, we reduce 2 days
        let days_with_saturday_reference = days - 2;
        days_with_saturday_reference / 7
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

    pub fn today_title(&self) -> String {
        let today = ptime::now();
        today.to_string("E d MMM yyyy")
    }

    pub fn week_title(&self) -> String {
        Self::week_title_from_date(self.reference)
    }

    fn _week_title_from_today() -> String {
        let today = ptime::now();
        let reference = Self::calculate_reference_based_on_saturday(&today);
        Self::week_title_from_date(reference)
    }

    fn week_title_from_date(week_reference: i64) -> String {
        // January 1, 1970 was Thursday
        // to get the Saturday, we add 2 days
        let seconds_shanbeh = (week_reference * 7 + 2) * 24 * 3600;
        let seconds_jomeh = seconds_shanbeh + (6 * 24 * 3600);
        let shanbeh = ptime::at(Timespec::new(seconds_shanbeh, 0,));
        let jomeh = ptime::at(Timespec::new(seconds_jomeh, 0,));
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
