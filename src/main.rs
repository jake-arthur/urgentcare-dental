use urgentcare_dental::prelude::*;
use std::collections::HashMap;

use UCDPages::*;

fn main() {
    
    let mut pages = vec![
        Page {
            foundation: PageFoundation { 
                title: SITE_NAME.to_owned(),
                slug: Some("/".to_owned()),
                metadescription: Some(SITE_DESCRIPTION.to_owned()),
                ..default() 
            },
            specification: Homepage,
        },

        Page {
            foundation: PageFoundation { 
                title: "Blog".to_owned(),
                ..default() 
            },
            specification: PostList,
        },
        
        Page {
            foundation: PageFoundation { 
                title: "Categories".to_owned(),
                slug: Some("categories".to_owned()),
                metadescription: Some("Browse all content categories".to_owned()),
                ..default() 
            },
            specification: CategoriesIndex,
        },
        Page {
            foundation: PageFoundation { 
                title: "About Us".to_owned(),
                ..default() 
            },
            specification: AboutUs,
        },
    
    ];
    
    // Add blog posts
    /*for post in get_all_posts_and_post_pages() {
        pages.push(Page {
            foundation: PageFoundation {
                title: post.frontmatter.title.clone(),
                slug: Some(post.slug.clone()),
                metadescription: Some(post.frontmatter.description.clone()),
                ..default()
            },
            specification: UCDPages::BlogPost(post),
        });
    }*/
    
    // Get all posts
    let all_posts = get_all_posts_and_post_pages();
    
    // Build a map of categories to their posts
    let mut categories: HashMap<String, Vec<PostData<UCDFrontmatter>>> = HashMap::new();
    
    // Add blog posts and collect categories
    for post in &all_posts {
        // Add the blog post page
        pages.push(Page {
            foundation: PageFoundation {
                title: post.frontmatter.title.clone(),
                slug: Some(post.slug.clone()),
                metadescription: Some(post.frontmatter.description.clone()),
                ..default()
            },
            specification: UCDPages::BlogPost(post.clone()),
        });
        
        // Collect posts by category
        if !post.frontmatter.exclude {  // Only include non-excluded posts in categories
            if post.frontmatter.category.is_empty() {
                // Add to Uncategorized if no categories
                categories
                    .entry("Uncategorized".to_owned())
                    .or_insert_with(Vec::new)
                    .push(post.clone());
            } else {
                // Add to each specified category
                for category in &post.frontmatter.category {
                    categories
                        .entry(category.clone())
                        .or_insert_with(Vec::new)
                        .push(post.clone());
                }
            }
        }
    }
    
    // Create category pages
    for (category_name, mut category_posts) in categories {
        // Sort posts by date (newest first) within each category
        category_posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
        
        let category_slug = slugify(&category_name);
        
        pages.push(Page {
            foundation: PageFoundation {
                title: category_name.to_owned(),
                slug: Some(category_slug),
                metadescription: Some(format!("Browse all {} posts in the {} category", 
                    category_posts.len(), category_name)),
                ..default()
            },
            specification: UCDPages::Category(category_name, category_posts),
        });
    }
    
    let mut site = Site::<UCDPages, ()> {
        title: SITE_NAME.to_owned(),
        base_url: Url::parse(SITE_URL).unwrap(),
        ..default()  // No imperium needed for this simple site
    }
    .add_constructor(Homepage, construct_homepage)
    .add_constructor(BlogPost(default()), construct_post)
    .add_constructor(PostList, construct_blog)
    .add_constructor(Category(String::new(), Vec::new()), construct_category)
    .add_constructor(CategoriesIndex, construct_categories)
    .add_constructor(AboutUs, construct_about_us)
    // Your constructors will go here
    .add_head_constructor()
    .add_pages(pages);
    
    
    add_core_css(&mut site);
    
    add_decrees(&mut site);
    
    add_head_code(&mut site);
    
    add_sitewide_schema(&mut site);
    
    
    site.commence();
}