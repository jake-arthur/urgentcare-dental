// src/page_types/categories.rs
use crate::prelude::*;
use std::collections::HashMap;

pub fn construct_categories(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    
    let head = site.construct_head(page);
    
    // Get all posts and build category map
    let all_posts = get_all_posts();
    let mut categories: HashMap<String, usize> = HashMap::new();
    
    // Count posts per category
    for post in &all_posts {
        if post.frontmatter.category.is_empty() {
            // Count uncategorized posts
            *categories.entry("Uncategorized".to_owned()).or_insert(0) += 1;
        } else {
            // Count categorized posts
            for category in &post.frontmatter.category {
                *categories.entry(category.clone()).or_insert(0) += 1;
            }
        }
    }
    
    // Sort categories alphabetically
    let mut sorted_categories: Vec<(String, usize)> = categories.into_iter().collect();
    sorted_categories.sort_by(|a, b| a.0.cmp(&b.0));
    
    // Build category cards
    let mut category_cards = Vec::new();
    for (category_name, count) in sorted_categories {
        let category_slug = slugify(&category_name);
        let post_word = if count == 1 { "post" } else { "posts" };
        
        let card = format!(r##"
            <a href="/{category_slug}" class="category-card">
                <div class="category-content">
                    <h3>{category_name}</h3>
                    <span class="post-count">{count} {post_word}</span>
                </div>
                <div class="arrow">→</div>
            </a>
        "##);
        
        category_cards.push(card);
    }
    
    let categories_html = category_cards.join("\n");
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-US">
        {head}
        <body>
            {header}
            <main class="categories-index">
                <div class="breadcrumbs">
                    <a href="/">Home</a>
                    <span class="separator">›</span>
                    <span>Categories</span>
                </div>
                <div class="categories-header">
                    <h1>Categories</h1>
                    <p>Browse content by topic</p>
                </div>
                <div class="categories-grid">
                    {categories_html}
                </div>
            </main>
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
    );
    
    css(site);
    
    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("categories_index", r##"
        main.categories-index {
            max-width: 1000px;
            margin: 60px auto 0;
            padding: 0 30px;
            
            .breadcrumbs {
                font-size: 16px;
                margin-bottom: 40px;
                padding-bottom: 10px;
                border-bottom: 1px solid #eee;
                
                a {
                    color: var(--link-color);
                    text-decoration: none;
                    
                    &:hover {
                        color: var(--link-hover-color);
                        text-decoration: underline;
                    }
                }
                
                .separator {
                    margin: 0 10px;
                    color: #999;
                }
            }
        }
        
        .categories-header {
            text-align: center;
            padding: 40px var(--site-padding-x) 60px;
            max-width: 600px;
            margin: 0 auto;
            
            h1 {
                font-size: 42tem;
                font-weight: 400;
                letter-spacing: 1px;
                margin-bottom: 15px;
                color: var(--primary);
            }
            
            p {
                font-size: 18tem;
                color: #666;
                line-height: 1.6;
            }
        }
        
        .categories-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
            gap: 20px;
            padding: 0 var(--site-padding-x) 60px;
            
            @media (max-width: 640px) {
                grid-template-columns: 1fr;
            }
        }
        
        .category-card {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 25px 30px;
            background: white;
            border: 2px solid #f0f0f0;
            border-radius: 12px;
            text-decoration: none;
            transition: all 0.3s ease;
            cursor: pointer;
            
            &:hover {
                border-color: var(--primary);
                transform: translateY(-3px);
                box-shadow: 0 10px 30px rgba(0,0,0,0.1);
                
                .arrow {
                    transform: translateX(5px);
                    color: var(--primary);
                }
            }
            
            .category-content {
                h3 {
                    font-size: 20tem;
                    font-weight: 500;
                    color: #333;
                    margin: 0 0 8px 0;
                    transition: color 0.3s ease;
                }
                
                .post-count {
                    font-size: 14tem;
                    color: #999;
                    font-weight: 400;
                }
            }
            
            .arrow {
                font-size: 24tem;
                color: #ccc;
                transition: all 0.3s ease;
            }
        }
        
        @keyframes fadeIn {
            from {
                opacity: 0;
                transform: translateY(20px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }
        
        .category-card {
            animation: fadeIn 0.5s ease-out;
            animation-fill-mode: both;
            
            &:nth-child(1) { animation-delay: 0.05s; }
            &:nth-child(2) { animation-delay: 0.1s; }
            &:nth-child(3) { animation-delay: 0.15s; }
            &:nth-child(4) { animation-delay: 0.2s; }
            &:nth-child(5) { animation-delay: 0.25s; }
            &:nth-child(6) { animation-delay: 0.3s; }
            &:nth-child(7) { animation-delay: 0.35s; }
            &:nth-child(8) { animation-delay: 0.4s; }
        }
    "##);
}