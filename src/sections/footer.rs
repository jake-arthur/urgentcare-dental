use crate::prelude::*;
use time::OffsetDateTime;

pub fn construct_footer<T, I>(site: &mut Site<T, I>) -> String {
    
    site.declare_css("footer", r##"
        
        footer {
            margin-top: 120px;
            border-top: 1px solid #eee;
            
            .footer-widgets {
                padding: 60px var(--site-padding-x);
                background: #fff;
                
                .wrap {
                    max-width: 1140px;
                    margin: 0 auto;
                    display: flex;
                    justify-content: center;
                    gap: 60px;
                    
                    .footer-widget-area {
                        flex: 1;
                        max-width: 360px;
                        
                        h3 {
                            font-size: 18px;
                            font-weight: 600;
                            margin-bottom: 20px;
                        }
                        
                        p {
                            font-size: 16px;
                            line-height: 1.5;
                            margin-bottom: 20px;
                            
                            &:last-child {
                                margin-bottom: 0;
                            }
                        }
                    }
                }
            }
            
            .site-footer {
                background: #fff;
                border-top: 1px solid #eee;
                padding: 30px var(--site-padding-x);
                text-align: center;
                
                .wrap {
                    max-width: 1140px;
                    margin: 0 auto;
                    
                    .nav-footer {
                        margin-bottom: 20px;
                        
                        ul {
                            display: flex;
                            justify-content: center;
                            gap: 30px;
                            list-style: none;
                            margin: 0;
                            padding: 0;
                            
                            li {
                                list-style: none;
                            }
                            
                            a {
                                color: var(--link-color);
                                text-decoration: none;
                                font-size: 15px;
                                transition: color 0.2s ease;
                                
                                &:hover {
                                    color: var(--link-hover-color);
                                    text-decoration: underline;
                                }
                            }
                        }
                    }
                    
                    p {
                        font-size: 15px;
                        color: #666;
                        margin: 0;
                    }
                }
            }
        }
        
        @media screen and (max-width: 768px) {
            footer {
                .footer-widgets .wrap {
                    flex-direction: column;
                    gap: 40px;
                    
                    .footer-widget-area {
                        max-width: 100%;
                        text-align: center;
                    }
                }
                
                .site-footer .wrap .nav-footer ul {
                    flex-wrap: wrap;
                    gap: 15px;
                }
            }
        }
        
    "##);
    
    format!(r##"
        <footer>
            <div class="site-footer">
                <div class="wrap">
                    <nav class="nav-footer" aria-label="Footer navigation">
                        <ul>
                            <li><a href="/privacy-policy">Privacy Policy</a></li>
                            <li><a href="/terms-of-service">Terms of Service</a></li>
                            <li><a href="/contact">Contact</a></li>
                        </ul>
                    </nav>
                    <p>Copyright © {year} · [SITE_NAME]</p>
                </div>
            </div>
        </footer>
    "##, year = get_current_year())
}