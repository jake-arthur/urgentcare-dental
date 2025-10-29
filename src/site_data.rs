use crate::prelude::*;
use time::OffsetDateTime;
use std::sync::OnceLock;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum UCDPages {
    Homepage,
    BlogPost(PostData<UCDFrontmatter>),
    PostList,
    Category(String, Vec<PostData<UCDFrontmatter>>), // Category name and its posts
    CategoriesIndex,
}

pub const SITE_NAME: &str = "UrgentCare Dental";
pub const SITE_NAME_BRANDED: &str = r##"<span class="site-name-branded"><span class="urgent">Urgent</span><span class="care">Care</span> <span class="dental">Dental</span></span>"##;
pub const SITE_URL: &str = "https://urgentcaredental.co.uk/";
pub const SITE_DESCRIPTION: &str = "This is your 24/7 emergency dentist in Northern England. Learn how to get immediate dental pain relief with our £20 assessment and same-day treatment at our Leeds and Manchester clinics.";
pub const PHONE_NUMBER: &str = "0113 868 3185";
pub const PHONE_NUMBER_LINK: &str = "tel:+441138683185";
pub const BOOKING_LINK: &str = "https://urgentcaredental.setmore.com";

pub static YEAR: OnceLock<u32> = OnceLock::new();

pub fn get_year() -> u32 {
    *YEAR.get_or_init(|| {
        let now = OffsetDateTime::now_utc();
        let current_year = now.year() as u32;
        
        if now.month() >= time::Month::October {
            current_year + 1
        } else {
            current_year
        }
    })
}

pub const COPYRIGHT_YEAR: &str = "2025";