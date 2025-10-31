use crate::prelude::*;

pub fn construct_homepage(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    
    let head = site.construct_head(page);
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-US">
        {head}
        <body>
            {header}
            <main class="home">
                <section class="dentist-splash-hero">
                    <div class="polka-dots"></div>
                    <div class="background-fade"></div>
                    <div class="text-area">
                        <h1>Pain Doesn't Wait.<br>Neither Do We.</h1>
                        <p>Emergency dental care when you need it most. No waiting. Just £20 to end your pain right now.</p>
                        <p>Our skilled team is ready 24/7 for any dental emergency. Whether it's a broken tooth, severe pain, or lost filling - we're here after hours when other clinics are closed. Fast, professional relief without the premium price tag.</p>
                        <p>We offer follow-up reviews after treatments free of charge too, unlike most other dental practices.</p>
                        <div class="buttons">
                            <a href="{BOOKING_LINK}" class="primary">Book an Appointment</a>
                            <a href="{PHONE_NUMBER_LINK}" class="secondary">Call {PHONE_NUMBER}</a>
                            <a href="https://wa.me/447383502032" class="secondary">WhatsApp Us</a>
                        </div>
                    </div>
                    <div class="person-area">
                        <div class="shadow"></div>
                        <img src="/images/dentists/Dentist.png" alt="Dentist at UrgentCare Dental" />
                    </div>
                </section>
                
                <section class="about-our-clinics">
                    <div class="inner">
                        <div class="image-area">
                            <div class="polka-dots"></div>
                            <img src="/images/homepage/RQIgz0STrqYXyK5hXdJNlI3rU.avif" alt="UrgentCare Dental Building" />
                            <a class="view-locations-button" href="/#locations-background"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" focusable="false" color="var(--token-bbd03ddf-1640-40ca-ba74-d0deb83f7d14, rgb(104, 248, 253))" style="user-select: none; width: 100%; height: 100%; display: inline-block; fill: var(--token-bbd03ddf-1640-40ca-ba74-d0deb83f7d14, rgb(104, 248, 253)); color: var(--token-bbd03ddf-1640-40ca-ba74-d0deb83f7d14, rgb(104, 248, 253)); flex-shrink: 0;"><g color="var(--token-bbd03ddf-1640-40ca-ba74-d0deb83f7d14, rgb(104, 248, 253))" weight="light"><path d="M128,66a38,38,0,1,0,38,38A38,38,0,0,0,128,66Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,128,130Zm0-112a86.1,86.1,0,0,0-86,86c0,30.91,14.34,63.74,41.47,94.94a252.32,252.32,0,0,0,41.09,38,6,6,0,0,0,6.88,0,252.32,252.32,0,0,0,41.09-38c27.13-31.2,41.47-64,41.47-94.94A86.1,86.1,0,0,0,128,18Zm0,206.51C113,212.93,54,163.62,54,104a74,74,0,0,1,148,0C202,163.62,143,212.93,128,224.51Z"></path></g></svg>
                            <p>Click Here to View Locations</p>
                            </a>
                            </div>
                        <div class="text-area">
                            <div class="subtitle">About</div>
                            <h2>Our Dental Clinics</h2>
                            <p>At UrgentCare Dental, we provide 24/7 emergency dental care in Leeds and Manchester. Our experienced team delivers professional treatment and pain relief during evenings, weekends, and holidays - when dental emergencies always seem to happen.</p>
                            <p>Our phone number is {PHONE_NUMBER}.</p>
                            <div class="achievement-boxes">
                                <div class="box">
                                    <div class="icon">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" focusable="false" color="var(--token-64549ac4-d685-4363-a417-5961f1d256e9, rgb(2, 146, 151))" style="user-select: none; width: 100%; height: 100%; display: inline-block; fill: var(--token-64549ac4-d685-4363-a417-5961f1d256e9, rgb(2, 146, 151)); color: var(--token-64549ac4-d685-4363-a417-5961f1d256e9, rgb(2, 146, 151)); flex-shrink: 0;"><g color="var(--token-64549ac4-d685-4363-a417-5961f1d256e9, rgb(2, 146, 151))" weight="regular"><path d="M232,64H208V48a8,8,0,0,0-8-8H56a8,8,0,0,0-8,8V64H24A16,16,0,0,0,8,80V96a40,40,0,0,0,40,40h3.65A80.13,80.13,0,0,0,120,191.61V216H96a8,8,0,0,0,0,16h64a8,8,0,0,0,0-16H136V191.58c31.94-3.23,58.44-25.64,68.08-55.58H208a40,40,0,0,0,40-40V80A16,16,0,0,0,232,64ZM48,120A24,24,0,0,1,24,96V80H48v32q0,4,.39,8Zm144-8.9c0,35.52-29,64.64-64,64.9a64,64,0,0,1-64-64V56H192ZM232,96a24,24,0,0,1-24,24h-.5a81.81,81.81,0,0,0,.5-8.9V80h24Z"></path></g></svg>
                                    </div>
                                    <h3>Years of Excellence</h3>
                                    <p>Established emergency dental service with clinics across Northern England.</p>
                                </div>
                                <div class="box">
                                    <div class="icon">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" focusable="false" color="var(--token-64549ac4-d685-4363-a417-5961f1d256e9, rgb(2, 146, 151))" style="user-select: none; width: 100%; height: 100%; display: inline-block; fill: var(--token-64549ac4-d685-4363-a417-5961f1d256e9, rgb(2, 146, 151)); color: var(--token-64549ac4-d685-4363-a417-5961f1d256e9, rgb(2, 146, 151)); flex-shrink: 0;"><g color="var(--token-64549ac4-d685-4363-a417-5961f1d256e9, rgb(2, 146, 151))" weight="regular"><path d="M239.35,70.08a13.41,13.41,0,0,0-11.77-9.28l-36.94-2.92L176.43,24.22a13.51,13.51,0,0,0-24.86,0L137.36,57.88,100.42,60.8a13.39,13.39,0,0,0-7.66,23.58l28.06,23.68-8.56,35.39a13.32,13.32,0,0,0,5.1,13.91,13.51,13.51,0,0,0,15,.69L164,139l31.65,19.06a13.54,13.54,0,0,0,15-.69,13.34,13.34,0,0,0,5.09-13.91l-8.56-35.39,28.06-23.68A13.32,13.32,0,0,0,239.35,70.08ZM193.08,99a8,8,0,0,0-2.61,8l8.28,34.21L168.13,122.8a8,8,0,0,0-8.25,0l-30.62,18.43L137.54,107a8,8,0,0,0-2.62-8L108,76.26l35.52-2.81a8,8,0,0,0,6.74-4.87L164,35.91l13.79,32.67a8,8,0,0,0,6.74,4.87l35.53,2.81Zm-105,24.18L29.66,181.66a8,8,0,0,1-11.32-11.32l58.45-58.45a8,8,0,0,1,11.32,11.32Zm10.81,49.87a8,8,0,0,1,0,11.31L45.66,237.66a8,8,0,0,1-11.32-11.32l53.27-53.26A8,8,0,0,1,98.92,173.08Zm73-1a8,8,0,0,1,0,11.32l-54.28,54.28a8,8,0,0,1-11.32-11.32l54.29-54.28A8,8,0,0,1,171.94,172.06Z"></path></g></svg>
                                    </div>
                                    <h3>100% of Patients Satisfied</h3>
                                    <p>Thousands of emergency patients treated, with quick response times and professional care.</p>
                                </div>
                            </div>
                            <div class="buttons">
                                <a href="https://urgentcaredental.setmore.com" class="primary">Book Online</a>
                                <a href="{PHONE_NUMBER_LINK}" class="small-button  secondary">Call Us</a>
                            </div>
                        </div>
                    </div>
                </section>
                
                <section class="locations">
                    <div class="inner">
                        <div class="polka-dots"></div>
                        <div class="text-area">
                            <div class="subtitle">Locations</div>
                            <h2>Locations</h2>
                            <p>With our locations in Leeds and Manchester, we provide emergency dental care where and when you need it. Professional treatment, accessible pricing, and fast relief - no matter which clinic you visit.</p>
                        </div>
                        <div class="grid">
                            <div class="location">
                                <iframe src="https://maps.google.com/maps?q=53.520984213115455,-2.166727319008508&amp;z=10&amp;output=embed" style="height: 320px; width: 100%; border: 0px;"></iframe>
                                <h3>Manchester</h3>
                                <p>8 Broadway<br>Manchester<br>M40 3LN</p>
                                <a href="{PHONE_NUMBER_LINK}">Call Us at {PHONE_NUMBER}</a>
                                <a href="{BOOKING_LINK}">Book Online</a>
                            </div>
                        </div>
                        
                    </div>
                </section>
                
                <section class="services">
                    <div class="inner">
                    
                        <div class="text-area">
                        
                            <div class="subtitle">Services</div>
                            <h2>Emergency & Urgent Care</h2>
                            <p>24/7 emergency dental care when everything else is closed. From severe pain to lost fillings, our team provides fast, professional treatment at times other dentists can't - or won't.</p>
                            
                        </div>
                        
                        <div class="details-area">
                            <div class="grid-area">
                                <div class="grid">
                                    <div class="service">
                                        <h3>Immediate Pain Relief</h3>
                                        <p>£20 expert assessment to identify the cause of your pain. Quick response within 1 hour, with same-day treatment usually available. Follow ups are free of charge.</p>
                                    </div>
                                    <div class="service">
                                        <h3>Emergency Extractions</h3>
                                        <p>Professional extraction service for severe pain, damaged teeth, or complex cases. Sedation available if needed.</p>
                                    </div>
                                    <div class="service">
                                        <h3>Lost Fillings & Crowns</h3>
                                        <p>Fast replacement of lost fillings and crowns to protect your tooth and stop pain or sensitivity. Prevent further damage.</p>
                                    </div>
                                    <div class="service">
                                        <h3>Dental Trauma & Injury</h3>
                                        <p>Immediate care for knocked-out teeth, breaks and injuries. Quick response is crucial - we're here 24/7 to save your smile.</p>
                                    </div>
                                    <div class="service">
                                        <h3>Abscess Treatment</h3>
                                        <p>Rapid relief from severe infections and abscesses. Professional treatment to stop pain and prevent complications.</p>
                                    </div>
                                    <div class="service">
                                        <h3>Specialist Emergency Care</h3>
                                        <p>Complex cases handled by our experienced team. From root canals to oral surgery, we provide complete emergency care.</p>
                                    </div>
                                </div>
                                
                                <a class="small-button secondary" href="/pricing/" >View Pricing</a>
                            
                            </div>
                        
                            <div class="image-area">
                                <img src="/images/homepage/m0Zzmss4wWYLkh6gcTTNvvdHXro.webp" alt="Dentist working" />
                                <div class="polka-dots"></div>
                            </div>
                        </div>
                    
                    </div>
                
                </section>
                
                <section class="general-services">

                    <div class="inner">
                    
                        <div class="text-area">
                            <div class="subtitle">Services</div>
                            <h2>General & Cosmetic Dentistry</h2>
                            <p>When the emergency is over, you'll want a dentist you can trust for the long term. We provide complete dental care - from routine checkups to advanced treatments - with the same professional service and fair pricing. After treatment, a follow-up appointment is free.</p>
                        </div>
                        <div class="grid">
                            <div class="service">
                                <div class="icon">
                                    <div class="circle">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" focusable="false" ><g weight="thin"><path d="M234.83,69.17l-48-48a4,4,0,0,0-5.66,5.66L202.34,48,168,82.34,130.83,45.17a4,4,0,1,0-5.66,5.66L134.34,60,47.51,146.83A12,12,0,0,0,44,155.31v51L21.17,229.17a4,4,0,0,0,5.66,5.66L49.66,212h51a11.93,11.93,0,0,0,8.48-3.51L196,121.66l9.17,9.17a4,4,0,0,0,5.66-5.66L173.66,88,208,53.66l21.17,21.17a4,4,0,1,0,5.66-5.66ZM103.51,202.83a4,4,0,0,1-2.82,1.17H52V155.31a4,4,0,0,1,1.17-2.82L74,131.66l23.17,23.17a4,4,0,1,0,5.66-5.66L79.66,126,98,107.66l23.17,23.17a4,4,0,0,0,5.66-5.66L103.66,102,140,65.66l25.17,25.17h0L190.34,116Z"></path></g></svg>
                                    </div>
                                </div>
                                <h3>Emergency Services</h3>
                                <ul>
                                    <li>24/7 Emergency Treatment</li>
                                    <li>Pain Assessment & Relief (£20)</li>
                                    <li>Lost Fillings/Crowns</li>
                                    <li>Tooth Trauma/Injury</li>
                                    <li>Severe Pain/Abscess</li>
                                    <li>Emergency Extractions</li>
                                </ul>
                            </div>
                            <div class="service">
                                <div class="icon">
                                    <div class="circle">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" focusable="false"><g weight="thin"><path d="M178.91,128l30.55-30.54a36,36,0,0,0-50.92-50.92L128,77.09,97.46,46.54A36,36,0,0,0,46.54,97.46L77.09,128,46.54,158.54a36,36,0,0,0,50.92,50.92L128,178.91l30.54,30.55a36,36,0,0,0,50.92-50.92ZM164.2,52.2a28,28,0,0,1,39.6,39.6l-30.55,30.54L133.66,82.75Zm3.4,75.8L128,167.6,88.4,128,128,88.4ZM52.2,91.8A28,28,0,0,1,91.8,52.2l30.54,30.55L82.75,122.34Zm39.6,112a28,28,0,0,1-39.6-39.6l30.55-30.54,39.59,39.59Zm112,0a28,28,0,0,1-39.6,0l-30.54-30.55,39.59-39.59L203.8,164.2a28,28,0,0,1,0,39.6ZM120,128a8,8,0,1,1,8,8A8,8,0,0,1,120,128Z"></path></g></svg>
                                    </div>
                                </div>
                                <h3>General Treatment</h3>
                                <ul>
                                    <li>Fillings</li>
                                    <li>Crowns</li>
                                    <li>Composite Bonding</li>
                                    <li>Extractions</li>
                                    <li>Root Canal Treatment</li>
                                    <li>Invisalign</li>
                                </ul>
                            </div>
                            <div class="service">
                                <div class="icon">
                                    <div class="circle">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" focusable="false"><g weight="thin"><path d="M169.49,67.71,138.77,80l30.72,12.28A4,4,0,0,1,168,100a3.91,3.91,0,0,1-1.49-.29L128,84.31,89.49,99.71A3.91,3.91,0,0,1,88,100a4,4,0,0,1-1.49-7.72L117.23,80,86.51,67.71a4,4,0,0,1,3-7.43L128,75.69l38.51-15.41a4,4,0,0,1,3,7.43Zm50.51,12c.07,71.08-23.16,130.07-45.65,146.05a11.74,11.74,0,0,1-11.93,1,11.91,11.91,0,0,1-6.9-10C154.35,200.93,149.33,164,128,164s-26.35,36.94-27.52,52.82a12.11,12.11,0,0,1-12,11.19,11.77,11.77,0,0,1-6.83-2.2c-22.49-16-45.72-75-45.65-146A52,52,0,0,1,88,28h80A52,52,0,0,1,220,79.75Zm-8,0A44,44,0,0,0,168,36H88A44,44,0,0,0,44,79.76c-.07,67.58,21.9,125,42.29,139.51a3.77,3.77,0,0,0,3.89.33,4,4,0,0,0,2.32-3.37C95.36,177.39,108,156,128,156s32.64,21.4,35.5,60.24a4,4,0,0,0,2.32,3.37,3.77,3.77,0,0,0,3.89-.33C190.1,204.8,212.07,147.34,212,79.76Z"></path></g></svg>
                                    </div>
                                </div>
                                <h3>Specialist Care</h3>
                                <ul>
                                    <li>Jaw Pain/TMJ</li>
                                    <li>Oral Surgery</li>
                                    <li>Complex Extractions</li>
                                    <li>Tooth Decay Treatment</li>
                                    <li>Sedation Available</li>
                                    <li>Nighttime Bookings</li>
                                </ul>
                            </div>
                        </div>
                        <a class="small-button secondary" href="/pricing/" >View Pricing</a>
                    </div>
                
                </section>
                
                <section class="meet-us">
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
                </section>
                
                <section class="open-times">
                    <div class="inner">
                        <div class="polka-dots"></div>
                        <div class="background-fade"></div>
                        
                        <div class="text-area">
                            <h2>Book an Appointment</h2>
                            <p>Our team is ready to help whenever you need us. Same-day appointments available, with fast response times (usually within an hour).</p>
                        </div>
                        <div class="times-area">
                            <div class="hours-area">
                                <h3>Our Working Program</h3>
                                <table>
                                    <tr>
                                        <td>Monday</td>
                                        <td>Open 24 Hours</td>
                                    </tr>
                                    <tr>
                                        <td>Tuesday</td>
                                        <td>Open 24 Hours</td>
                                    </tr>
                                    <tr>
                                        <td>Wednesday</td>
                                        <td>Open 24 Hours</td>
                                    </tr>
                                    <tr>
                                        <td>Thursday</td>
                                        <td>Open 24 Hours</td>
                                    </tr>
                                    <tr>
                                        <td>Friday</td>
                                        <td>Open 24 Hours</td>
                                    </tr>
                                    <tr>
                                        <td>Saturday</td>
                                        <td>Open 24 Hours</td>
                                    </tr>
                                    <tr>
                                        <td>Sunday</td>
                                        <td>Open 24 Hours</td>
                                    </tr>
                                </table>
                            </div>
                            <div class="image-area">
                                <div class="buttons">
                                    <a href="{BOOKING_LINK}" class="primary">Book Online</a>
                                    <a href="{PHONE_NUMBER_LINK}" class="secondary">Call {PHONE_NUMBER}</a>
                                    <a href="https://wa.me/447383502032" class="secondary">WhatsApp Us</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
                
                <<section class="patient-stories">
                <div class="text-area">
                    <div class="subtitle">Testimonials</div>
                    <h2>Kind Words from Our Patients</h2>
                    <p>Real feedback from emergency patients we've helped.</p>
                </div>
                <div class="testimonials">
                    <div class="testimonial">
                        <div class="image-circle">
                            <img src="/images/homepage/testimonials/SkjDmb1eRBfviouDKjS5VNX7as.webp" alt="Sarah Mitchell">
                        </div>
                        <div class="stars">★★★★★</div>
                        <p>"Sorted my broken crown same evening. Clean modern practice, caring staff. Called several dentists but only these could help after hours. Quick response on the phone and clear pricing upfront. The dentist explained everything thoroughly."</p>
                        <cite>— Sarah Mitchell</cite>
                    </div>
                    <div class="testimonial">
                        <div class="image-circle">
                            <img src="/images/homepage/testimonials/IAOjZkUp6Oc9Ct0TRrlYXUX6PLo.webp" alt="Emma Wilson">
                        </div>
                        <div class="stars">★★★★★</div>
                        <p>"Relief from abscess pain when my regular dentist was closed. Worth every penny for evening care. Was worried about needing emergency treatment but they made the whole process straightforward. Excellent follow-up care instructions too."</p>
                        <cite>— Emma Wilson</cite>
                    </div>
                    <div class="testimonial">
                        <div class="image-circle">
                            <img src="/images/homepage/testimonials/D2d3tLp1h0R0GSpF1T8OggW2ro.webp" alt="Jennifer O'Connor">
                        </div>
                        <div class="stars">★★★★★</div>
                        <p>"Emergency visit for broken crown. Reception was efficient and got me in within 30 minutes. Thorough treatment and helpful aftercare advice. Great to know they're available when needed."</p>
                        <cite>— Jennifer O'Connor</cite>
                    </div>
                    <div class="testimonial">
                        <div class="image-circle">
                            <img src="/images/homepage/testimonials/bkfRdAxlZmoDYYSDzbXucX5WA.webp" alt="David Thompson">
                        </div>
                        <div class="stars">★★★★★</div>
                        <p>"Sorted my broken crown same evening. Clean modern practice, caring staff. Called several dentists but only these could help after hours. Quick response on the phone and clear pricing upfront. The dentist explained everything thoroughly."</p>
                        <cite>— David Thompson</cite>
                    </div>
                    <div class="testimonial">
                        <div class="image-circle">
                            <img src="/images/homepage/testimonials/a7qhxcvMrGG7nO7isLG7wkBU.webp" alt="David Chen">
                        </div>
                        <div class="stars">★★★★★</div>
                        <p>"Sudden toothache after work - really relieved they could see me same evening. Quick appointment and professional service. Dentist explained everything clearly and fixed the problem. Clean, modern clinic."</p>
                        <cite>— David Chen</cite>
                    </div>
                    <div class="testimonial">
                        <div class="image-circle">
                            <img src="/images/homepage/testimonials/iloHmwiKeL15574tMg7gfcPfeac.webp" alt="Fatima Rahman">
                        </div>
                        <div class="stars">★★★★★</div>
                        <p>"Called with severe pain from abscess. Professional team and prompt service got me sorted quickly. Clean facilities and caring staff made a stressful situation much easier. Very grateful for their help."</p>
                        <cite>— Fatima Rahman</cite>
                    </div>
                </div>
            </section>
                
                
            </main>
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
    );
    
    css(site);
    
    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("homepage", r##"
        {}
        
        main.home {
            
            
            .small-button {
                display: block;
                width: fit-content;
                font-size: 16px;
                font-weight: 500;
                height: min-content;
                place-content: center flex-start;
                align-items: center;
                padding: 0 12px;
                height: auto;
                text-decoration: none;
                align-self: center;
                place-content: center;
                background-color: #fff;
                color: var(--turquoise-15);
                
                padding: 14px 32px;
                border-radius: 25px;
                text-decoration: none;
                font-weight: 500;
                transition: all 0.3s ease;
                
                &.secondary {
                    background: white;
                    color: var(--turquoise-30);
                    border: 2px solid var(--turquoise-30);
                    
                    &:hover {
                        background: var(--turquoise-98);
                        transform: translateY(-2px);
                    }
                }
                
            }
        
            section.dentist-splash-hero {
            
                display: flex;
                padding: 0 var(--site-padding-x);
                height: 100vh;
                width: 100%;
                background-color: #029297;
                align-items: center;
                justify-content: center;
                margin: 0 auto;
                
                .polka-dots {
                    width: 100%;
                    height: 100%;
                    background-color: transparent;
                    background-image: var(--polka-dots-70);
                    background-position: 0px 0px, 20px 20px;
                    background-size: 40px 40px;
                    position: absolute;
                    top: 0;
                    left: 0;
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
                    background: linear-gradient(150deg, rgba(2, 145, 150, 0) 0.1%, #029297, rgb(2, 146, 151) 140%);
                }
                
                
                .text-area {
                    flex: 1;
                    display: flex;
                    flex-direction: column;
                    align-items: flex-start;
                    justify-content: center;
                    max-width: 600px;
                    z-index: 10;
                    
                    h1 {
                        font-size: 48px;
                        color: var(--turquoise-70);
                        line-height: 1.4em;
                    }
                        
                    p {
                        font-size: 18px;
                        color: #fff;
                        line-height: 1.4em;
                        paragraph-spacing: 20px;
                        font-weight: 300;
                    }
                        
                    .buttons {
                        display: flex;
                        gap: 10px;
                        font-size: 22px;
                        font-weight: 500;
                        flex-flow: wrap;
                        height: min-content;
                        place-content: center flex-start;
                        align-items: center;
                        max-width: 576px;
                    
                        
                        .primary {
                        
                            background-color: var(--turquoise-15);
                            color: white;
                        }
                        
                        .secondary {
                        
                            background-color: #fff;
                            color: var(--turquoise-15);
                         
                        }
                        
                        a {
                            padding: 0 36px;
                            height: 60px;
                            text-decoration: none;
                            align-self: center;
                            place-content: center;
                        }
                    }
                }
                    
                .person-area {
                    
                    z-index: 8;
                    
                    img {
                        object-position: center center;
                        object-fit: contain;
                        width: 100%;
                        margin-bottom: -30vh;
                        z-index: 10;
                    }
                    
                    .shadow {
                        position: absolute;
                        background-color: #001819;
                        border-radius: 2000px;
                        height: 1200px;
                        width: 640px;
                        margin-top: 80px;
                        filter: blur(100px);
                        -webkit-filter: blur(100px);
                        opacity: .25;
                        z-index: -1;
                    }
                    
                }
                
                
                & + * {
                
                    z-index: 10;
                    
                }
                
            }
                
            
            section.about-our-clinics {
                    position: relative;
                    background: white;
                    z-index: 100;
                    
                
                    .inner {
                        display: flex;
                        align-items: center;
                        padding: 120px var(--site-padding-x);
                        margin: 0 auto;
                        position: relative;
                        gap: 80px;
                        max-width: 1200px;
                        
                        .image-area {
                            flex: 1;
                            position: relative;
                            
                            .polka-dots {
                                position: absolute;
                                width: 450px;
                                height: 450px;
                                top: -10%;
                                left: -10%;
                                background-image: radial-gradient(rgb(2, 220, 227) 1px, transparent 1px), radial-gradient(rgb(2, 220, 227) 1px, rgba(35, 84, 84, 0) 1px);
                                background-position: 0 0, 10px 10px;
                                background-size: 20px 20px;
                                opacity: 0.5;
                                z-index: 0;
                            }
                            
                            img {
                                width: 570px;
                                height: 570px;
                                position: relative;
                                z-index: 1;
                                box-shadow: 0 20px 60px rgba(0,0,0,0.1);
                                aspect-ratio: 1/1;
                                object-fit: cover;
                            }
                            
                            .view-locations-button {
                                position: absolute;
                                display: flex;
                                bottom: -40px;
                                right: 30px;
                                background: var(--turquoise-15);
                                color: white;
                                padding: 12px 24px;
                                text-decoration: none;
                                font-weight: 500;
                                z-index: 2;
                                transition: all 0.3s ease;
                                justify-items: center;
                                align-items: center;
                                place-content: center;
                                gap: 12px;
                                
                                p {
                                    margin: 0;
                                    width: 150px;
                                }
                                
                                svg {
                                    width: 100%;
                                    height: 100%;
                                    max-width: 48px;
                                    display: inline-block;
                                }
                                
                                &:hover {
                                    background: var(--turquoise-15);
                                    transform: translateY(-2px);
                                }
                            }
                        }
                    
                        .text-area {
                            flex: 1;
                            
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
                                margin-bottom: 20px;
                                font-weight: 300;
                            }
                            
                            .achievement-boxes {
                                display: grid;
                                grid-template-columns: 1fr 1fr;
                                gap: 24px;
                                margin: 40px 0;
                                
                                .box {
                                    padding: 24px 0;
                                    
                                    border-radius: 12px;
                                    
                                    .icon {
                                        background: var(--turquoise-98);
                                        padding: 12px;
                                        width: 24px;
                                        height: 24px;
                                        margin-bottom: 16px;
                                        
                                        svg {
                                            width: 100%;
                                            height: 100%;
                                            fill: var(--turquoise-30);
                                        }
                                    }
                                    
                                    h3 {
                                        font-size: 18px;
                                        color: var(--turquoise-15);
                                        margin-bottom: 8px;
                                    }
                                    
                                    p {
                                        font-size: 14px;
                                        line-height: 1.5;
                                        color: var(--grey-50);
                                    }
                                }
                            }
                            
                            .buttons {
                                display: flex;
                                gap: 16px;
                                
                                a {
                                    padding: 14px 32px;
                                    border-radius: 25px;
                                    text-decoration: none;
                                    font-weight: 500;
                                    transition: all 0.3s ease;
                                    
                                    &.primary {
                                        background: var(--turquoise-30);
                                        color: white;
                                        
                                        &:hover {
                                            background: var(--turquoise-15);
                                            transform: translateY(-2px);
                                        }
                                    }
                                    
                                    &.secondary {
                                        background: white;
                                        color: var(--turquoise-30);
                                        border: 2px solid var(--turquoise-30);
                                        
                                        &:hover {
                                            background: var(--turquoise-98);
                                            transform: translateY(-2px);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            
            section.locations {
                
                
                .inner {
                    position: relative;
                    max-width: 1200px;
                    
                    margin: 0 auto;
                    
                    padding: 120px var(--site-padding-x);
                    
                    text-align: center;
                
                    .polka-dots {
                        position: absolute;
                        width: 50%;
                        height: 480px;
                        top: 480px;
                        left: 0;
                        background-image: radial-gradient(rgb(2, 220, 227) 1px, transparent 1px), radial-gradient(rgb(2, 220, 227) 1px, rgba(35, 84, 84, 0) 1px);
                        background-position: 0 0, 15px 15px;
                        background-size: 30px 30px;
                        z-index: -1;
                    }
                    
                    
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
                        margin: 0 auto;
                        max-width: 800px;
                    }
                    
                    .grid {
                        display: grid;
                        grid-template-columns: repeat(1, minmax(200px, 1fr));
                        grid-auto-rows: minmax(0, 1fr);
                        justify-content: center;
                        gap: 40px;
                        margin-top: 40px;
                        
                        .location {
                            
                            align-items: center;
                            gap: 10px;
                            width: 240px;
                            margin: 0 auto;
                            place-items: center;
                            text-align: center;
                            
                            iframe {
                                height: 320px;
                            }
                            
                            p {
                                margin: 0;
                            }
                            
                            h3 {
                                font-size: 18px;
                                color: var(--turquoise-15);
                                margin-bottom: 8px;
                            }
                            
                            a {
                                color: var(--grey-50);
                            }
                            
                        }
                                            
                    }
                    
                }

                
                
            }
            
            
            
            
            section.services {
                
                background-color: #f5ffff;
                
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
                        margin: 0 auto 48px 0;
                        max-width: 800px;
                        width: 55%;
                    }
                    
                    .details-area {
                        
                        display: flex;
                        max-width: 100%;
                        margin: 0 auto;
                        
                        .grid-area {
                                             
                            width: 55%;
                            height: min-content;
                            position: relative;
                            padding: 0 48px 0 0;
                            position: relative;
                            margin: 0 auto 48px;
                            height: min-content;
                            
                            
                            .grid {
                                grid-template-columns: repeat(2, minmax(200px, 1fr));
                                grid-auto-rows: minmax(0, 1fr);
                                justify-content: center;
                                gap: 24px;
                                
                                padding: 0;
                                display: grid;
                                
                                
                                h3 {
                                    font-size: 18px;
                                    color: var(--turquoise-15);
                                    margin-bottom: 8px;
                                }
                                
                                p {
                                    font-size: 14px;
                                    font-weight: 300;
                                }
                                
                               
                                                    
                            }
                            
                            
                        }
                        
                        
                        .image-area {
                            
                            align-self: stretch;
                            flex: 1 0 0;
                            position: relative;
                            overflow: visible;
                            max-width: 720px;
                            height: auto;
                            flex-flow: column;
                            
                            img {
                                display: block;
                                object-fit: cover;
                                width: 100%;
                                height: 100%;
                                object-position: center center;
                                
                            }
                            
                            
                        }
                        
                        
                            
                        
                        
                    }
                    
                }

                
                
            }
            
            
            
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
            
            
            
            
            
            section.general-services {
                
                
                .inner {
                    position: relative;
                    max-width: 1200px;
                    
                    margin: 0 auto;
                    
                    padding: 120px var(--site-padding-x);
                    
                    text-align: center;
                
                    .polka-dots {
                        position: absolute;
                        width: 50%;
                        height: 480px;
                        top: 480px;
                        left: 0;
                        background-image: radial-gradient(rgb(2, 220, 227) 1px, transparent 1px), radial-gradient(rgb(2, 220, 227) 1px, rgba(35, 84, 84, 0) 1px);
                        background-position: 0 0, 15px 15px;
                        background-size: 30px 30px;
                        z-index: -1;
                    }
                    
                    
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
                        margin: 0 auto;
                        max-width: 800px;
                    }
                    
                    .grid {
                        display: flex;
                        justify-content: center;
                        gap: 40px;
                        margin-top: 40px;
                        
                        .service {
                            
                            align-items: center;
                            gap: 10px;
                            width: 240px;
                            margin: 0 auto;
                            place-items: center;
                            text-align: center;
                            
                            .icon {
                                background-color: transparent;
                                background-image: radial-gradient(rgb(2, 220, 227) 1px, transparent 1px), radial-gradient(rgb(2, 220, 227) 1px, rgba(35, 84, 84, 0) 1px);
                                background-position: 0 0, 12px 12px;
                                background-size: 24px 24px;
                                width: 168px;
                                height: 168px;
                                place-items: center;
                                place-content: center;
                                
                                .circle {
                                    border: 1px solid rgb(2, 146, 151);
                                    background-color: white;
                                    border-radius: 96px;
                                    height: 120px;
                                    width: 120px;
                                    place-content: center;
                                    
                                    &::after {
                                        content: "";
                                        border-width: var(--border-top-width, 0) var(--border-right-width, 0) var(--border-bottom-width, 0) var(--border-left-width, 0);
                                        border-color: var(--border-color, none);
                                        border-style: var(--border-style, none);
                                    }
                                    
                                    svg {
                                        fill: rgb(2, 146, 151);
                                        width: 72px;
                                        height: 72px;
                                    }
                                }
                                
                            }
                            
                            
                            p {
                                margin: 0;
                            }
                            
                            h3 {
                                font-size: 18px;
                                color: var(--turquoise-15);
                                margin-bottom: 8px;
                            }
                            
                            a {
                                color: var(--grey-50);
                            }
                            
                            ul {
                                margin: 0;
                                padding: 0;
                                text-align: center;
                                
                                li {
                                    list-style-type: none;
                                    color: var(--grey-50);
                                    position: relative;
                                    padding-left: 0;
                                    margin: 0;
                                    line-height: 1.6;
                                    
                                    &::before {
                                        content: "✓";
                                        color: var(--turquoise-30);
                                        position: relative;
                                        margin-right: 0.5em;
                                    }
                                }
                            }
                            
                        }
                                            
                    }
                    
                    .small-button {
                        
                        margin: 48px auto 0;
                    }
                    
                }

                
                
            }
            
            
            
            section.open-times {
            
                display: flex;
                padding: 128px var(--site-padding-x);
                width: 100%;
                background-color: #029297;
                align-items: center;
                justify-content: center;
                margin: 0 auto;
                position: relative;
                
                .inner {
                    
                    max-width: 1200px;
                    width: 100%;
                    
                    
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
                        background: linear-gradient(330deg, rgba(2, 145, 150, 0) 0.1%, #029297, rgb(2, 146, 151) 140%);

                    }
                    
                    
                    .text-area {
                        flex: 1;
                        display: flex;
                        flex-direction: column;
                        align-items: flex-start;
                        justify-content: center;
                        max-width: 400px;
                        z-index: 10;
                        
                        h2 {
                            font-size: 36px;
                            color: #68f8fd;
                            line-height: 1.4em;
                            z-index: 10;
                        }
                            
                        p {
                            font-size: 18px;
                            color: #fff;
                            line-height: 1.4em;
                            paragraph-spacing: 20px;
                            font-weight: 300;
                            z-index: 10;
                        }
                            
                        .buttons {
                            display: flex;
                            gap: 10px;
                            font-size: 22px;
                            font-weight: 500;
                            wrap: nowrap;
                            flex-flow: wrap;
                            height: min-content;
                            place-content: center flex-start;
                            align-items: center;
                        
                            
                            .primary {
                            
                                background-color: var(--turquoise-15);
                                color: white;
                            }
                            
                            .secondary {
                            
                                background-color: #fff;
                                color: var(--turquoise-15);
                            
                            }
                            
                            a {
                                padding: 0 36px;
                                height: 60px;
                                text-decoration: none;
                                align-self: center;
                                place-content: center;
                            }
                        }
                    }
                    
                    .times-area {
                        
                        display: flex;
                        align-items: center;
                        
                        z-index: 10;
                        
                        
                        
                        
                        .hours-area {
                            
                            width: 40%;
                            
                            h3 {
                                color: #68f8fd;
                                font-size: 18px;
                            }
                            
                            background-color: #01494b;
                            border-radius: 2px;
                            padding: 48px;
                            
                            height: fit-content;
                            z-index: 10;

                            
                            table {
                                width: 100%;
                                border-collapse: collapse;
                                
                                tr {
                                    border-bottom: 1px solid #68f8fd;
                                    
                                    &:last-child {
                                        border-bottom: none;
                                    }
                                    
                                    td {
                                        padding: 0.8rem 0;
                                        color: white;
                                        
                                        &:first-child {
                                            text-align: left;
                                        }
                                        
                                        &:last-child {
                                            text-align: right;
                                        }
                                    }
                                }
                            }
                        }
                        
                        .image-area {
                            
                            height: 574px;
                            width: 60%;
                            
                            background-image: url('/images/homepage/UBCKIXJtyeQQHICDWd76O00vxA.avif');
                            
                            object-fit: cover;
                            background-position: center;
                            background-size: cover;
                            border-radius: 2px;
                            position: relative;
                            z-index: 10;
                            
                            
                            
                            .buttons {
                                display: flex;
                                background: linear-gradient(180deg, rgba(255, 255, 255, 0) 0%, rgb(255, 255, 255) 100%);
                                width: 100%;
                                height: 100%;
                                gap: 10px;
                                font-size: 22px;
                                font-weight: 500;
                                wrap: nowrap;
                                flex-flow: wrap;
                                flex-direction: column;
                                place-items: center;
                                place-content: center;
                            
                                
                                .primary {
                                
                                    background-color: var(--turquoise-15);
                                    color: white;
                                }
                                
                                .secondary {
                                
                                    background-color: #fff;
                                    color: var(--turquoise-15);
                                 
                                }
                                
                                a {
                                    padding: 0 36px;
                                    height: 60px;
                                    text-decoration: none;
                                    align-self: center;
                                    place-content: center;
                                }
                            }
                            
                            
                        }
                        
                        
                    }
                    
                    
                }
                    
                
                
            }
            
            
                
            
            
            
            section.patient-stories {
                padding: 100px var(--site-padding-x);
                background: white;
                
                padding: 120px var(--site-padding-x);
                
                .text-area {
                    
                    margin-bottom: 48px;
                            
                    .subtitle {
                        font-size: 16px;
                        color: var(--turquoise-30);
                        font-weight: 600;
                        margin: 0 auto 24px;
                        text-align: center;
                        
                    }
                    
                    h2 {
                        text-align: center;
                        font-size: 42px;
                        color: var(--turquoise-15);
                        margin-bottom: 24px;
                    }
                    
                    p {
                        text-align: center;
                    }
                    
                    
                    
                }
                
                
                
                .testimonials {
                    display: grid;
                    grid-template-columns: repeat(3, 1fr);
                    gap: 32px;
                    max-width: 1200px;
                    margin: 0 auto;
                    
                    .testimonial {
                        padding: 32px;
                        background: var(--turquoise-98);
                        border-radius: 12px;
                        
                        .image-circle {
                            width: 96px;
                            height: 96px;
                            border-radius: 50%;
                            overflow: hidden;
                            margin: 0 auto 1rem;
                            
                            img {
                                width: 100%;
                                height: 100%;
                                object-fit: cover;
                                
                                /* Adjust the crop position of just the first image */
                                &:first-child {
                                    object-position: top;
                                }
                                
                            }
                            
                            
                            
                        }
                        
                        .stars {
                            color: #ffc107;
                            font-size: 24px;
                            margin-bottom: 16px;
                        }
                        
                        p {
                            color: var(--grey-25);
                            line-height: 1.6;
                            margin-bottom: 16px;
                            font-style: italic;
                        }
                        
                        cite {
                            color: var(--turquoise-30);
                            font-style: normal;
                            font-weight: 500;
                        }
                    }
                }
            }
        }
    "##);
}