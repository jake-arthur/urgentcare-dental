use crate::prelude::*;

pub fn construct_sidebar(site: &mut Site<UCDPages>, current_slug: &str) -> String {
    // Get all posts
    let all_posts = get_all_posts();
    
    // Get featured posts (exclude current post)
    let featured_posts: Vec<_> = all_posts
        .iter()
        .filter(|p| p.frontmatter.featured && p.slug != current_slug)
        .take(8)
        .collect();
    
    // Get recent posts (exclude current post)
    let recent_posts: Vec<_> = all_posts
        .iter()
        .filter(|p| p.slug != current_slug)
        .take(15)
        .collect();
    
    // Get recently updated posts (exclude current post, and only posts with updated field)
    let mut updated_posts: Vec<_> = all_posts
        .iter()
        .filter(|p| !p.frontmatter.updated.is_empty() && p.slug != current_slug)
        .collect();
    
    // Sort by updated date (newest first)
    updated_posts.sort_by(|a, b| b.frontmatter.updated.cmp(&a.frontmatter.updated));
    
    // Take only the top 5
    let recently_updated: Vec<_> = updated_posts.into_iter().take(15).collect();
    
    // Build featured posts section
    let featured_section = if !featured_posts.is_empty() {
        let posts_items = featured_posts
            .iter()
            .map(|p| {
                format!(r##"
                    <div class="sidebar-post">
                        <a href="/{slug}">
                            <div class="sidebar-image-wrapper">
                                <img src="{image}" alt="{title}">
                                <div class="sidebar-overlay">
                                    <h3>{title}</h3>
                                </div>
                            </div>
                        </a>
                    </div>
                "##,
                    slug = p.slug,
                    image = p.frontmatter.image,
                    title = p.frontmatter.title,
                )
            })
            .collect::<Vec<_>>()
            .join("");
        
        format!(r##"
            <div class="widget">
                <h2 class="widget-title">Featured Posts</h2>
                <div class="featured-posts">
                    {posts_items}
                </div>
            </div>
        "##)
    } else {
        String::new()
    };
    
    // Build recent posts section
    let recent_section = if !recent_posts.is_empty() {
        let posts_items = recent_posts
            .iter()
            .map(|p| {
                format!(r##"
                    <div class="sidebar-post">
                        <a href="/{slug}">
                            <div class="sidebar-image-wrapper">
                                <img src="{image}" alt="{title}">
                                <div class="sidebar-overlay">
                                    <h3>{title}</h3>
                                </div>
                            </div>
                        </a>
                    </div>
                "##,
                    slug = p.slug,
                    image = p.frontmatter.image,
                    title = p.frontmatter.title,
                )
            })
            .collect::<Vec<_>>()
            .join("");
        
        format!(r##"
            <div class="widget">
                <h2 class="widget-title">Recent Posts</h2>
                <div class="recent-posts">
                    {posts_items}
                </div>
            </div>
        "##)
    } else {
        String::new()
    };
    
    // Build recently updated section
    let updated_section = if !recently_updated.is_empty() {
        let posts_items = recently_updated
            .iter()
            .map(|p| {
                let updated_date = format_date(&p.frontmatter.updated);
                format!(r##"
                    <div class="sidebar-updated-post">
                        <a href="/{slug}">
                            <div class="sidebar-updated-content">
                                <h3>{title}</h3>
                                <span class="updated-date">Updated: {updated_date}</span>
                            </div>
                        </a>
                    </div>
                "##,
                    slug = p.slug,
                    title = p.frontmatter.title,
                    updated_date = updated_date,
                )
            })
            .collect::<Vec<_>>()
            .join("");
        
        format!(r##"
            <div class="widget">
                <h2 class="widget-title">Recently Updated</h2>
                <div class="recently-updated">
                    {posts_items}
                </div>
            </div>
        "##)
    } else {
        String::new()
    };
    
    // Add CSS for sidebar
    site.declare_css("sidebar", r##"
        .sidebar {
            margin-top: 142px;
            width: 300px;
            flex: 0 0 300px; /* Don't grow or shrink, stay 300px */
            
            @media (max-width: 968px) {
                width: 100%;
                flex: 1 1 auto;
                margin-top: 60px;
            }
            
            .widget {
                margin-bottom: 40px;
                
                &:last-child {
                    margin-bottom: 0;
                }
                
                &-title {
                    font-size: 18tem;
                    font-weight: 600;
                    margin-bottom: 20px;
                    color: #333;
                }
            }
            
            .featured-posts,
            .recent-posts {
                .sidebar-post {
                    margin-bottom: 20px;
                    
                    &:last-child {
                        margin-bottom: 0;
                    }
                    
                    a {
                        text-decoration: none;
                        color: inherit;
                        display: block;
                    }
                    
                    .sidebar-image-wrapper {
                        position: relative;
                        overflow: hidden;
                        border-radius: 8px;
                        aspect-ratio: 16/9;
                        
                        img {
                            width: 100%;
                            height: 100%;
                            object-fit: cover;
                            display: block;
                            transition: transform 0.3s ease;
                        }
                        
                        &:hover img {
                            transform: scale(1.05);
                        }
                        
                        .sidebar-overlay {
                            position: absolute;
                            bottom: 0;
                            left: 0;
                            right: 0;
                            background: linear-gradient(to top, rgba(0,0,0,0.8) 0%, transparent 100%);
                            padding: 15px;
                            
                            h3 {
                                color: white;
                                font-size: 14tem;
                                font-weight: 500;
                                line-height: 1.3;
                                margin: 0;
                            }
                        }
                    }
                }
            }
            
            .recently-updated {
                .sidebar-updated-post {
                    margin-bottom: 15px;
                    padding: 12px 15px;
                    background: #f8f8f8;
                    border-radius: 8px;
                    transition: all 0.3s ease;
                    
                    &:last-child {
                        margin-bottom: 0;
                    }
                    
                    &:hover {
                        background: #f0f0f0;
                        transform: translateX(5px);
                    }
                    
                    a {
                        text-decoration: none;
                        color: inherit;
                        display: block;
                    }
                    
                    .sidebar-updated-content {
                        h3 {
                            font-size: 15tem;
                            font-weight: 500;
                            line-height: 1.3;
                            margin: 0 0 5px 0;
                            color: #333;
                            transition: color 0.3s ease;
                        }
                        
                        .updated-date {
                            font-size: 12tem;
                            color: #999;
                            font-style: italic;
                        }
                    }
                    
                    &:hover h3 {
                        color: var(--link-color);
                    }
                }
            }
        }
    "##);
    
    // Return the complete sidebar HTML
    if !featured_section.is_empty() || !recent_section.is_empty() || !updated_section.is_empty() {
        format!(r##"
            <aside class="sidebar">
                {featured_section}
                {recent_section}
                {updated_section}
            </aside>
        "##)
    } else {
        String::new()
    }
}