// In src/page_types/post.rs
use crate::prelude::*;

pub fn construct_post(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    
    let post = match page.specification.clone() {
        UCDPages::BlogPost(post) => post,
        _ => return,
    };
    
    // Add schema for this blog post
    add_blog_post_schema(page, &post);

    css(site);

    let head = site.construct_head(page);
    let date = format_date(&post.frontmatter.date);
    
    // Get the first category for breadcrumbs
    let category = post.frontmatter.category
        .first()
        .map(|c| c.as_str())
        .unwrap_or("Uncategorized");

    let category_slug = slugify(category);
    let author = if !post.frontmatter.author.is_empty() {
        format!(" by {}", post.frontmatter.author)
    } else {
        String::new()
    };
    
    // Build meta HTML with both date and updated
    let meta_html = if date.is_empty() && author.is_empty() && post.frontmatter.updated.is_empty() {
        String::new()
    } else {
        let updated_html = if !post.frontmatter.updated.is_empty() {
            let updated_date = format_date(&post.frontmatter.updated);
            format!(r##"<div class="updated">Last Updated: {updated_date}</div>"##)
        } else {
            String::new()
        };
        
        format!(r##"
            <div class="meta">
                <div class="published">{date}{author}</div>
                {updated_html}
            </div>
        "##)
    };
    
    // Get the sidebar
    let sidebar = construct_sidebar(site, &post.slug);
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-US">
        {head}
        <body>
            {header}
            <div class="content-sidebar-wrap">
                <main class="post">
                    <div class="breadcrumbs">
                        <a href="/">Home</a>
                        <span class="separator">›</span>
                        <a href="/blog">Blog</a>
                        <span class="separator">›</span>
                        <a href="/{category_slug}">{category}</a>
                        
                    </div>
                    <h1>{title}</h1>
                    {meta_html}
                    <div class="image">
                        <img src="{image}" alt="{title}">
                    </div>
                    <div class="content">
                        {content}
                    </div>
                </main>
                {sidebar}
            </div>
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
        title = post.frontmatter.title,
        meta_html = meta_html,
        image = post.frontmatter.image,
        content = post.content,
        category = category,
        sidebar = sidebar,
    );
    
    page.foundation.content = Some(html);
}


fn css(site: &mut Site<UCDPages>) {
    site.declare_css("post", r##"
        .content-sidebar-wrap {
            max-width: 1140px;
            margin: 60px auto 0;
            //padding: 0 30px;
            display: flex;
            gap: 60px;
            align-items: flex-start;
            
            @media (max-width: 968px) {
                flex-direction: column;
                gap: 40px;
            }
        }
        
        main.post {
            flex: 1 1 auto;
            min-width: 0; /* Prevents flex items from overflowing */
            max-width: 702px;
            padding: 0;
            
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
            
            h1 {
                font-size: 30px;
                font-weight: 600;
                line-height: 1.2;
                margin-bottom: 10px;
            }
            
            .meta {
                font-size: 16px;
                color: #666;
                padding-bottom: 20px;
                
                .published {
                    margin-bottom: 5px;
                }
                
                .updated {
                    font-size: 14px;
                    color: #999;
                    font-style: italic;
                }
            }
            
            .image {
                margin-bottom: 30px;
                text-align: center;
                
                img {
                    max-width: 100%;
                    height: auto;
                    display: block;
                    margin: 0 auto;
                }
            }
            
            .content {
                font-size: 18px;
                line-height: 1.625;
                color: #333;
                
                p {
                    margin: 0 0 28px;
                    
                    &:last-child {
                        margin-bottom: 0;
                    }
                }
                
                h2 {
                    font-size: 27px;
                    font-weight: 600;
                    line-height: 1.2;
                    margin: 40px 0 20px;
                    color: #333;
                }
                
                h3 {
                    font-size: 24px;
                    font-weight: 600;
                    line-height: 1.2;
                    margin: 30px 0 20px;
                    color: #333;
                }
                
                h4 {
                    font-size: 20px;
                    font-weight: 600;
                    line-height: 1.2;
                    margin: 25px 0 15px;
                    color: #333;
                }
                
                a {
                    color: var(--link-color);
                    text-decoration: underline;
                    transition: color 0.2s ease-in-out;
                    
                    &:hover {
                        color: var(--link-hover-color);
                        text-decoration: none;
                    }
                }
                
                img {
                    max-width: 100%;
                    height: auto;
                    display: block;
                    margin: 30px auto;
                    border-radius: 3px;
                }
                
                blockquote {
                    margin: 30px 40px;
                    padding: 20px 30px;
                    background-color: #f5f5f5;
                    border-left: 4px solid var(--link-color);
                    font-style: italic;
                    
                    p:last-child {
                        margin-bottom: 0;
                    }
                }
                
                ul, ol {
                    margin: 0 0 30px;
                    padding-left: 40px;
                    
                    li {
                        margin-bottom: 10px;
                        line-height: 1.625;
                    }
                }
                
                ul li {
                    list-style-type: disc;
                }
                
                ol li {
                    list-style-type: decimal;
                }
                
                code {
                    background-color: #f5f5f5;
                    padding: 2px 6px;
                    border-radius: 3px;
                    font-family: 'Monaco', 'Menlo', monospace;
                    font-size: 0.9em;
                }
                
                pre {
                    background-color: #f5f5f5;
                    padding: 20px;
                    border-radius: 5px;
                    overflow-x: auto;
                    margin: 30px 0;
                    
                    code {
                        background: none;
                        padding: 0;
                    }
                }
                
                hr {
                    border: 0;
                    border-bottom: 1px solid #eee;
                    margin: 40px auto;
                    clear: both;
                }
                
                table {
                    width: 100%;
                    border-collapse: collapse;
                    margin: 30px 0;

                    overflow-x: auto;
                    position: relative;
                    display: block;
                    
                    tr {
                        background-color: #fffefc;
                        word-break: normal;
                        
                        &:nth-child(even) {
                            background-color: #fcfce3;
                        }
                    }
                    
                    tbody:only-child {
                        tr {
                            &:nth-child(odd) {
                                background-color: #fcfce3;
                            }
                            
                            &:nth-child(even) {
                                background-color: #fffefc;
                            }
                            
                            &:first-child {
                                background-color: #f2ec2c;
                                font-weight: 600;
                                color: #cd7f32;
                                
                                td {
                                    font-weight: 600;
                                }
                            }
                        }
                    }
                    
                    th {
                        background-color: #f2ec2c;
                        font-weight: 600;
                        color: #cd7f32;
                        text-align: left;
                        padding: 12px;
                    }
                    
                    td {
                        padding: 12px;
                        border: 1px solid #e3d4af;
                        text-align: left;
                        vertical-align: middle;
                    }
                    
                    th {
                        border: 1px solid #e3d4af;
                    }
                }
                
                .alignleft {
                    float: left;
                    margin: 0 20px 20px 0;
                }
                
                .alignright {
                    float: right;
                    margin: 0 0 20px 20px;
                }
                
                .aligncenter {
                    display: block;
                    margin: 30px auto;
                    text-align: center;
                }
                
                figure {
                    margin: 30px 0;
                    
                    img {
                        margin-bottom: 10px;
                    }
                    
                    figcaption {
                        font-size: 14px;
                        color: #666;
                        text-align: center;
                        font-style: italic;
                        padding: 0 10px;
                    }
                }
            }
        }

        @media (max-width: 768px) {
            .content-sidebar-wrap {
                padding: 0 20px;
                margin-top: 30px;
            }
            
            main.post {

                max-width: 100%;

                h1 {
                    font-size: 24px;
                }
                
                .content {
                    font-size: 16px;
                    
                    h2 {
                        font-size: 22px;
                    }
                    
                    h3 {
                        font-size: 20px;
                    }
                    
                    h4 {
                        font-size: 18px;
                    }
                    
                    blockquote {
                        margin: 20px 20px;
                        padding: 15px 20px;
                    }
                    
                    ul, ol {
                        padding-left: 30px;
                    }
                    
                    .alignleft, .alignright {
                        float: none;
                        display: block;
                        margin: 20px auto;
                    }
                }
            }
        }
    "##);
}