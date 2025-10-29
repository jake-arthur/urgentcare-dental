use crate::prelude::*;
use serde_json::json;

pub fn add_sitewide_schema<T, I>(site: &mut Site<T, I>) {
    let organization_schema = json!({
        "@context": "https://schema.org",
        "@type": "Organization",
        "name": SITE_NAME,
        "url": SITE_URL,
        "description": SITE_DESCRIPTION,
        "logo": {
            "@type": "ImageObject",
            "url": format!("{}images/logo.png", SITE_URL)
        },
        "sameAs": []
    });
    
    let website_schema = json!({
        "@context": "https://schema.org",
        "@type": "WebSite",
        "name": SITE_NAME,
        "url": SITE_URL,
        "publisher": {
            "@type": "Organization",
            "name": SITE_NAME,
            "logo": {
                "@type": "ImageObject",
                "url": format!("{}images/logo.png", SITE_URL)
            }
        },
        "potentialAction": {
            "@type": "SearchAction",
            "target": {
                "@type": "EntryPoint",
                "urlTemplate": format!("{}search?q={{search_term_string}}", SITE_URL)
            },
            "query-input": "required name=search_term_string"
        }
    });
    
    let weblog_schema = json!({
        "@context": "https://schema.org",
        "@type": "Blog",
        "name": SITE_NAME,
        "url": SITE_URL,
        "description": SITE_DESCRIPTION,
        "publisher": {
            "@type": "Organization",
            "name": SITE_NAME,
            "logo": {
                "@type": "ImageObject",
                "url": format!("{}images/logo.png", SITE_URL)
            }
        }
    });
    
    let graph = json!({
        "@context": "https://schema.org",
        "@graph": [organization_schema, website_schema, weblog_schema]
    });
    
    let schema_script = format!(
        r##"<script type="application/ld+json">{}</script>"##,
        graph.to_string()
    );
    
    site.declare_placement(PlacementPosition::Schema, &schema_script);
}

pub fn add_blog_post_schema(
    page: &mut Page<UCDPages>,
    post: &PostData<UCDFrontmatter>
) {
    // Format dates to ISO 8601 - handle dates that may already have T in them
    let date_published = if !post.frontmatter.date.is_empty() {
        if post.frontmatter.date.contains('T') {
            post.frontmatter.date.clone()
        } else {
            format!("{}T00:00:00+00:00", post.frontmatter.date)
        }
    } else {
        String::new()
    };
    
    let date_modified = if !post.frontmatter.updated.is_empty() {
        if post.frontmatter.updated.contains('T') {
            post.frontmatter.updated.clone()
        } else {
            format!("{}T00:00:00+00:00", post.frontmatter.updated)
        }
    } else if !date_published.is_empty() {
        date_published.clone()
    } else {
        String::new()
    };
    
    // Use author if set, otherwise use Organization
    let author = if !post.frontmatter.author.is_empty() {
        json!({
            "@type": "Person",
            "name": post.frontmatter.author
        })
    } else {
        json!({
            "@type": "Organization",
            "name": SITE_NAME,
            "logo": {
                "@type": "ImageObject",
                "url": format!("{}images/logo.png", SITE_URL)
            }
        })
    };
    
    // Build image array for better SEO
    let image = if !post.frontmatter.image.is_empty() {
        json!([
            post.frontmatter.image.clone(),
            {
                "@type": "ImageObject",
                "url": post.frontmatter.image.clone(),
                "width": 1200,
                "height": 630
            }
        ])
    } else {
        json!([format!("{}images/logo.png", SITE_URL)])
    };
    
    let article_schema = json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "mainEntityOfPage": {
            "@type": "WebPage",
            "@id": format!("{}{}", SITE_URL, post.slug)
        },
        "headline": post.frontmatter.title.clone(),
        "description": post.frontmatter.description.clone(),
        "image": image,
        "author": author,
        "publisher": {
            "@type": "Organization",
            "name": SITE_NAME,
            "logo": {
                "@type": "ImageObject",
                "url": format!("{}images/logo.png", SITE_URL),
                "width": 600,
                "height": 60
            }
        },
        "datePublished": date_published,
        "dateModified": date_modified,
        "articleSection": post.frontmatter.category.first().unwrap_or(&"General".to_owned()).clone(),
        "keywords": post.frontmatter.tags.join(", "),
        "wordCount": post.content.split_whitespace().count(),
        "inLanguage": "en-US"
    });
    
    // Add breadcrumb schema
    let category = post.frontmatter.category
        .first()
        .map(|c| c.as_str())
        .unwrap_or("Uncategorized");
    
    let breadcrumb_schema = json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            {
                "@type": "ListItem",
                "position": 1,
                "name": "Home",
                "item": SITE_URL
            },
            {
                "@type": "ListItem",
                "position": 2,
                "name": "Blog",
                "item": format!("{}blog", SITE_URL)
            },
            {
                "@type": "ListItem",
                "position": 3,
                "name": category,
                "item": format!("{}{}", SITE_URL, slugify(category))
            },
            {
                "@type": "ListItem",
                "position": 4,
                "name": post.frontmatter.title.clone()
            }
        ]
    });
    
    // Add FAQ schema if the content has headers that look like questions
    let faq_items = extract_faq_from_content(&post.content);
    let faq_schema = if !faq_items.is_empty() {
        Some(json!({
            "@context": "https://schema.org",
            "@type": "FAQPage",
            "mainEntity": faq_items
        }))
    } else {
        None
    };
    
    let mut graph_items = vec![article_schema, breadcrumb_schema];
    if let Some(faq) = faq_schema {
        graph_items.push(faq);
    }
    
    let graph = json!({
        "@context": "https://schema.org",
        "@graph": graph_items
    });
    
    let schema_script = format!(
        r##"<script type="application/ld+json">{}</script>"##,
        graph.to_string()
    );
    
    page.declare_placement(PlacementPosition::Schema, &schema_script);
}

pub fn add_blog_index_schema(
    page: &mut Page<UCDPages>,
    posts: &[PostData<UCDFrontmatter>]
) {
    // Build item list of all blog posts
    let post_items: Vec<serde_json::Value> = posts.iter().enumerate().map(|(index, post)| {
        let date_published = if !post.frontmatter.date.is_empty() {
            if post.frontmatter.date.contains('T') {
                post.frontmatter.date.clone()
            } else {
                format!("{}T00:00:00+00:00", post.frontmatter.date)
            }
        } else {
            String::new()
        };
        
        json!({
            "@type": "ListItem",
            "position": index + 1,
            "item": {
                "@type": "BlogPosting",
                "headline": post.frontmatter.title.clone(),
                "url": format!("{}{}", SITE_URL, post.slug),
                "description": post.frontmatter.description.clone(),
                "datePublished": date_published,
                "author": if !post.frontmatter.author.is_empty() {
                    json!({
                        "@type": "Person",
                        "name": post.frontmatter.author.clone()
                    })
                } else {
                    json!({
                        "@type": "Organization",
                        "name": SITE_NAME
                    })
                }
            }
        })
    }).collect();
    
    let collection_schema = json!({
        "@context": "https://schema.org",
        "@type": "CollectionPage",
        "name": format!("{} Blog", SITE_NAME),
        "description": SITE_DESCRIPTION,
        "url": format!("{}blog", SITE_URL),
        "isPartOf": {
            "@type": "Blog",
            "name": SITE_NAME,
            "url": SITE_URL
        },
        "publisher": {
            "@type": "Organization",
            "name": SITE_NAME,
            "logo": {
                "@type": "ImageObject",
                "url": format!("{}images/logo.png", SITE_URL)
            }
        },
        "numberOfItems": posts.len(),
        "mainEntity": {
            "@type": "ItemList",
            "itemListElement": post_items
        },
        "inLanguage": "en-US"
    });
    
    let breadcrumb_schema = json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            {
                "@type": "ListItem",
                "position": 1,
                "name": "Home",
                "item": SITE_URL
            },
            {
                "@type": "ListItem",
                "position": 2,
                "name": "Blog"
            }
        ]
    });
    
    let graph = json!({
        "@context": "https://schema.org",
        "@graph": [collection_schema, breadcrumb_schema]
    });
    
    let schema_script = format!(
        r##"<script type="application/ld+json">{}</script>"##,
        graph.to_string()
    );
    
    page.declare_placement(PlacementPosition::Schema, &schema_script);
}

pub fn add_category_schema(
    page: &mut Page<UCDPages>,
    category_name: &str,
    posts: &[PostData<UCDFrontmatter>]
) {
    // Build item list of all posts in this category
    let post_items: Vec<serde_json::Value> = posts.iter().enumerate().map(|(index, post)| {
        let date_published = if !post.frontmatter.date.is_empty() {
            if post.frontmatter.date.contains('T') {
                post.frontmatter.date.clone()
            } else {
                format!("{}T00:00:00+00:00", post.frontmatter.date)
            }
        } else {
            String::new()
        };
        
        json!({
            "@type": "ListItem",
            "position": index + 1,
            "item": {
                "@type": "BlogPosting",
                "headline": post.frontmatter.title.clone(),
                "url": format!("{}{}", SITE_URL, post.slug),
                "description": post.frontmatter.description.clone(),
                "datePublished": date_published
            }
        })
    }).collect();
    
    let collection_schema = json!({
        "@context": "https://schema.org",
        "@type": "CollectionPage",
        "name": format!("{} - {}", category_name, SITE_NAME),
        "description": format!("Browse all {} posts in the {} category on {}", posts.len(), category_name, SITE_NAME),
        "url": format!("{}{}", SITE_URL, slugify(category_name)),
        "isPartOf": {
            "@type": "Blog",
            "name": SITE_NAME,
            "url": SITE_URL
        },
        "publisher": {
            "@type": "Organization",
            "name": SITE_NAME,
            "logo": {
                "@type": "ImageObject",
                "url": format!("{}images/logo.png", SITE_URL)
            }
        },
        "numberOfItems": posts.len(),
        "mainEntity": {
            "@type": "ItemList",
            "itemListElement": post_items
        },
        "inLanguage": "en-US"
    });
    
    let breadcrumb_schema = json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            {
                "@type": "ListItem",
                "position": 1,
                "name": "Home",
                "item": SITE_URL
            },
            {
                "@type": "ListItem",
                "position": 2,
                "name": "Blog",
                "item": format!("{}blog", SITE_URL)
            },
            {
                "@type": "ListItem",
                "position": 3,
                "name": category_name
            }
        ]
    });
    
    let graph = json!({
        "@context": "https://schema.org",
        "@graph": [collection_schema, breadcrumb_schema]
    });
    
    let schema_script = format!(
        r##"<script type="application/ld+json">{}</script>"##,
        graph.to_string()
    );
    
    page.declare_placement(PlacementPosition::Schema, &schema_script);
}

// Helper function to extract FAQ items from content
fn extract_faq_from_content(content: &str) -> Vec<serde_json::Value> {
    let mut faq_items = Vec::new();
    
    // Look for headers that end with "?" as potential FAQ questions
    // This is a simple implementation - you could make it more sophisticated
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Check if it's a header with a question
        if (line.starts_with("## ") || line.starts_with("### ")) && line.ends_with("?") {
            let question = line.trim_start_matches('#').trim();
            
            // Collect answer until next header or end
            let mut answer = String::new();
            i += 1;
            
            while i < lines.len() {
                let next_line = lines[i];
                if next_line.starts_with("#") {
                    break;
                }
                if !next_line.trim().is_empty() {
                    if !answer.is_empty() {
                        answer.push(' ');
                    }
                    answer.push_str(next_line.trim());
                }
                i += 1;
            }
            
            if !answer.is_empty() {
                faq_items.push(json!({
                    "@type": "Question",
                    "name": question,
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": answer
                    }
                }));
            }
        } else {
            i += 1;
        }
    }
    
    faq_items
}