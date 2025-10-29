use crate::prelude::*;

pub fn add_core_css<T, I>(site: &mut Site<T, I>) {

    site.declare_css("typography", r##"
        {}
        
        /* Typography */

        body {
            font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, sans-serif;
            font-size: 18tem;
            font-weight: 400;
            line-height: 1.625;
            color: #333;
            background-color: #fff;
            margin: 0;
            padding: 0;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
        }

        a {
            color: var(--link-color);
            text-decoration: underline;
            transition: color 0.2s ease-in-out;
            cursor: pointer;
            
            &:hover {
                color: var(--secondary);
                text-decoration: none;
            }
        }

        h1, h2, h3, h4, h5, h6 {
            font-weight: 600;
            line-height: 1.2;
            color: var(--primary);
            margin: 0 0 20tem;
            
            a {
                color: var(--secondary);
                
                &:hover {
                    filter: brightness(0.7);
                }
            }
        }

        h1 {
            font-size: 30tem;
        }

        h2 {
            font-size: 27tem;
        }

        h3 {
            font-size: 24tem;
        }

        h4 {
            font-size: 20tem;
        }

        p {
            margin: 0 0 28tem;
        }

        img {
            max-width: 100%;
            height: auto;
            vertical-align: top;
        }
        
        
        .site-name-branded {
            .urgent {
                color: var(--turquoise-30);
                font-weight: 700;
            }
            .care {
                color: var(--turquoise-15);
                font-weight: 300;
            }
            .dental {
                color: var(--grey-25);
                font-weight: 400;
            }
        }
    
    "##);
    
    
    site.declare_css("foundation", r##"
    
    {}
        
        /* Foundation */
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        *:where(:not(html, iframe, canvas, img, svg, video, audio):not(svg *, symbol *)) {
            all: unset;
            display: revert;
        }
        
        html, body {
            overflow-x: hidden;
        }
                  
        
    "##);
    
    site.declare_css("layout", r##"
    
    {}
    
        :root {
            
            --site-padding-x: 48px;
            
            @media screen and (max-width: [mobile]) {
            
                --site-padding-x: 24px;
                
            }
        }
        
        
        main {
            margin: 0 auto 0;
        }
    
    
        
    
    
    "##);
    
    
    
    
    site.declare_css("colors", r##"
    
    {}
    
    :root {
        
        
    
        --turquoise-15: #01494B;
        --turquoise-30: #029297;
        --turquoise-70: #68F8FD;
        --turquoise-90: #CCFFFF;
        --turquoise-98: #F5FFFF;
        
        --grey-25: #404040;
        --grey-50: #808080;
        
        --primary: var(--turqoise-15);
        --secondary: var(--turquoise-30);
        
        --link-color: var(--primary);
        --link-hover-color: var(--secondary);
        
        --polka-dots-70:radial-gradient(rgb(2, 220, 227) 1px, transparent 1px), radial-gradient(rgb(2, 220, 227) 1px, rgba(35, 84, 84, 0) 1px);
    }
        
    
    
    "##);
    
    
    
    
    
    
    
    
    
}