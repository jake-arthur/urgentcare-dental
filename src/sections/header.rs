use crate::prelude::*;

pub fn construct_header<T, I>(site: &mut Site<T, I>, page: &PageFoundation) -> String {
    // Define the navigation items
    let nav_items = vec![
        NavItem {
            name: "About Us".to_owned(), 
            path: "/about-us/".to_owned(),
        },
        NavItem {
            name: "Implants".to_owned(), 
            path: "/implants/".to_owned(),
        },
        NavItem {
            name: "Plans".to_owned(),
            path: "/plans/".to_owned(),
        },
        NavItem {
            name: "Pricing".to_owned(),
            path: "/pricing/".to_owned(),
        },
        NavItem {
            name: "Contact".to_owned(),
            path: "/contact/".to_owned(),
        },
    ];
    
    let home_nav_item = NavWrappedItem {
        name: SITE_NAME.to_owned(),
        path: "/".to_owned(),
        content: format!(r##"<img class="logo" src="/images/branding/UrgentCare-Dental-Logo.svg">{SITE_NAME_BRANDED}"##).to_owned(),
        class: Some("site-title".to_owned()),
    };
    
    let home_link = site.construct_nav_wrapped_link(&home_nav_item, &page);
    
    // Generate the navigation links using Citadel's nav system
    let nav_links = site.construct_nav_links(&nav_items, page);
    
    let (nav_checkbox, nav_button) = construct_nav_toggle(site);
    
    
    let directive_item = NavWrappedItem { 
        name: "Jobs".to_owned(), 
        path: "/jobs".to_owned(),
        content: r##"Book Appointment"##.to_owned(),
        class: Some("directive".to_owned()),
    };
    
    let directive = site.construct_nav_wrapped_link(&directive_item, &page);
    
    let content = format!(
        r####"
        <header>
        <nav aria-label="Main navigation">
            {nav_checkbox}
            {home_link}                
            <ul>
                {nav_links}
            </ul>
            {nav_button}
            {directive}
        </nav>
        </header>
        
        "####
    );
    
    
    
    
    site.declare_css("header", r##"
    
        /* Header */
        {}
        header {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            z-index: 1000;
            padding: 18px var(--site-padding-x);
            margin: 0 auto;
            max-width: 100%;
            background-color: white;
            
            nav {
                --transform: scale(0.97);
                --nav-link-padding-x: 14px;
                display: flex;
                justify-content: space-between;
                align-items: center;
                max-width: 1200px;
                gap: 40px;
                margin: 0 auto;

                ul {
                    display: flex;
                    list-style: none;
                    gap: 12px;
                    margin: 0;
                    padding: 0;
                    
                    li {
                        list-style: none;
                        transition: transform 0.3s ease;
                        width: fit-content;
                        
                        &:hover {
                            transform: var(--transform);
                        }
                    
                        a {
                            padding: var(--nav-link-padding-x) 14px;
                        }
                    }
                }
                
                a {
                    text-decoration: none;
                    color: #333;
                    font-size: 16tem;
                    cursor: pointer;
                }
                
                .site-title {
                    font-size: 24tem;
                    color: #333;
                    transition: transform 0.3s ease;
                    
                    &:hover {
                        transform: scale(0.97);
                    }
                        
                    .logo {
                        height: 35px;
                        margin-right: 10px;
                    }
                }
                
                input {
                    display: none;
                }
            }
        }

        @media screen and (max-width: [mobile]) {

            header {

                &:has(#nav-toggle-checkbox:checked){
                    position: relative;
                    padding-bottom: 30px;
                }
                
                nav {
                    
                    --transform: translateX(6px) scale(0.98);
                    
                    ul {
                        display: none;
                        
                        li {
                            a {
                                margin-left: calc(-1 * var(--nav-link-padding-x));
                            }
                        }
                    }
                    
                    
                    &:has(#nav-toggle-checkbox:checked){
                        display: grid;
                        grid-template-columns: 1fr;
                        grid-template-rows: auto 1fr;
                        row-gap: 30px;
                    }
                    
                    #nav-toggle-checkbox:checked {
                        
                        & ~ ul {
                            display: flex;
                            flex-direction: column;
                            grid-area: 2 / 1 / 2 / 3;
                            padding-top: 10px;
                        }
                        
                        & ~ .directive {
                            display: block;
                            grid-area: 3 / 1 / 3 / 3;
                            position: relative;
                        }
                        
                        
                        & ~ #nav-toggle-button {
                            grid-area: 1 / 2;
                        }
                    }
                }

            }
        }
        
        
        
        
       
    
    
    
    
    
    
    "##);
    
    
    content
}