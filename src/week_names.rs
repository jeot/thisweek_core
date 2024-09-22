/* week names are general to calendars and specific to language! */
/* the order is defined by WeekDaysUnixOffset enum */

const WEEKDAY_NAME_FULL_EN: [&str; 7] = [
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
];

const WEEKDAY_NAME_HALF_EN: [&str; 7] = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];

const WEEKDAY_NAME_HALF_CAP_EN: [&str; 7] = ["THU", "FRI", "SAT", "SUN", "MON", "TUE", "WED"];

const WEEKDAY_NAME_FULL_FA: [&str; 7] = [
    "پنج شنبه",
    "جمعه",
    "شنبه",
    "یک شنبه",
    "دوشنبه",
    "سه شنبه",
    "چهارشنبه",
];

// In Arabic	Pronounce	In English
// السبت	As sabt	Saturday
// الأحد	Al ‘ahad	Sunday
// الأثنين	A lith nayn	Monday
// الثلاثاء	Ath thu la tha’	Tuesday
// الأربعاء	Al ar ba a’	Wednesday
// الخميس	Al kha mis	Thursday
// الجمعه	Al jum ah	Friday
const WEEKDAY_NAME_FULL_AR: [&str; 7] = [
    "الخميس",
    "الجمعه",
    "السبت",
    "الأحد",
    "الأثنين",
    "الثلاثاء",
    "الأربعاء",
];

// Chinese	Pinyin	English
// 星期一 	xīng qī yī	Monday
// 星期二 	xīng qī èr	Tuesday
// 星期三 	xīng qī sān	Wednesday
// 星期四 	xīng qī sì	Thursday
// 星期五 	xīng qī wǔ	Friday
// 星期六 	xīng qī liù	Saturday
// 星期日  / 星期天 	xīng qī rì / xīng qī tiān	Sunday

const WEEKDAY_NAME_FULL_CN: [&str; 7] = [
    "星期四",
    "星期五",
    "星期六",
    "星期日",
    "星期一",
    "星期二",
    "星期三",
];
