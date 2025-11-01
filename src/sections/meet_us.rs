use crate::prelude::*;

pub fn construct_meet_us<T, I>(site: &mut Site<T, I>) -> String {
    
    site.declare_css("meet-us", r##"
    
        {}
    
        section.meet-us {
                
            background-color: #f5ffff;
            text-align: center;
            
            .inner {
                
                position: relative;
                max-width: 1200px;
                margin: 0 auto;
                padding: 120px var(--site-padding-x);
                
                
                .subtitle {
                    font-size: 16px;
                    color: var(--turquoise-30);
                    font-weight: 600;
                    margin-bottom: 24px;
                }
                
                h2 {
                    font-size: 36px;
                    color: var(--turquoise-15);
                    margin-bottom: 24px;
                }
                
                p {
                    font-size: 18px;
                    line-height: 1.6;
                    color: var(--grey-50);
                    margin: 0 auto 20px;
                    font-weight: 300;                      
                }
                
                .text-area {
                    margin: 0 auto 48px;
                    max-width: 720px;
                }
                
    
                        
                .grid {
                    grid-template-columns: repeat(3, minmax(200px, 1fr));
                    grid-auto-rows: minmax(0, 1fr);
                    justify-content: center;
                    gap: 24px;
                    
                    padding: 0;
                    display: grid;
                    
                    
                    
                    .team-member {
                        position: relative;
                        width: 240px;
                        
                        margin: 0 auto;
                        
                        
                        .image-area {
                            
                            position: relative;
                            
                            height: 320px;
                            width: 240px;
                            
                            margin-bottom: 18px;
                            
                            
                            .circle {
                            
                                background-color: rgb(230, 230, 230);
                                border-radius: 480px;
                                height: 240px;
                                position: absolute;
                                bottom: 0;
                                left: 0;
                                right: 0;
                                overflow: hidden;
                                width: 240px;
                            
                            }
                        
                            img {
                                display: block;
                                position: absolute;
                                width: 100%;
                                height: 320px;
                                border-radius: 480px;
                                object-position: center center;
                                object-fit: cover;
                                aspect-ratio: .75;
                                bottom: 0;
                                transform: none;
                                transform-origin: 50% 50% 0px;
                                transition: all 0.35s ease 0s;
                                
                                &:hover {
                                    transform: translateY(-5px) scale(1.02);
                                    
                                    
                                }
                            }
                            
                            
                        }
                        
                        
                        h3 {
                            font-size: 18px;
                            color: var(--turquoise-15);
                            margin-bottom: 4px;
                        }
                        
                        p {
                            font-size: 14px;
                            font-weight: 400;
                            margin-bottom: 0;
                        }
                        
                        
                    }

                    
                } 
                                    
            }

        }
    "##);
    
    r##"<section class="meet-us">
            <div class="inner">
                <div class="text-area">
                    <div class="subtitle">Meet Us</div>
                    <h2>Our Passionate Team</h2>
                    <p>Our experienced team delivers fast, professional care around the clock. From emergency relief to complex treatments, we're here when other clinics are closed - ready to end your pain and fix the problem properly.</p>
                </div>
                <div class="grid">
                    <div class="team-member">
                        <div class="image-area">
                            <div class="circle"></div>
                            <img loading="lazy" src="/images/dentists/Dr. Zain Chishty.png" />
                        </div>
                        <h3>Dr. Zain Chishty</h3>
                        <p>General & Implant Dentist</p>
                    </div>
                    <div class="team-member">
                        <div class="image-area">
                            <div class="circle"></div>
                            <img loading="lazy" src="/images/dentists/Dr. Parasto Farshchi.png" />
                        </div>
                        <h3>Dr. Parasto Farshchi</h3>
                        <p>Cosmetic Dentist and Facial Aesthetician</p>
                    </div>
                    <div class="team-member">
                        <div class="image-area">
                            <div class="circle"></div>
                            <img loading="lazy" src="/images/dentists/Dr. Anoop Kizaekka.png" />
                        </div>
                        <h3>Dr. Anoop Kizaekka</h3>
                        <p>Oral Surgeon</p>
                    </div>
                    <div class="team-member">
                        <div class="image-area">
                            <div class="circle"></div>
                            <img loading="lazy" src="/images/dentists/Dr. Mohammed Fahhr Hussain.png" />
                        </div>
                        <h3>Dr. Mohammed Fahhr Hussain</h3>
                        <p>Oral Surgery & Endodontics</p>
                    </div>
                    <div class="team-member">
                        <div class="image-area">
                            <div class="circle"></div>
                            <img loading="lazy" src="/images/dentists/Dr. Shahee Ahmed.png" />
                        </div>
                        <h3>Dr. Shahee Ahmed</h3>
                        <p>Oral Surgery and Paediatric Dentistry</p>
                    </div>
                    <div class="team-member">
                        <div class="image-area">
                            <div class="circle"></div>
                            <img loading="lazy" src="/images/dentists/Dr. Rafaela Fernandes Zancan.png" />
                        </div>
                        <h3>Dr. Rafaela Fernandes Zancan</h3>
                        <p>Special Interest in Endodontics</p>
                    </div>
                </div>
            </div>
        </section>"##.to_owned()
}