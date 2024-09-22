use crate::language::str_to_vec;
use crate::weekdays::convert_weekday;
use crate::{language::Language, week_info::Date, week_info::DateView};
use chrono::Datelike;
use chrono::{DateTime, Local};
use serde::Serialize;

use super::{Calendar, CalendarSpecificDateView, CalendarView, CALENDAR_ARABIC};

include!("../week_names.rs");

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ArabicCalendar;

// Muharram (مُحَرَّم): The first month, meaning “forbidden,” as fighting is prohibited.
// Safar (صَفَر): The second month, meaning “void,” as homes were often empty during this time.
// Rabi’ al-Awwal (رَبِيعُ الأَوَّلِ): The third month, meaning “the first spring.”
// Rabi’ al-Thani (رَبِيعُ الثَّانِي): The fourth month, meaning “the second spring.”
// Jumada al-Awwal (جُمَادَىٰ الأُولَىٰ): The fifth month, marking the dry period.
// Jumada al-Thani (جُمَادَىٰ الآخِرَة): The sixth month, marking the end of the dry period.
// Rajab (رَجَب): The seventh month, a sacred month when fighting is prohibited.
// Sha’ban (شَعْبَان): The eighth month, leading into Ramadan.
// Ramadan (رَمَضَان): The ninth and holiest month, known for fasting.
// Shawwal (شَوَّال): The tenth month, marking the end of Ramadan.
// Dhu al-Qi’dah (ذُو القَعْدَة): The eleventh month, another sacred month.
// Dhu al-Hijjah (ذُو الحِجَّة): The twelfth month, during which Hajj takes place.

const MONTH_NAME_FULL_EN: [&str; 12] = [
    "Muharram",
    "Safar",
    "Rabi’ al-Awwal",
    "Rabi’ al-Thani",
    "Jumada al-Awwal",
    "Jumada al-Thani",
    "Rajab",
    "Sha’ban",
    "Ramadan",
    "Shawwal",
    "Dhu al-Qi’dah",
    "Dhu al-Hijjah",
];

const MONTH_NAME_FULL_AR: [&str; 12] = [
    "مُحَرَّم",
    "صَفَر",
    "رَبِيعُ الأَوَّلِ",
    "رَبِيعُ الثَّانِي",
    "جُمَادَىٰ الأُولَىٰ",
    "جُمَادَىٰ الآخِرَة",
    "رَجَب",
    "شَعْبَان",
    "رَمَضَان",
    "شَوَّال",
    "ذُو القَعْدَة",
    "ذُو الحِجَّة",
];

const MONTH_NAME_FULL_FA: [&str; 12] = [
    "محرم",
    "صفر",
    "ربیع الاول",
    "ربیع الثانی",
    "جمادی الاول",
    "جمادی الثانی",
    "رجب",
    "شعبان",
    "رمضان",
    "شوال",
    "ذیقعده",
    "ذیحجه",
];

// نام فصل‌ها به عربی چگونه بیان می‌شود؟
// چهار فصل در زبان عربی به شکل «فصل الربیع»، «فصل الصیف»، «فصل الخریف» و «فصل الشتاء» بیان می‌شود.

const SEASON_NAME_FULL_AR: [&str; 4] = ["فصل الربیع", "فصل الصیف", "فصل الخریف", "فصل الشتاء"];

const SEASON_NAME_FULL_EN: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

const SEASON_NAME_FULL_FA: [&str; 4] = ["بهار", "تابستان", "پاییز", "زمستان"];

impl CalendarSpecificDateView for ArabicCalendar {
    fn new_date(datetime: DateTime<Local>) -> Date {
        let day = datetime.day() as u8;
        let month = datetime.month() as u8;
        let year = datetime.year();
        let weekday = convert_weekday(datetime.weekday()) as u32;
        use icu::calendar::islamic::IslamicCivil;
        let date_iso = icu::calendar::Date::try_new_iso_date(year, month, day)
            .expect("Failed to initialize ISO Date instance.");
        // Conversion into Indian calendar: 1914-08-02.
        let date = date_iso.to_calendar(IslamicCivil);
        let year = date.year().number;
        let month = date.month().ordinal;
        let day = date.day_of_month().0;
        Date {
            calendar: Calendar::Arabic(ArabicCalendar),
            day,
            month,
            weekday,
            year,
        }
    }

    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> DateView {
        let date = Self::new_date(datetime);
        let day = date.day.to_string();
        let day = lang.change_numbers_language(&day);
        let month0 = (date.month - 1) as usize;
        let month = match lang {
            Language::Arabic => MONTH_NAME_FULL_AR[month0],
            Language::Farsi => MONTH_NAME_FULL_FA[month0],
            _ => MONTH_NAME_FULL_EN[month0],
        };
        let month = month.to_string();
        let weekday = date.weekday;
        let weekday = match lang {
            Language::Arabic => WEEKDAY_NAME_FULL_AR[weekday as usize],
            Language::Farsi => WEEKDAY_NAME_FULL_FA[weekday as usize],
            _ => WEEKDAY_NAME_HALF_CAP_EN[weekday as usize],
        };
        let weekday = weekday.to_string();
        let year = date.year.to_string();
        let year = lang.change_numbers_language(&year);
        DateView {
            unix_day: 0,
            day,
            month,
            weekday,
            year,
        }
    }

    fn get_calendar_view(lang: &Language) -> CalendarView {
        let months_names: Vec<String> = match lang {
            Language::Arabic => str_to_vec(&MONTH_NAME_FULL_AR),
            Language::Farsi => str_to_vec(&MONTH_NAME_FULL_FA),
            _ => str_to_vec(&MONTH_NAME_FULL_EN),
        };
        let seasons_names: Vec<String> = match lang {
            Language::Arabic => str_to_vec(&SEASON_NAME_FULL_AR),
            Language::Farsi => str_to_vec(&SEASON_NAME_FULL_FA),
            _ => str_to_vec(&SEASON_NAME_FULL_EN),
        };
        let calendar_name: String = match lang {
            Language::Arabic => "التقويم العربيّ (الهجريّ القمريّ)".into(),
            Language::Farsi => "تقویم هجری قمری (عربی)".into(),
            _ => "Arabic Calendar".into(),
        };
        CalendarView {
            calendar: CALENDAR_ARABIC,
            calendar_name,
            language: lang.clone().into(),
            direction: lang.default_direction(),
            months_names,
            seasons_names,
        }
    }
}
