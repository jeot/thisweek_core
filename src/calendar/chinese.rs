use crate::{
    language::{str_to_vec, Language},
    week_info::{Date, DateView},
    weekdays::{convert_weekday, WeekDaysUnixOffset},
};

include!("../week_names.rs");

use super::{Calendar, CalendarSpecificDateView, CalendarView, CALENDAR_CHINESE};
use chinese_lunisolar_calendar::{LunisolarDate, SolarDate};
use chrono::{DateTime, Datelike, Local};
use serde::Serialize;

// First = 1 正月
// Second = 2 二月
// Third = 3 三月
// Fourth = 4 四月
// Fifth = 5 五月
// Sixth = 6 六月
// Seventh = 7 七月
// Eighth = 8 八月
// Ninth = 9 九月
// Tenth = 10 十月
// Eleventh = 11 冬月
// Twelfth = 12 臘月
// LeapFirst = 101 閏正月
// LeapSecond = 102 閏二月
// LeapThird = 103 閏三月
// LeapFourth = 104 閏四月
// LeapFifth = 105 閏五月
// LeapSixth = 106 閏六月
// LeapSeventh = 107 閏七月
// LeapEighth = 108 閏八月
// LeapNinth = 109 閏九月
// LeapTenth = 110 閏十月
// LeapEleventh = 111 閏冬月
// LeapTwelfth = 112 閏臘月
const MONTH_NAME_FULL_CN: [&str; 12] = [
    "正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "冬月", "臘月",
];

// Start Date	Traditional Name	Earthly Branch Name	Modern Name
//
// Between Jan 21 - Feb 20	陬月 (zōu yuè) Corner Month	寅月 (yín yuè) Tiger Month	正月 (zhēng yuè) 1st Month
// Between Feb 20 - Mar 21	杏月 (xìng yuè) Apricot Month	卯月 (mǎo yuè) Rabbit Month	二月 (èr yuè) 2nd Month
// Between Mar 21 - Apr 20	桃月 (táo yuè) Peach Month	辰月 (chén yuè) Dragon Month	三月 (sān yuè) 3rd Month
// Between Apr 20 - May 21	梅月 (méi yuè) Plum Flower Month	巳月 (sì yuè) Snake Month	四月 (sì yuè) 4th Month
// Between May 21 - Jun 21	榴月 (liú yuè) Pomegranate Flower Month	午月 (wǔ yuè) Horse Month	五月 (wǔ yuè) 5th Month
// Between Jun 21 - Jul 23	荷月 (hé yuè) Lotus Month	未月 (wèi yuè) Goat Month	六月 (liù yuè) 6th Month
// Between Jul 23 - Aug 23	兰月 (lán yuè) Orchid Month	申月 (shēn yuè) Monkey Month	七月 (qī yuè) 7th Month
// Between Aug 23 - Sept 23	桂月 (guì yuè) Osmanthus Month	酉月 (yǒu yuè) Rooster Month	八月 (bā yuè) 8th Month
// Between Sept 23 - Oct 23	菊月 (jú yuè) Chrysanthemum Month	戌月 (xū yuè) Dog Month	九月 (jiǔ yuè) 9th Month
// Between Oct 23 - Nov 22	露月 (lù yuè) Dew Month	亥月 (hài yuè) Pig Month	十月 (shí yuè) 10th Month
// Between Nov 22 - Dec 22	冬月 (dōng yuè) Winter Month	子月 (zǐ yuè) Rat Month	十一月 (shí yī yuè) 11th Month
// Between Dec 22 - Jan 21	冰月 (bīng yuè) Ice Month	丑月 (chǒu yuè) Ox Month	腊月 (là yuè) End-of-the-Year Month, Preserved Meat Month
const MONTH_NAME_FULL_EN: [&str; 12] = [
    "1st Month",
    "2nd Month",
    "3rd Month",
    "4th Month",
    "5th Month",
    "6th Month",
    "7th Month",
    "8th Month",
    "9th Month",
    "10th Month",
    "11th Month",
    "12th Month",
];

// Mandarin	Pinyin	English
// 夏天	Xià tiān	Summer
// 秋天	Qiū tiān	Autumn
// 冬天	Dōng tiān	Winter
// 春天	Chūn tiān	Spring
const SEASON_NAME_FULL_CN: [&str; 4] = ["春天", "夏天", "秋天", "冬天"];

const SEASON_NAME_FULL_EN: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ChineseCalendar;

impl CalendarSpecificDateView for ChineseCalendar {
    fn new_date(datetime: DateTime<Local>) -> Date {
        let year = datetime.year() as u16;
        let month = datetime.month() as u8;
        let day = datetime.day() as u8;
        let solar_date = SolarDate::from_ymd(year, month, day).unwrap();
        let lunisolar_date = LunisolarDate::from_solar_date(solar_date).unwrap();
        let month_number = lunisolar_date.to_lunar_month().to_u8() as u32;
        Date {
            calendar: Calendar::Chinese(ChineseCalendar),
            weekday: convert_weekday(datetime.weekday()) as u32,
            day: lunisolar_date.to_lunar_day() as u32,
            month: month_number,
            year: lunisolar_date.to_lunisolar_year().to_u16() as i32,
        }
    }

    fn new_date_view(datetime: DateTime<Local>, lang: &Language) -> DateView {
        let year = datetime.year() as u16;
        let month = datetime.month() as u8;
        let day = datetime.day() as u8;
        let weekday = datetime.weekday();
        let solar_date = SolarDate::from_ymd(year, month, day).unwrap();
        let lunisolar_date = LunisolarDate::from_solar_date(solar_date).unwrap();
        let day = lunisolar_date.to_lunar_day();
        let month = lunisolar_date.to_lunar_month();
        let year = lunisolar_date.to_lunisolar_year();
        let month_index = (lunisolar_date.to_lunar_month().to_u8() - 1) as u32;
        let day = match lang {
            Language::Chinese => day.to_string(),
            _ => day.to_u8().to_string(),
        };
        let month = match lang {
            Language::Chinese => format!("{:#}", month), // simplified chinese names
            _ => {
                if month.is_leap_month() {
                    format!("Leap {}", MONTH_NAME_FULL_EN[month_index as usize])
                } else {
                    MONTH_NAME_FULL_EN[month_index as usize].to_string()
                }
            }
        };
        let weekday: WeekDaysUnixOffset = convert_weekday(weekday);
        let weekday = match lang {
            Language::Chinese => WEEKDAY_NAME_FULL_CN[weekday as usize],
            _ => WEEKDAY_NAME_HALF_CAP_EN[weekday as usize],
        };
        let weekday = weekday.to_string();
        let year = match lang {
            Language::Chinese => year.to_string(),
            _ => year.to_u16().to_string(),
        };

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
            Language::Chinese => str_to_vec(&MONTH_NAME_FULL_CN),
            _ => str_to_vec(&MONTH_NAME_FULL_EN),
        };
        let seasons_names: Vec<String> = match lang {
            Language::Chinese => str_to_vec(&SEASON_NAME_FULL_CN),
            _ => str_to_vec(&SEASON_NAME_FULL_EN),
        };
        let calendar_name: String = match lang {
            Language::Chinese => "中国农历".into(),
            _ => "Chinese Calendar".into(),
        };
        CalendarView {
            calendar: CALENDAR_CHINESE,
            calendar_name,
            language: lang.clone().into(),
            direction: lang.default_direction(),
            months_names,
            seasons_names,
        }
    }
}
