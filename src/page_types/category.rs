// src/page_types/category.rs
use crate::prelude::*;

pub fn construct_category(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    
    let (category_name, posts) = match page.specification.clone() {
        UCDPages::Category(name, posts) => (name, posts),
        _ => return,
    };
    
    // Add schema for this category page
    add_category_schema(page, &category_name, &posts);

    let head = site.construct_head(page);

    let mut post_list = Vec::new();

    for post in &posts {  // Borrow here with &
        let title = &post.frontmatter.title;
        let description = &post.frontmatter.description;
        let image = &post.frontmatter.image;
        let date = format_date(&post.frontmatter.date);
        
        // Get the post link from the slug
        let link = format!("/{}", post.slug);
        
        // Get up to two tags
        let tags_html = post.frontmatter.tags
            .iter()
            .take(2)
            .map(|tag| format!(r##"<span class="tag">{}</span>"##, tag))
            .collect::<Vec<_>>()
            .join("");
        
        let image_html = if !image.is_empty() {
            format!(r##"<img src="{image}" alt="{title}">"##)
        } else {
            String::new()
        };

        let post_item = format!(r##"
        <div class="post-card">
            <a href="{link}">
                <div class="image-wrapper">
                    {image_html}
                    <div class="overlay">
                        <div class="category">{category_name}</div>
                        <h3 class="title">{title}</h3>
                    </div>
                </div>
                <div class="content">
                    <p>{description}</p>
                    <div class="meta">
                        <span class="date">{date}</span>
                        <div class="tags">
                            {tags_html}
                        </div>
                    </div>
                </div>
            </a>
        </div>
    "##);

        post_list.push(post_item);
    }

    // Join all post items into a single string
    let posts_html = post_list.join("\n");
    
    let post_count = posts.len();
    let description = format!("Discover {} posts about {}", post_count, category_name.to_lowercase());
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-US">
        {head}
        <body>
            {header}
            <main class="category">
                <div class="breadcrumbs">
                    <a href="/">Home</a>
                    <span class="separator">›</span>
                    <a href="/blog">Blog</a>
                    <span class="separator">›</span>
                    <span>{category_name}</span>
                </div>
                <div class="category-header">
                    <h1>{category_name}</h1>
                    <p>{description}</p>
                </div>
                <div class="posts-grid">
                    {posts_html}
                </div>
            </main>
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
        posts_html = posts_html,
        category_name = category_name,
        description = description,
    );
    
    css(site);
    
    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("category", r##"
        main.category {
            max-width: 1000px;
            
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
        
        .category-header {
            text-align: center;
            padding: 40px var(--site-padding-x) 40px;
            max-width: 800px;
            margin: 0 auto;
            
            h1 {
                font-size: 36tem;
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
        
        /* Reusing the posts-grid styles from blog.rs */
        .posts-grid {
            max-width: 1200px;
            margin: 40px auto;
            padding: 0 var(--site-padding-x);
            columns: 3;
            column-gap: 20px;
            
            @media (max-width: 968px) {
                columns: 2;
            }
            
            @media (max-width: 640px) {
                columns: 1;
            }
        }
        
        .post-card {
            break-inside: avoid;
            margin-bottom: 20px;
            background: white;
            border-radius: 12px;
            overflow: hidden;
            box-shadow: 0 3px 15px rgba(0,0,0,0.08);
            transition: all 0.3s ease;
            cursor: pointer;
            animation: fadeIn 0.6s ease-out;
            
            &:hover {
                transform: translateY(-5px);
                box-shadow: 0 8px 25px rgba(0,0,0,0.15);
            }
            
            a {
                text-decoration: none;
                color: inherit;
                display: block;
            }
            
            .image-wrapper {
                position: relative;
                overflow: hidden;
                
                img {
                    width: 100%;
                    height: auto;
                    display: block;
                    transition: transform 0.5s ease;
                }
                
                &:hover img {
                    transform: scale(1.05);
                }
            }
            
            .overlay {
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: linear-gradient(to bottom, transparent 0%, rgba(0,0,0,0.7) 100%);
                display: flex;
                flex-direction: column;
                justify-content: flex-end;
                padding: 20px;
                
                .category {
                    color: var(--primary);
                    font-size: 11tem;
                    text-transform: uppercase;
                    letter-spacing: 1px;
                    margin-bottom: 5px;
                }
                
                .title {
                    color: white;
                    font-size: 18tem;
                    font-weight: 500;
                    line-height: 1.3;
                    margin: 0;
                }
            }
            
            .content {
                padding: 20px;
                
                p {
                    font-size: 14tem;
                    color: #666;
                    line-height: 1.6;
                    margin-bottom: 15px;
                }
            }
            
            .meta {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding-top: 15px;
                border-top: 1px solid #f0f0f0;
                
                .date {
                    font-size: 12tem;
                    color: #999;
                }
                
                .tags {
                    display: flex;
                    gap: 8px;
                    
                    .tag {
                        display: inline-block;
                        padding: 4px 12px;
                        background: var(--primary);
                        color: white;
                        font-size: 11tem;
                        border-radius: 15px;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    }
                }
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
    "##);
}