use crate::config::config;
use crate::mail_list_utils::MailList;

pub struct RonUtils {}

impl RonUtils {
    pub fn load_config() -> config {
        let ron_string = std::fs::read_to_string("config.ron").unwrap();
        let result: config = ron::de::from_str(&ron_string).unwrap();
        result
    }

    pub fn save_mail_list(list: &MailList) {
        let ron_string =
            ron::ser::to_string_pretty(&list, ron::ser::PrettyConfig::default()).unwrap();

        std::fs::write("mail_list.ron", ron_string).unwrap();
    }

    pub fn load_mail_list() -> MailList {
        let ron_string = std::fs::read_to_string("mail_list.ron").unwrap();
        let result: MailList = ron::de::from_str(&ron_string).unwrap();
        result
    }
}
