use crate::prelude::*;

pub fn add_decrees<T, I>(site: &mut Site<T, I>) {
 
    site.declare_decree("[YEAR]", &get_year().to_string());

    site.declare_decree("[SITE_NAME]", SITE_NAME);
    
    
}