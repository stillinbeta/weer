use std::fmt::{self, Display};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Language {
    #[serde(rename="ar")]
    Arabic,
    #[serde(rename="bn")]
    Bengali,
    #[serde(rename="bg")]
    Bulgarian,
    #[serde(rename="zh")]
    ChineseSimplified,
    #[serde(rename="zh_tw")]
    ChineseTraditional,
    #[serde(rename="cs")]
    Czech,
    #[serde(rename="da")]
    Danish,
    #[serde(rename="nl")]
    Dutch,
    #[serde(rename="fi")]
    Finnish,
    #[serde(rename="fr")]
    French,
    #[serde(rename="de")]
    German,
    #[serde(rename="el")]
    Greek,
    #[serde(rename="hi")]
    Hindi,
    #[serde(rename="hu")]
    Hungarian,
    #[serde(rename="it")]
    Italian,
    #[serde(rename="ja")]
    Japanese,
    #[serde(rename="jv")]
    Javanese,
    #[serde(rename="ko")]
    Korean,
    #[serde(rename="zh_cmn")]
    Mandarin,
    #[serde(rename="mr")]
    Marathi,
    #[serde(rename="pl")]
    Polish,
    #[serde(rename="pt")]
    Portuguese,
    #[serde(rename="pa")]
    Punjabi,
    #[serde(rename="ro")]
    Romanian,
    #[serde(rename="ru")]
    Russian,
    #[serde(rename="sr")]
    Serbian,
    #[serde(rename="si")]
    Sinhalese,
    #[serde(rename="sk")]
    Slovak,
    #[serde(rename="es")]
    Spanish,
    #[serde(rename="sv")]
    Swedish,
    #[serde(rename="ta")]
    Tamil,
    #[serde(rename="te")]
    Telugu,
    #[serde(rename="tr")]
    Turkish,
    #[serde(rename="uk")]
    Ukrainian,
    #[serde(rename="ur")]
    Urdu,
    #[serde(rename="vi")]
    Vietnamese,
    #[serde(rename="zh_wuu")]
    WuShanghainese,
    #[serde(rename="zh_hsn")]
    Xiang,
    #[serde(rename="zh_yue")]
    YueCantonese,
    #[serde(rename="zu")]
    Zulu
}

impl Language {
    pub fn new(lang: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_value(serde_json::json![lang])
    }

    pub fn content(&self) -> &str {
        match self {
            Language::Arabic => "ar",
            Language::Bengali => "bn",
            Language::Bulgarian => "bg",
            Language::ChineseSimplified => "zh",
            Language::ChineseTraditional => "zh_tw",
            Language::Czech => "cs",
            Language::Danish => "da",
            Language::Dutch => "nl",
            Language::Finnish => "fi",
            Language::French => "fr",
            Language::German => "de",
            Language::Greek => "el",
            Language::Hindi => "hi",
            Language::Hungarian => "hu",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Javanese => "jv",
            Language::Korean => "ko",
            Language::Mandarin => "zh_cmn",
            Language::Marathi => "mr",
            Language::Polish => "pl",
            Language::Portuguese => "pt",
            Language::Punjabi => "pa",
            Language::Romanian => "ro",
            Language::Russian => "ru",
            Language::Serbian => "sr",
            Language::Sinhalese => "si",
            Language::Slovak => "sk",
            Language::Spanish => "es",
            Language::Swedish => "sv",
            Language::Tamil => "ta",
            Language::Telugu => "te",
            Language::Turkish => "tr",
            Language::Ukrainian => "uk",
            Language::Urdu => "ur",
            Language::Vietnamese => "vi",
            Language::WuShanghainese => "zh_wuu",
            Language::Xiang => "zh_hsn",
            Language::YueCantonese => "zh_yue",
            Language::Zulu => "zu"
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}
