////////////////////////
// GREGORIAN CALENDAR //
////////////////////////

pub const GREGORIAN_MONTH_NAME_EN: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub const GREGORIAN_MONTH_NAME_FA: [&str; 12] = [
    "ژانویه",
    "فوریه",
    "مارس",
    "آوریل",
    "می",
    "جون",
    "جولای",
    "آگوست",
    "سپتامبر",
    "اکتبر",
    "نوامبر",
    "دسامبر",
];

/*
Here is the list of Gregorian months in Chinese, using the standard numerical naming convention:
This naming system is straightforward, as it simply combines the number of the month with the character **月** (Yuè), meaning "month."

1. **一月** (Yīyuè) – January
2. **二月** (Èryuè) – February
3. **三月** (Sānyuè) – March
4. **四月** (Sìyuè) – April
5. **五月** (Wǔyuè) – May
6. **六月** (Liùyuè) – June
7. **七月** (Qīyuè) – July
8. **八月** (Bāyuè) – August
9. **九月** (Jiǔyuè) – September
10. **十月** (Shíyuè) – October
11. **十一月** (Shíyīyuè) – November
12. **十二月** (Shí'èryuè) – December
*/

pub const GREGORIAN_MONTH_NAME_ZH: [&str; 12] = [
    "一月",
    "二月",
    "三月",
    "四月",
    "五月",
    "六月",
    "七月",
    "八月",
    "九月",
    "十月",
    "十一月",
    "十二月",
];

// Gregorian months names in Arabic

// January (يَنايِر): Yanāyir
// February (فِبْرايِر): Fibrāyir
// March (مارِس): Māris
// April (أبْريل): Abreel
// May (مايو): Māyu
// June (يُونِيُو): Yūniyū
// July (يُولِيُو): Yūliyū
// August (أغُسْطُس): Aghustus
// September (سِبْتَمْبِر): Sibtambir
// October (أُكْتوبِر): Uktūbir
// November (نُوفَمْبِر): Nūfambir
// December (دِيسَمْبِر): Dīsambir

pub const GREGORIAN_MONTH_NAME_AR: [&str; 12] = [
    "يَنايِر",
    "فِبْرايِر",
    "مارِس",
    "أبْريل",
    "مايو",
    "يُونِيُو",
    "يُولِيُو",
    "أغُسْطُس",
    "سِبْتَمْبِر",
    "أُكْتوبِر",
    "نُوفَمْبِر",
    "دِيسَمْبِر",
];

/////////////////////
// ARABIC CALENDAR //
/////////////////////

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

pub const ARABIC_MONTH_NAME_EN: [&str; 12] = [
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

pub const ARABIC_MONTH_NAME_AR: [&str; 12] = [
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

pub const ARABIC_MONTH_NAME_FA: [&str; 12] = [
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

////////////////////////////
// CHINESE LUNAR CALENDAR //
////////////////////////////

/*

In the traditional **Chinese lunar calendar**, the months are named slightly differently than the Gregorian calendar, often associated with festivals or traditional terms. Below is the standard list of lunar months with their representations:

1. **正月** (Zhēngyuè) – First month, often associated with Chinese New Year.
2. **二月** (Èryuè) – Second month.
3. **三月** (Sānyuè) – Third month.
4. **四月** (Sìyuè) – Fourth month.
5. **五月** (Wǔyuè) – Fifth month, includes the Dragon Boat Festival.
6. **六月** (Liùyuè) – Sixth month.
7. **七月** (Qīyuè) – Seventh month, the "Ghost Month" (includes the Ghost Festival).
8. **八月** (Bāyuè) – Eighth month, includes the Mid-Autumn Festival.
9. **九月** (Jiǔyuè) – Ninth month, includes the Double Ninth Festival.
10. **十月** (Shíyuè) – Tenth month.
11. **冬月** (Dōngyuè) – Eleventh month, "Winter Month."
12. **腊月** (Làyuè) – Twelfth month, "Waxing Month," leading up to the Lunar New Year.

### Notes:
- **正月** (Zhēngyuè) and **腊月** (Làyuè) carry cultural significance, marking the start and end of the lunar year.
- The names for the other months are straightforward numerical terms except for the 11th and 12th months, which have unique names.

In the **Chinese lunar calendar**, a **leap month** (闰月, Rùnyuè) is added approximately every three years to align the lunar calendar with the solar year.

The leap month is represented by adding the character **闰** (Rùn) before the name of the month it repeats. For example:

- **闰正月** (Rùn Zhēngyuè) – Leap First Month
- **闰二月** (Rùn Èryuè) – Leap Second Month
- **闰三月** (Rùn Sānyuè) – Leap Third Month
...and so on, up to **闰腊月** (Rùn Làyuè) – Leap Twelfth Month.

The leap month does not have a fixed position—it depends on astronomical calculations and varies year to year. For instance, in one year, the leap month might be **闰五月** (Leap Fifth Month), while in another, it might be **闰八月** (Leap Eighth Month). This ensures the lunar calendar stays synchronized with the solar year.
*/
pub const CHINESE_LEAP_MONTH_PREFIX_ZH: &str = "闰";
pub const CHINESE_MONTH_NAME_ZH: [&str; 12] = [
    "正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "冬月", "腊月",
];

/*
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
*/

pub const CHINESE_LEAP_MONTH_PREFIX_EN: &str = "Leap-";
pub const CHINESE_MONTH_NAME_EN: [&str; 12] = [
    "1st-Lunar-Month",
    "2nd-Lunar-Month",
    "3rd-Lunar-Month",
    "4th-Lunar-Month",
    "5th-Lunar-Month",
    "6th-Lunar-Month",
    "7th-Lunar-Month",
    "8th-Lunar-Month",
    "9th-Lunar-Month",
    "10th-Lunar-Month",
    "11th-Lunar-Month",
    "12th-Lunar-Month",
];

////////////////////////////
// PERSIAN CALENDAR //
////////////////////////////

pub const PERSIAN_MONTH_NAME_EN: [&str; 12] = [
    "Farvardin",
    "Ordibehesht",
    "Khordad",
    "Tir",
    "Mordad",
    "Shahrivar",
    "Mehr",
    "Aban",
    "Azar",
    "Dey",
    "Bahman",
    "Esfand",
];

pub const PERSIAN_MONTH_NAME_FA: [&str; 12] = [
    "فروردین",
    "اردیبهشت",
    "خرداد",
    "تیر",
    "مرداد",
    "شهریور",
    "مهر",
    "آبان",
    "آذر",
    "دی",
    "بهمن",
    "اسفند",
];
