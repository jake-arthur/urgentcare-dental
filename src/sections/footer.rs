use crate::prelude::*;

pub fn construct_footer<T, I>(site: &mut Site<T, I>) -> String {
    
    site.declare_css("footer", r##"
        
    
    footer {
    
    
        padding: 120px var(--site-padding-x);
        width: 100%;
        background-color: var(--turquoise-15);
        
        position: relative;
        
    
        .inner {
        
        
            .site-title {
                font-size: 24tem;
                transition: transform 0.3s ease;
                
                z-index: 10;
                
                &:hover {
                    transform: scale(0.97);
                }
                    
                .logo {
                    height: 35px;
                    margin-right: 10px;
                }
                    
                .urgent, .care, .dental {
                
                    color: #ccffff;
                    
                }
            }
                
            a {
                color: white;
                text-decoration: none;
                z-index: 10;
            }
                
            
            .polka-dots {
                width: 100%;
                height: 100%;
                background-color: transparent;
                background-image: var(--polka-dots-70);
                background-position: 0px 0px, 20px 20px;
                background-size: 40px 40px;
                position: absolute;
                bottom: 0;
                right: 0;
                z-index: 0;
            }
            
            .background-fade {
                position: absolute;
                z-index: 1;
                width: 100%;
                height: 100%;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: linear-gradient(165deg, rgb(1, 73, 75) 54%, rgba(1, 72, 74, 0) 100%);

            }
                
            .nav-area {
            
                display: flex;
                max-width: 1200px;
                justify-content: space-between;
                position: relative;
                padding-bottom: 48px;
                margin-bottom: 48px;
                border-bottom: 1px solid #68f8fd;
                z-index: 10;
                nav {
                z-index: 10;
                
                
                    p {
                        color: #68f8fd;
                        text-transform: uppercase;
                        font-weight: 300;
                        z-index: 10;
                    
                    }
                        
                    ul  {
                    
                        z-index: 10;
                    
                        li {
                            z-index: 10;
                            list-style: none;
                            margin-bottom: 1rem;
                        
                        }
                            
                    }
                        
                    a:hover {
                        
                        text-decoration: underline;
                    
                    }
                
                
                
                }
                

               
            
            }
                
            .copyright {
                position: relative;
                z-index: 10;
                color: white;
                font-size: 14px;
            
            }
                
        
        }   

    
    }
    
        
        
        
    "##);
    
    format!(r##"
        <footer>
            <div class="inner">
                <div class="polka-dots"></div>
                <div class="background-fade"></div>
                <div class="nav-area">
                    <a class="site-title" href="/"><img class="logo" src="/images/branding/UrgentCare-Dental-Logo-White.svg">{SITE_NAME_BRANDED}</a>
                    <nav class="menu" aria-label="Footer menu navigation">
                        <p>Menu</p>
                        <ul>
                            <li><a href="/about-us/">About Us</a></li>
                            <li><a href="/services/">Services</a></li>
                            <li><a href="/pricing/">Pricing</a></li>
                            <li><a href="/blog/">Blog</a></li>
                            
                            <li><a href="/contact">Contact</a></li>
                        </ul>
                    </nav>
                    
                    <nav class="policies" aria-label="Policies navigation">
                        <p>Policies</p>
                        <ul>
                            <li><a href="/privacy-policy">Privacy Policy</a></li>
                            <li><a href="/terms-and-conditions">Terms and Conditions</a></li>
                            <li><a href="/cookie-policy">Cookie Policy</a></li>
                            <li><a href="/complaints">Complaints Procedure</a></li>
                        </ul>
                    </nav>
                    
                    <nav class="contact" aria-label="Contact navigation">
                        <p>Contact</p>
                        <ul>
                            <li><a href="{EMAIL_LINK}">{EMAIL}</a></li>
                            <li><a href="{PHONE_NUMBER_LINK}">{PHONE_NUMBER}</a></li>
                        </ul>
                    </nav>
                    
                </div>
                <p class="copyright">© Copyright {year} · [SITE_NAME] · All rights reserved</p>
            </div>
        </footer>
    "##, year = get_current_year())
}