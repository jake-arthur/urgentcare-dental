use citadel::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UCDFrontmatter {
    pub title: String,
    pub date: String,
    pub updated: String,  // Add this field
    pub author: String,
    pub description: String,
    pub category: Vec<String>,
    pub tags: Vec<String>,
    pub image: String,
    pub featured: bool,
    pub exclude: bool,
}

pub fn parse_urgentcaredental_frontmatter(content: &str) -> Result<(UCDFrontmatter, String), Box<dyn std::error::Error>> {
    let content = content.trim_start();  // Trim any leading whitespace
    
    if !content.starts_with("+++") {
        return Err("No TOML frontmatter found".into());
    }
    
    let end_marker = content[3..].find("+++").ok_or("No closing frontmatter marker")?;
    let frontmatter_content = &content[3..end_marker + 3];
    let markdown_content = content[end_marker + 6..].trim_start();
    
    let mut frontmatter = UCDFrontmatter::default();
    
    // Simple TOML parsing
    for line in frontmatter_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            
            match key {
                "title" => frontmatter.title = value.to_owned(),
                "date" => frontmatter.date = value.to_owned(),
                "updated" | "date_modified" | "modified" => frontmatter.updated = value.to_owned(),  // Check both
                "author" => frontmatter.author = value.to_owned(),
                "description" => frontmatter.description = value.to_owned(),
                "category" | "categories" => {
                    // Handle both single category and array syntax
                    if value.starts_with('[') && value.ends_with(']') {
                        // Array syntax: ["cat1", "cat2"]
                        let cats = value.trim_matches('[').trim_matches(']');
                        frontmatter.category = cats.split(',')
                            .map(|s| normalize_category(s.trim().trim_matches('"')))
                            .filter(|s| !s.is_empty())
                            .collect();
                    } else {
                        // Single category: "Crafts" or just Crafts
                        let single_cat = normalize_category(value.trim_matches('"').trim_matches('\''));
                        if !single_cat.is_empty() {
                            frontmatter.category = vec![single_cat];
                        }
                    }
                },
                "tags" => {
                    let tags = value.trim_matches('[').trim_matches(']');
                    frontmatter.tags = tags.split(',')
                        .map(|s| s.trim().trim_matches('"').to_owned())
                        .filter(|s| !s.is_empty())
                        .collect();
                },
                "image" | "featured_image" => frontmatter.image = value.to_owned(),
                "featured" => frontmatter.featured = value.trim().to_lowercase() == "true",
                "exclude" => frontmatter.exclude = value.trim().to_lowercase() == "true",
                _ => {}
            }
        }
    }
    
    Ok((frontmatter, markdown_content.to_owned()))
}

pub fn get_all_posts() -> Vec<PostData<UCDFrontmatter>> {
    let mut posts = load_content("content", parse_urgentcaredental_frontmatter);
    
    let today = get_today();

    // Filter out excluded posts AND future posts
    posts.retain(|post| {
        !post.frontmatter.exclude && 
        !is_future_date(&post.frontmatter.date, &today)
    });
    
    // Sort by date (newest first)
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
    
    posts
}

pub fn get_all_posts_and_post_pages() -> Vec<PostData<UCDFrontmatter>> {
    let mut posts = load_content("content", parse_urgentcaredental_frontmatter);
    
    let today = get_today();
    
    // Filter out posts in the future
    posts.retain(|post| !is_future_date(&post.frontmatter.date, &today));
    
    // Sort by date (newest first)
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
    
    posts
}


fn get_today() -> (u32, u32, u32) {
    // Use time just to get today's date parts
    let today = time::OffsetDateTime::now_utc().date();
    (today.year() as u32, today.month() as u32, today.day() as u32)
}

fn is_future_date(date_str: &str, today: &(u32, u32, u32)) -> bool {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 { return false; }
    
    let year: u32 = parts[0].parse().unwrap_or(0);
    let month: u32 = parts[1].parse().unwrap_or(0);
    let day: u32 = parts[2].parse().unwrap_or(0);
    
    // Compare year, month, day in order
    year > today.0 || 
    (year == today.0 && month > today.1) ||
    (year == today.0 && month == today.1 && day > today.2)
}

fn normalize_category(cat: &str) -> String {
    // Convert "tool-comparison" to "Tool Comparison"
    cat.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}