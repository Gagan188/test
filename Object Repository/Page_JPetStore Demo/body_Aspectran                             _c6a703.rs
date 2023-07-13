<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Aspectran                             _c6a703</name>
   <tag></tag>
   <elementGuidId>47d11041-d352-4630-befc-a53a59f4d598</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#top-of-page</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body[@id='top-of-page']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>76bb8499-cae9-4138-a9ad-59c91ca47631</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>top-of-page</value>
      <webElementGuid>5f1a58a1-480b-4268-984a-d4cebf4c23ef</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>plate solid</value>
      <webElementGuid>19091064-36f7-4746-ac09-1d7157a48a92</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>itemtype</name>
      <type>Main</type>
      <value>http://schema.org/WebPage</value>
      <webElementGuid>5594f29f-a304-4ef7-a034-04a494132d82</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

    
        
            
        
        
            Aspectran
        
        
            
        
    
    
        
            
                
                    
                
            
            
                
                    
                        JPetStore Demo
                    
                    
                        APM
                    
                
            
            
                
                    
                        More Demo Apps
                        
                            Aspectran Demo
                            Log Relay
                        
                    
                
                
                    
                        
                            
                            
                                
                            
                        
                    
                
            
        
        
            
                
                    
                        Demo Apps
                        JPetStore Demo
                    
                
            
        
    


    
        
            
                
                    
                        
                        JPetStore Demo
                        
                            JPetStore is a full web application built on top of MyBatis 3, Aspectran 7.
                        
                    
                    
                        
                        
                        
                        
                        
                    
                
            
        
        
            
                
                    
                        Demo Apps
                        JPetStore Demo
                    
                
            
        
    
    
    
    
        
            








	
		
			
		
	

	
		
			
			
				
				Sign In
				
				Sign Up
			
			
			
			?
		
	

	
		
			
				
					
					
						Search
					
				
			
		
	

	
	
		Fish
	
		Dogs
	
		Reptiles
	
		Cats
	
		Birds
	
	





	Return to Main Menu




	Dogs

	
		
			Product ID
			Name
		
		
			
				
					K9-BD-01
				
				Bulldog
			
		
			
				
					K9-PO-02
				
				Poodle
			
		
			
				
					K9-DL-01
				
				Dalmation
			
		
			
				
					K9-RT-01
				
				Golden Retriever
			
		
			
				
					K9-RT-02
				
				Labrador Retriever
			
		
			
				
					K9-CW-01
				
				Chihuahua
			
		
	







	
		aspectran.com
		mybatis.org
	
	
		
	



        
    
    


    
        
            
        
    


    
        
            
                
                    
                
                
                    About Aspectran
                    Aspectran is a framework for developing Java applications that can be used to build simple shell applications and large enterprise web applications.
                
                
                    Get Involved
                    
                         GitHub
                    
                
                
                    Support
                    
                        FAQ
                        Contact Us
                    
                
            
        
    
    
        
            
                
                    Copyright © 2008-2023 The Aspectran Project
                
                
                    Powered by Aspectran 7.3.3
                
            
        
    



    $(document).foundation();
    $(function() {
        let $win = $(window);
        let $nav = $(&quot;#navigation&quot;);
        let navHeight = $(&quot;#masthead&quot;).height() - $nav.height();
        let lastScrollTop = 0;
        let scrolled;
        let navFixed;
        $win.scroll(function() {
            scrolled = true;
        });
        setInterval(function() {
            if (scrolled) {
                let scrollTop = $win.scrollTop();
                if (Math.abs(lastScrollTop - scrollTop) &lt;= 10) {
                    return;
                }
                if (scrollTop &lt;= navHeight) {
                    if (navFixed) {
                        $nav.removeClass(&quot;fixed&quot;);
                        navFixed = false;
                    }
                } else if (scrollTop > lastScrollTop) {
                    if (navFixed) {
                        $nav.removeClass(&quot;fixed&quot;);
                        navFixed = false;
                    }
                } else {
                    if (!navFixed) {
                        if ($nav.hasClass(&quot;immediate&quot;)) {
                            $nav.removeClass(&quot;immediate&quot;)
                        } else {
                            $nav.addClass(&quot;fixed&quot;);
                            $nav.hide().fadeIn(500);
                            navFixed = true;
                        }
                    }
                }
                lastScrollTop = scrollTop;
                scrolled = false;
            }
        }, 200);
        /* google search */
        $(&quot;form[name=google_quick_search]&quot;).submit(function(event) {
            window.open('https://www.google.com/search?q=' + this.keyword.value + '+site:{{ site.url | cgi_escape }}');
            event.preventDefault();
        });
    });


    $(function() {
        $(&quot;#masthead h1, article h1, article h2, article h3, article h4, article h5, article h6&quot;).each(function(index, item) {
            let tagn = item.localName;
            let anchor = &quot;top-of-page&quot;;
            if (tagn !== &quot;h1&quot;) {
                anchor = &quot;anchor-&quot; + (index + 1);
                $(this).before(&quot;&lt;a class='toc-anchor &quot; + anchor + &quot;' id='&quot; + anchor + &quot;' name='&quot; + anchor + &quot;'>&lt;/a>&quot;);
            }
            $(&quot;#toc ul&quot;).append(&quot;&lt;li class='toc-&quot; + tagn + &quot;'>&lt;a anchor='&quot; + anchor + &quot;' href='#&quot; + anchor + &quot;'>&quot; + $(item).text() + &quot;&lt;/a>&lt;/li>&quot;);
        });
    });


    $(function() {
        $(&quot;.lazy-sticky&quot;).each(function() {
            const $win = $(window);
            const $this = $(this);
            const baseOffsetTop = $this.offset().top;
            const upToTopHeight = $(&quot;#up-to-top&quot;).height() + 30 + 60;
            let footerHeight = $(&quot;#footer-content&quot;).height() + upToTopHeight;
            let offsetTop = 0;
            let thisHeight = $this.height();
            let winHeight = $win.height();
            let scrollTimer = null;
            let immediate = false;
            $this.find(&quot;#toc ul a&quot;).click(function(e) {
                immediate = true;
                let anchor = $(this).attr(&quot;anchor&quot;);
                if (anchor !== &quot;top-of-page&quot;) {
                    $(&quot;#navigation&quot;).addClass(&quot;immediate&quot;);
                }
            });
            $win.scroll(function() {
                let scrollTop = $win.scrollTop();
                if (scrollTop &lt; baseOffsetTop) {
                    if (scrollTimer) {
                        clearInterval(scrollTimer);
                        scrollTimer = null;
                    }
                    scrollTimer = setInterval(function() {
                        if (offsetTop !== 0) {
                            $this.css({
                                top: 0
                            });
                        }
                        offsetTop = 0;
                        clearInterval(scrollTimer);
                        scrollTimer = null;
                        immediate = false;
                    }, immediate ? 250 : 500);
                } else {
                    let topBarHeight = $(&quot;#navigation.fixed .top-bar&quot;).height()||0;
                    if (immediate || (scrollTop > baseOffsetTop + topBarHeight + offsetTop + thisHeight - 20) ||
                        (scrollTop &lt; baseOffsetTop + topBarHeight + offsetTop)) {
                        if ($this.offset().left >= 15 &amp;&amp; $this.width() &lt; 500) {
                            if (scrollTimer) {
                                clearInterval(scrollTimer);
                                scrollTimer = null;
                            }
                            scrollTimer = setInterval(function() {
                                topBarHeight = $(&quot;#navigation.fixed .top-bar&quot;).height()||0;
                                scrollTop = $win.scrollTop();
                                if (scrollTop &lt; baseOffsetTop + topBarHeight) {
                                    scrollTop = 0;
                                } else {
                                    scrollTop = scrollTop - baseOffsetTop + topBarHeight + 30;
                                }
                                if (scrollTop > $(document).height() - footerHeight - thisHeight - baseOffsetTop + topBarHeight) {
                                    scrollTop = $(document).height() - footerHeight - thisHeight - baseOffsetTop + topBarHeight;
                                }
                                offsetTop = scrollTop;
                                $this.css({
                                    position: &quot;relative&quot;
                                });
                                $this.animate({
                                    top: scrollTop + &quot;px&quot;
                                }, 300);
                                clearInterval(scrollTimer);
                                scrollTimer = null;
                                winHeight = $win.height();
                                thisHeight = $this.height();
                                footerHeight = $(&quot;#footer-content&quot;).height() + upToTopHeight;
                                immediate = false;
                            }, immediate ? 250 : 500);
                        }
                    }
                }
            });
            $win.resize(function() {
                if ($this.offset().left &lt; 15 || $this.width() >= 500) {
                    clearInterval(scrollTimer);
                    $this.css(&quot;top&quot;, 0);
                } else {
                    $win.scroll();
                }
            });
            setTimeout(function() {
                if ($win.scrollTop() > baseOffsetTop) {
                    offsetTop = $win.scrollTop();
                    $win.scroll();
                }
            }, 150);
        });
    });


    /* Creating custom :external selector */
    $.expr[':'].external = function(obj) {
        return obj.href
            &amp;&amp; !obj.href.match(/^javascript:/)
            &amp;&amp; !obj.href.match(/^mailto:/)
            &amp;&amp; (obj.hostname !== location.hostname);
    };
    $(function() {
        /* Add 'external' CSS class to all external links */
        $('a:external').addClass('external');
        /* turn target into target=_blank for elements w external class */
        $(&quot;.external&quot;).attr('target','_blank');
    })


    $(function() {
        let menuitem = $(&quot;#gnb-menu .dropdown li a[href='&quot; + location.pathname + &quot;']&quot;).last();
        if (menuitem.length > 0) {
            let arr = [];
            arr.push({'name': menuitem.text(), 'href': null});
            menuitem.parentsUntil(&quot;.dropdown > li:eq(0)&quot;).each(function() {
                if ($(this).hasClass(&quot;menu&quot;)) {
                    let a2 = $(this).prev();
                    if (a2.is(&quot;a&quot;)) {
                        arr.push({'name': a2.text(), 'href': a2.attr(&quot;href&quot;) || &quot;&quot;});
                    }
                }
            });
            arr.reverse();
            for (let i in arr) {
                let item = arr[i];
                if (i &lt; arr.length - 1) {
                    $(&quot;.breadcrumbs&quot;).append(&quot;&lt;li>&lt;a href='&quot; + item.href + &quot;'>&quot; + item.name + &quot;&lt;/a>&lt;/li>&quot;);
                } else {
                    $(&quot;.breadcrumbs&quot;).append(&quot;&lt;li>&lt;span class='show-for-sr'>Current: &lt;/span> &lt;span class='current'>&quot; + item.name + &quot;&lt;/span>&lt;/li>&quot;);
                }
            }
        }
    });



id(&quot;Catalog&quot;)/table[1]/tbody[1]/tr[2]/td[1]/a[1]</value>
      <webElementGuid>0a0985f8-440c-4abd-8c13-d5f8856df2c5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;top-of-page&quot;)</value>
      <webElementGuid>70cc16ed-7043-471f-84d3-d4a19cc8db46</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//body[@id='top-of-page']</value>
      <webElementGuid>1642ad09-2411-4c50-825e-cab6b356e685</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>265460c9-612b-4c38-9bb9-2b0f360e8a9c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[@id = 'top-of-page' and (text() = concat(&quot;

    
        
            
        
        
            Aspectran
        
        
            
        
    
    
        
            
                
                    
                
            
            
                
                    
                        JPetStore Demo
                    
                    
                        APM
                    
                
            
            
                
                    
                        More Demo Apps
                        
                            Aspectran Demo
                            Log Relay
                        
                    
                
                
                    
                        
                            
                            
                                
                            
                        
                    
                
            
        
        
            
                
                    
                        Demo Apps
                        JPetStore Demo
                    
                
            
        
    


    
        
            
                
                    
                        
                        JPetStore Demo
                        
                            JPetStore is a full web application built on top of MyBatis 3, Aspectran 7.
                        
                    
                    
                        
                        
                        
                        
                        
                    
                
            
        
        
            
                
                    
                        Demo Apps
                        JPetStore Demo
                    
                
            
        
    
    
    
    
        
            








	
		
			
		
	

	
		
			
			
				
				Sign In
				
				Sign Up
			
			
			
			?
		
	

	
		
			
				
					
					
						Search
					
				
			
		
	

	
	
		Fish
	
		Dogs
	
		Reptiles
	
		Cats
	
		Birds
	
	





	Return to Main Menu




	Dogs

	
		
			Product ID
			Name
		
		
			
				
					K9-BD-01
				
				Bulldog
			
		
			
				
					K9-PO-02
				
				Poodle
			
		
			
				
					K9-DL-01
				
				Dalmation
			
		
			
				
					K9-RT-01
				
				Golden Retriever
			
		
			
				
					K9-RT-02
				
				Labrador Retriever
			
		
			
				
					K9-CW-01
				
				Chihuahua
			
		
	







	
		aspectran.com
		mybatis.org
	
	
		
	



        
    
    


    
        
            
        
    


    
        
            
                
                    
                
                
                    About Aspectran
                    Aspectran is a framework for developing Java applications that can be used to build simple shell applications and large enterprise web applications.
                
                
                    Get Involved
                    
                         GitHub
                    
                
                
                    Support
                    
                        FAQ
                        Contact Us
                    
                
            
        
    
    
        
            
                
                    Copyright © 2008-2023 The Aspectran Project
                
                
                    Powered by Aspectran 7.3.3
                
            
        
    



    $(document).foundation();
    $(function() {
        let $win = $(window);
        let $nav = $(&quot;#navigation&quot;);
        let navHeight = $(&quot;#masthead&quot;).height() - $nav.height();
        let lastScrollTop = 0;
        let scrolled;
        let navFixed;
        $win.scroll(function() {
            scrolled = true;
        });
        setInterval(function() {
            if (scrolled) {
                let scrollTop = $win.scrollTop();
                if (Math.abs(lastScrollTop - scrollTop) &lt;= 10) {
                    return;
                }
                if (scrollTop &lt;= navHeight) {
                    if (navFixed) {
                        $nav.removeClass(&quot;fixed&quot;);
                        navFixed = false;
                    }
                } else if (scrollTop > lastScrollTop) {
                    if (navFixed) {
                        $nav.removeClass(&quot;fixed&quot;);
                        navFixed = false;
                    }
                } else {
                    if (!navFixed) {
                        if ($nav.hasClass(&quot;immediate&quot;)) {
                            $nav.removeClass(&quot;immediate&quot;)
                        } else {
                            $nav.addClass(&quot;fixed&quot;);
                            $nav.hide().fadeIn(500);
                            navFixed = true;
                        }
                    }
                }
                lastScrollTop = scrollTop;
                scrolled = false;
            }
        }, 200);
        /* google search */
        $(&quot;form[name=google_quick_search]&quot;).submit(function(event) {
            window.open(&quot; , &quot;'&quot; , &quot;https://www.google.com/search?q=&quot; , &quot;'&quot; , &quot; + this.keyword.value + &quot; , &quot;'&quot; , &quot;+site:{{ site.url | cgi_escape }}&quot; , &quot;'&quot; , &quot;);
            event.preventDefault();
        });
    });


    $(function() {
        $(&quot;#masthead h1, article h1, article h2, article h3, article h4, article h5, article h6&quot;).each(function(index, item) {
            let tagn = item.localName;
            let anchor = &quot;top-of-page&quot;;
            if (tagn !== &quot;h1&quot;) {
                anchor = &quot;anchor-&quot; + (index + 1);
                $(this).before(&quot;&lt;a class=&quot; , &quot;'&quot; , &quot;toc-anchor &quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot; id=&quot; , &quot;'&quot; , &quot;&quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot; name=&quot; , &quot;'&quot; , &quot;&quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot;>&lt;/a>&quot;);
            }
            $(&quot;#toc ul&quot;).append(&quot;&lt;li class=&quot; , &quot;'&quot; , &quot;toc-&quot; + tagn + &quot;&quot; , &quot;'&quot; , &quot;>&lt;a anchor=&quot; , &quot;'&quot; , &quot;&quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot; href=&quot; , &quot;'&quot; , &quot;#&quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot;>&quot; + $(item).text() + &quot;&lt;/a>&lt;/li>&quot;);
        });
    });


    $(function() {
        $(&quot;.lazy-sticky&quot;).each(function() {
            const $win = $(window);
            const $this = $(this);
            const baseOffsetTop = $this.offset().top;
            const upToTopHeight = $(&quot;#up-to-top&quot;).height() + 30 + 60;
            let footerHeight = $(&quot;#footer-content&quot;).height() + upToTopHeight;
            let offsetTop = 0;
            let thisHeight = $this.height();
            let winHeight = $win.height();
            let scrollTimer = null;
            let immediate = false;
            $this.find(&quot;#toc ul a&quot;).click(function(e) {
                immediate = true;
                let anchor = $(this).attr(&quot;anchor&quot;);
                if (anchor !== &quot;top-of-page&quot;) {
                    $(&quot;#navigation&quot;).addClass(&quot;immediate&quot;);
                }
            });
            $win.scroll(function() {
                let scrollTop = $win.scrollTop();
                if (scrollTop &lt; baseOffsetTop) {
                    if (scrollTimer) {
                        clearInterval(scrollTimer);
                        scrollTimer = null;
                    }
                    scrollTimer = setInterval(function() {
                        if (offsetTop !== 0) {
                            $this.css({
                                top: 0
                            });
                        }
                        offsetTop = 0;
                        clearInterval(scrollTimer);
                        scrollTimer = null;
                        immediate = false;
                    }, immediate ? 250 : 500);
                } else {
                    let topBarHeight = $(&quot;#navigation.fixed .top-bar&quot;).height()||0;
                    if (immediate || (scrollTop > baseOffsetTop + topBarHeight + offsetTop + thisHeight - 20) ||
                        (scrollTop &lt; baseOffsetTop + topBarHeight + offsetTop)) {
                        if ($this.offset().left >= 15 &amp;&amp; $this.width() &lt; 500) {
                            if (scrollTimer) {
                                clearInterval(scrollTimer);
                                scrollTimer = null;
                            }
                            scrollTimer = setInterval(function() {
                                topBarHeight = $(&quot;#navigation.fixed .top-bar&quot;).height()||0;
                                scrollTop = $win.scrollTop();
                                if (scrollTop &lt; baseOffsetTop + topBarHeight) {
                                    scrollTop = 0;
                                } else {
                                    scrollTop = scrollTop - baseOffsetTop + topBarHeight + 30;
                                }
                                if (scrollTop > $(document).height() - footerHeight - thisHeight - baseOffsetTop + topBarHeight) {
                                    scrollTop = $(document).height() - footerHeight - thisHeight - baseOffsetTop + topBarHeight;
                                }
                                offsetTop = scrollTop;
                                $this.css({
                                    position: &quot;relative&quot;
                                });
                                $this.animate({
                                    top: scrollTop + &quot;px&quot;
                                }, 300);
                                clearInterval(scrollTimer);
                                scrollTimer = null;
                                winHeight = $win.height();
                                thisHeight = $this.height();
                                footerHeight = $(&quot;#footer-content&quot;).height() + upToTopHeight;
                                immediate = false;
                            }, immediate ? 250 : 500);
                        }
                    }
                }
            });
            $win.resize(function() {
                if ($this.offset().left &lt; 15 || $this.width() >= 500) {
                    clearInterval(scrollTimer);
                    $this.css(&quot;top&quot;, 0);
                } else {
                    $win.scroll();
                }
            });
            setTimeout(function() {
                if ($win.scrollTop() > baseOffsetTop) {
                    offsetTop = $win.scrollTop();
                    $win.scroll();
                }
            }, 150);
        });
    });


    /* Creating custom :external selector */
    $.expr[&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;].external = function(obj) {
        return obj.href
            &amp;&amp; !obj.href.match(/^javascript:/)
            &amp;&amp; !obj.href.match(/^mailto:/)
            &amp;&amp; (obj.hostname !== location.hostname);
    };
    $(function() {
        /* Add &quot; , &quot;'&quot; , &quot;external&quot; , &quot;'&quot; , &quot; CSS class to all external links */
        $(&quot; , &quot;'&quot; , &quot;a:external&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;external&quot; , &quot;'&quot; , &quot;);
        /* turn target into target=_blank for elements w external class */
        $(&quot;.external&quot;).attr(&quot; , &quot;'&quot; , &quot;target&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;);
    })


    $(function() {
        let menuitem = $(&quot;#gnb-menu .dropdown li a[href=&quot; , &quot;'&quot; , &quot;&quot; + location.pathname + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).last();
        if (menuitem.length > 0) {
            let arr = [];
            arr.push({&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;: menuitem.text(), &quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;: null});
            menuitem.parentsUntil(&quot;.dropdown > li:eq(0)&quot;).each(function() {
                if ($(this).hasClass(&quot;menu&quot;)) {
                    let a2 = $(this).prev();
                    if (a2.is(&quot;a&quot;)) {
                        arr.push({&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;: a2.text(), &quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;: a2.attr(&quot;href&quot;) || &quot;&quot;});
                    }
                }
            });
            arr.reverse();
            for (let i in arr) {
                let item = arr[i];
                if (i &lt; arr.length - 1) {
                    $(&quot;.breadcrumbs&quot;).append(&quot;&lt;li>&lt;a href=&quot; , &quot;'&quot; , &quot;&quot; + item.href + &quot;&quot; , &quot;'&quot; , &quot;>&quot; + item.name + &quot;&lt;/a>&lt;/li>&quot;);
                } else {
                    $(&quot;.breadcrumbs&quot;).append(&quot;&lt;li>&lt;span class=&quot; , &quot;'&quot; , &quot;show-for-sr&quot; , &quot;'&quot; , &quot;>Current: &lt;/span> &lt;span class=&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;>&quot; + item.name + &quot;&lt;/span>&lt;/li>&quot;);
                }
            }
        }
    });



id(&quot;Catalog&quot;)/table[1]/tbody[1]/tr[2]/td[1]/a[1]&quot;) or . = concat(&quot;

    
        
            
        
        
            Aspectran
        
        
            
        
    
    
        
            
                
                    
                
            
            
                
                    
                        JPetStore Demo
                    
                    
                        APM
                    
                
            
            
                
                    
                        More Demo Apps
                        
                            Aspectran Demo
                            Log Relay
                        
                    
                
                
                    
                        
                            
                            
                                
                            
                        
                    
                
            
        
        
            
                
                    
                        Demo Apps
                        JPetStore Demo
                    
                
            
        
    


    
        
            
                
                    
                        
                        JPetStore Demo
                        
                            JPetStore is a full web application built on top of MyBatis 3, Aspectran 7.
                        
                    
                    
                        
                        
                        
                        
                        
                    
                
            
        
        
            
                
                    
                        Demo Apps
                        JPetStore Demo
                    
                
            
        
    
    
    
    
        
            








	
		
			
		
	

	
		
			
			
				
				Sign In
				
				Sign Up
			
			
			
			?
		
	

	
		
			
				
					
					
						Search
					
				
			
		
	

	
	
		Fish
	
		Dogs
	
		Reptiles
	
		Cats
	
		Birds
	
	





	Return to Main Menu




	Dogs

	
		
			Product ID
			Name
		
		
			
				
					K9-BD-01
				
				Bulldog
			
		
			
				
					K9-PO-02
				
				Poodle
			
		
			
				
					K9-DL-01
				
				Dalmation
			
		
			
				
					K9-RT-01
				
				Golden Retriever
			
		
			
				
					K9-RT-02
				
				Labrador Retriever
			
		
			
				
					K9-CW-01
				
				Chihuahua
			
		
	







	
		aspectran.com
		mybatis.org
	
	
		
	



        
    
    


    
        
            
        
    


    
        
            
                
                    
                
                
                    About Aspectran
                    Aspectran is a framework for developing Java applications that can be used to build simple shell applications and large enterprise web applications.
                
                
                    Get Involved
                    
                         GitHub
                    
                
                
                    Support
                    
                        FAQ
                        Contact Us
                    
                
            
        
    
    
        
            
                
                    Copyright © 2008-2023 The Aspectran Project
                
                
                    Powered by Aspectran 7.3.3
                
            
        
    



    $(document).foundation();
    $(function() {
        let $win = $(window);
        let $nav = $(&quot;#navigation&quot;);
        let navHeight = $(&quot;#masthead&quot;).height() - $nav.height();
        let lastScrollTop = 0;
        let scrolled;
        let navFixed;
        $win.scroll(function() {
            scrolled = true;
        });
        setInterval(function() {
            if (scrolled) {
                let scrollTop = $win.scrollTop();
                if (Math.abs(lastScrollTop - scrollTop) &lt;= 10) {
                    return;
                }
                if (scrollTop &lt;= navHeight) {
                    if (navFixed) {
                        $nav.removeClass(&quot;fixed&quot;);
                        navFixed = false;
                    }
                } else if (scrollTop > lastScrollTop) {
                    if (navFixed) {
                        $nav.removeClass(&quot;fixed&quot;);
                        navFixed = false;
                    }
                } else {
                    if (!navFixed) {
                        if ($nav.hasClass(&quot;immediate&quot;)) {
                            $nav.removeClass(&quot;immediate&quot;)
                        } else {
                            $nav.addClass(&quot;fixed&quot;);
                            $nav.hide().fadeIn(500);
                            navFixed = true;
                        }
                    }
                }
                lastScrollTop = scrollTop;
                scrolled = false;
            }
        }, 200);
        /* google search */
        $(&quot;form[name=google_quick_search]&quot;).submit(function(event) {
            window.open(&quot; , &quot;'&quot; , &quot;https://www.google.com/search?q=&quot; , &quot;'&quot; , &quot; + this.keyword.value + &quot; , &quot;'&quot; , &quot;+site:{{ site.url | cgi_escape }}&quot; , &quot;'&quot; , &quot;);
            event.preventDefault();
        });
    });


    $(function() {
        $(&quot;#masthead h1, article h1, article h2, article h3, article h4, article h5, article h6&quot;).each(function(index, item) {
            let tagn = item.localName;
            let anchor = &quot;top-of-page&quot;;
            if (tagn !== &quot;h1&quot;) {
                anchor = &quot;anchor-&quot; + (index + 1);
                $(this).before(&quot;&lt;a class=&quot; , &quot;'&quot; , &quot;toc-anchor &quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot; id=&quot; , &quot;'&quot; , &quot;&quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot; name=&quot; , &quot;'&quot; , &quot;&quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot;>&lt;/a>&quot;);
            }
            $(&quot;#toc ul&quot;).append(&quot;&lt;li class=&quot; , &quot;'&quot; , &quot;toc-&quot; + tagn + &quot;&quot; , &quot;'&quot; , &quot;>&lt;a anchor=&quot; , &quot;'&quot; , &quot;&quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot; href=&quot; , &quot;'&quot; , &quot;#&quot; + anchor + &quot;&quot; , &quot;'&quot; , &quot;>&quot; + $(item).text() + &quot;&lt;/a>&lt;/li>&quot;);
        });
    });


    $(function() {
        $(&quot;.lazy-sticky&quot;).each(function() {
            const $win = $(window);
            const $this = $(this);
            const baseOffsetTop = $this.offset().top;
            const upToTopHeight = $(&quot;#up-to-top&quot;).height() + 30 + 60;
            let footerHeight = $(&quot;#footer-content&quot;).height() + upToTopHeight;
            let offsetTop = 0;
            let thisHeight = $this.height();
            let winHeight = $win.height();
            let scrollTimer = null;
            let immediate = false;
            $this.find(&quot;#toc ul a&quot;).click(function(e) {
                immediate = true;
                let anchor = $(this).attr(&quot;anchor&quot;);
                if (anchor !== &quot;top-of-page&quot;) {
                    $(&quot;#navigation&quot;).addClass(&quot;immediate&quot;);
                }
            });
            $win.scroll(function() {
                let scrollTop = $win.scrollTop();
                if (scrollTop &lt; baseOffsetTop) {
                    if (scrollTimer) {
                        clearInterval(scrollTimer);
                        scrollTimer = null;
                    }
                    scrollTimer = setInterval(function() {
                        if (offsetTop !== 0) {
                            $this.css({
                                top: 0
                            });
                        }
                        offsetTop = 0;
                        clearInterval(scrollTimer);
                        scrollTimer = null;
                        immediate = false;
                    }, immediate ? 250 : 500);
                } else {
                    let topBarHeight = $(&quot;#navigation.fixed .top-bar&quot;).height()||0;
                    if (immediate || (scrollTop > baseOffsetTop + topBarHeight + offsetTop + thisHeight - 20) ||
                        (scrollTop &lt; baseOffsetTop + topBarHeight + offsetTop)) {
                        if ($this.offset().left >= 15 &amp;&amp; $this.width() &lt; 500) {
                            if (scrollTimer) {
                                clearInterval(scrollTimer);
                                scrollTimer = null;
                            }
                            scrollTimer = setInterval(function() {
                                topBarHeight = $(&quot;#navigation.fixed .top-bar&quot;).height()||0;
                                scrollTop = $win.scrollTop();
                                if (scrollTop &lt; baseOffsetTop + topBarHeight) {
                                    scrollTop = 0;
                                } else {
                                    scrollTop = scrollTop - baseOffsetTop + topBarHeight + 30;
                                }
                                if (scrollTop > $(document).height() - footerHeight - thisHeight - baseOffsetTop + topBarHeight) {
                                    scrollTop = $(document).height() - footerHeight - thisHeight - baseOffsetTop + topBarHeight;
                                }
                                offsetTop = scrollTop;
                                $this.css({
                                    position: &quot;relative&quot;
                                });
                                $this.animate({
                                    top: scrollTop + &quot;px&quot;
                                }, 300);
                                clearInterval(scrollTimer);
                                scrollTimer = null;
                                winHeight = $win.height();
                                thisHeight = $this.height();
                                footerHeight = $(&quot;#footer-content&quot;).height() + upToTopHeight;
                                immediate = false;
                            }, immediate ? 250 : 500);
                        }
                    }
                }
            });
            $win.resize(function() {
                if ($this.offset().left &lt; 15 || $this.width() >= 500) {
                    clearInterval(scrollTimer);
                    $this.css(&quot;top&quot;, 0);
                } else {
                    $win.scroll();
                }
            });
            setTimeout(function() {
                if ($win.scrollTop() > baseOffsetTop) {
                    offsetTop = $win.scrollTop();
                    $win.scroll();
                }
            }, 150);
        });
    });


    /* Creating custom :external selector */
    $.expr[&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;].external = function(obj) {
        return obj.href
            &amp;&amp; !obj.href.match(/^javascript:/)
            &amp;&amp; !obj.href.match(/^mailto:/)
            &amp;&amp; (obj.hostname !== location.hostname);
    };
    $(function() {
        /* Add &quot; , &quot;'&quot; , &quot;external&quot; , &quot;'&quot; , &quot; CSS class to all external links */
        $(&quot; , &quot;'&quot; , &quot;a:external&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;external&quot; , &quot;'&quot; , &quot;);
        /* turn target into target=_blank for elements w external class */
        $(&quot;.external&quot;).attr(&quot; , &quot;'&quot; , &quot;target&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;);
    })


    $(function() {
        let menuitem = $(&quot;#gnb-menu .dropdown li a[href=&quot; , &quot;'&quot; , &quot;&quot; + location.pathname + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).last();
        if (menuitem.length > 0) {
            let arr = [];
            arr.push({&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;: menuitem.text(), &quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;: null});
            menuitem.parentsUntil(&quot;.dropdown > li:eq(0)&quot;).each(function() {
                if ($(this).hasClass(&quot;menu&quot;)) {
                    let a2 = $(this).prev();
                    if (a2.is(&quot;a&quot;)) {
                        arr.push({&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;: a2.text(), &quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;: a2.attr(&quot;href&quot;) || &quot;&quot;});
                    }
                }
            });
            arr.reverse();
            for (let i in arr) {
                let item = arr[i];
                if (i &lt; arr.length - 1) {
                    $(&quot;.breadcrumbs&quot;).append(&quot;&lt;li>&lt;a href=&quot; , &quot;'&quot; , &quot;&quot; + item.href + &quot;&quot; , &quot;'&quot; , &quot;>&quot; + item.name + &quot;&lt;/a>&lt;/li>&quot;);
                } else {
                    $(&quot;.breadcrumbs&quot;).append(&quot;&lt;li>&lt;span class=&quot; , &quot;'&quot; , &quot;show-for-sr&quot; , &quot;'&quot; , &quot;>Current: &lt;/span> &lt;span class=&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;>&quot; + item.name + &quot;&lt;/span>&lt;/li>&quot;);
                }
            }
        }
    });



id(&quot;Catalog&quot;)/table[1]/tbody[1]/tr[2]/td[1]/a[1]&quot;))]</value>
      <webElementGuid>41877aa1-cf66-42f4-820f-b27598558e0a</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
