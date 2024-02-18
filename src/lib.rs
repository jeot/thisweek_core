use ptime;
use time::Timespec;
use serde::Serialize;

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
    reference: ptime::Tm,
    goals: Vec<Goal>,
}

impl WeekState {
    pub fn new() -> Self {
        Self {
            reference: ptime::now(),
            goals: vec![
                Goal::from("اینجا خیلی کارا میشه"),
                Goal::from("t سلام"),
                Goal::from("xاین کار دیگه خیلی واجبه!!"),
                Goal::from("این کار دیگه خیلی واجبه!!"),
                Goal::from("first goal"),
                Goal::from("this must be done!")],
        }
    }

    pub fn next(&mut self) {
        let reference = self.reference.clone();
        let next_week_date = ptime::at(Timespec::new(
            reference.to_timespec().sec + (7 * 24 * 3600),
            0,
        ));
        self.reference = next_week_date;
    }

    pub fn previous(&mut self) {
        let reference = self.reference.clone();
        let next_week_date = ptime::at(Timespec::new(
            reference.to_timespec().sec - (7 * 24 * 3600),
            0,
        ));
        self.reference = next_week_date;
    }

    pub fn current(&mut self) {
        self.reference = ptime::now();
    }

    pub fn today_title(&self) -> String {
        let today = ptime::now();
        today.to_string("E d MMM yyyy")
    }

    pub fn week_title(&self) -> String {
        Self::week_title_from_date(&self.reference)
    }

    fn _week_title_from_today() -> String {
        let today = ptime::now();
        Self::week_title_from_date(&today)
    }

    fn week_title_from_date(date_in_week: &ptime::Tm) -> String {
        let shanbeh = ptime::at(Timespec::new(
            date_in_week.to_timespec().sec - i64::from(date_in_week.tm_wday * (24 * 3600)),
            0,
        ));
        let jomeh = ptime::at(Timespec::new(
            shanbeh.to_timespec().sec + i64::from(6 * (24 * 3600)),
            0,
        ));
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
