<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Create New Customer Accoun</name>
   <tag></tag>
   <elementGuidId>323f6905-4388-470e-a4da-51c40466e021</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//main[@id='maincontent']/div[2]/div/div/div/div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>vcms-standard-output</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        Create New Customer Account    
    


    
        
           or        
    
    
                                                 
    	
	Facebook Sign in

    
    var newwindow;
    var intId;
    function fbLogin(){
        var  screenX    = typeof window.screenX != 'undefined' ? window.screenX : window.screenLeft;
        var	 screenY    = typeof window.screenY != 'undefined' ? window.screenY : window.screenTop;
        var	 outerWidth = typeof window.outerWidth != 'undefined' ? window.outerWidth : document.body.clientWidth;
        var	 outerHeight = typeof window.outerHeight != 'undefined' ? window.outerHeight : (document.body.clientHeight - 22);
        var	 width    = 500;
        var	 height   = 270;
        var	 left     = parseInt(screenX + ((outerWidth - width) / 2), 10);
        var	 top      = parseInt(screenY + ((outerHeight - height) / 2.5), 10);
        var	 features = (
                'width=' + width +
                ',height=' + height +
                ',left=' + left +
                ',top=' + top
              );
    
        newwindow=window.open('https://www.facebook.com/dialog/oauth?client_id=1565488793524246&amp;redirect_uri=https%3A%2F%2Fwww.masoko.com%2Fsociallogin%2Fsociallogin%2Ffblogin%2Fauth%2F1%2F&amp;state=743a3c44542a3bbbf08bd725783612e6&amp;display=popup&amp;scope=email','Login_by_facebook',features);
    
        if (window.focus) {
            newwindow.focus()
        }

        jQuery(document).trigger('magestoreInvalidateSections');

        return false;
    }
    
   
 

                
                    require([
                        'prototype'
                    ], function  () {

                        if($('bt-loginfb' ))
                            $('bt-loginfb').addClassName('visible');
                        if($('bt-loginfb' ))
                            $('bt-loginfb' ).setStyle('min-width: 80px');


                    });

                
                                                                                         
	Google Sign in


var newwindow;
var intId;
function goLogin(){
	var  screenX    = typeof window.screenX != 'undefined' ? window.screenX : window.screenLeft;
	var	 screenY    = typeof window.screenY != 'undefined' ? window.screenY : window.screenTop;
	var	 outerWidth = typeof window.outerWidth != 'undefined' ? window.outerWidth : document.body.clientWidth;
	var	 outerHeight = typeof window.outerHeight != 'undefined' ? window.outerHeight : (document.body.clientHeight - 22);
	var	 width    = 700;
	var	 height   = 400;
	var	 left     = parseInt(screenX + ((outerWidth - width) / 2), 10);
	var	 top      = parseInt(screenY + ((outerHeight - height) / 2.5), 10);
	var	 features = (
			'width=' + width +
			',height=' + height +
			',left=' + left +
			',top=' + top
		  );

	newwindow=window.open('https://www.masoko.com/sociallogin/sociallogin/gologin/','Login_by_google',features);

	if (window.focus) {
		newwindow.focus()
	}
	return false;
}

 

                
                    require([
                        'prototype'
                    ], function  () {

                        if($('bt-logingo' ))
                            $('bt-logingo').addClassName('visible');
                        if($('bt-logingo' ))
                            $('bt-logingo' ).setStyle('min-width: 80px');


                    });

                
                                                    
    




    function showOtherButton(){
        $('social_login_popup').show();
        centerWindow('sociallogin_button');
        centerWindow('magestore-popup_social');
    }
    function hideShownButtons(number){
        i = 0;
        $$('#social_login_popup ul li').each(function(el){
            if(i&lt;number){
                el.hide();
                i++;
            }else el.show();
        });
    }

    function centerWindow(element) {
        var windowHeight = parseFloat($(element).getHeight())/2;
        var windowWidth = parseFloat($(element).getWidth())/2;
        if(typeof window.innerHeight != 'undefined') {
            $(element).style.top = Math.round(document.body.offsetTop + ((window.innerHeight - $(element).getHeight()))/2)+'px';
            $(element).style.left = Math.round(document.body.offsetLeft + ((window.innerWidth - $(element).getWidth()))/2)+'px';
        } else {
            $(element).style.top = Math.round(document.body.offsetTop + ((document.documentElement.offsetHeight - $(element).getHeight()))/2)+'px';
            $(element).style.left = Math.round(document.body.offsetLeft + ((document.documentElement.offsetWidth - $(element).getWidth()))/2)+'px';
        }
    }




    window.mspReCaptchaConfig = {&quot;siteKey&quot;:&quot;6LfZnTkUAAAAAEaBLYSrH3K13lwf-b3f8Rodhdq8&quot;,&quot;enabled&quot;:false};




            
        Personal Information
        
        
        
            
            

            
                
            
        
            
            

            
                
            
        
    
                                            
                
                
                                                                                        
    
    
        Afghanistan (‫افغانستان‬‎)+93Albania (Shqipëri)+355Algeria (‫الجزائر‬‎)+213American Samoa+1684Andorra+376Angola+244Anguilla+1264Antigua and Barbuda+1268Argentina+54Armenia (Հայաստան)+374Aruba+297Australia+61Austria (Österreich)+43Azerbaijan (Azərbaycan)+994Bahamas+1242Bahrain (‫البحرين‬‎)+973Bangladesh (বাংলাদেশ)+880Barbados+1246Belarus (Беларусь)+375Belgium (België)+32Belize+501Benin (Bénin)+229Bermuda+1441Bhutan (འབྲུག)+975Bolivia+591Bosnia and Herzegovina (Босна и Херцеговина)+387Botswana+267Brazil (Brasil)+55British Indian Ocean Territory+246British Virgin Islands+1284Brunei+673Bulgaria (България)+359Burkina Faso+226Burundi (Uburundi)+257Cambodia (កម្ពុជា)+855Cameroon (Cameroun)+237Canada+1Cape Verde (Kabu Verdi)+238Caribbean Netherlands+599Cayman Islands+1345Central African Republic (République centrafricaine)+236Chad (Tchad)+235Chile+56China (中国)+86Christmas Island+61Cocos (Keeling) Islands+61Colombia+57Comoros (‫جزر القمر‬‎)+269Congo (DRC) (Jamhuri ya Kidemokrasia ya Kongo)+243Congo (Republic) (Congo-Brazzaville)+242Cook Islands+682Costa Rica+506Côte d’Ivoire+225Croatia (Hrvatska)+385Cuba+53Curaçao+599Cyprus (Κύπρος)+357Czech Republic (Česká republika)+420Denmark (Danmark)+45Djibouti+253Dominica+1767Dominican Republic (República Dominicana)+1Ecuador+593Egypt (‫مصر‬‎)+20El Salvador+503Equatorial Guinea (Guinea Ecuatorial)+240Eritrea+291Estonia (Eesti)+372Ethiopia+251Falkland Islands (Islas Malvinas)+500Faroe Islands (Føroyar)+298Fiji+679Finland (Suomi)+358France+33French Guiana (Guyane française)+594French Polynesia (Polynésie française)+689Gabon+241Gambia+220Georgia (საქართველო)+995Germany (Deutschland)+49Ghana (Gaana)+233Gibraltar+350Greece (Ελλάδα)+30Greenland (Kalaallit Nunaat)+299Grenada+1473Guadeloupe+590Guam+1671Guatemala+502Guernsey+44Guinea (Guinée)+224Guinea-Bissau (Guiné Bissau)+245Guyana+592Haiti+509Honduras+504Hong Kong (香港)+852Hungary (Magyarország)+36Iceland (Ísland)+354India (भारत)+91Indonesia+62Iran (‫ایران‬‎)+98Iraq (‫العراق‬‎)+964Ireland+353Isle of Man+44Israel (‫ישראל‬‎)+972Italy (Italia)+39Jamaica+1876Japan (日本)+81Jersey+44Jordan (‫الأردن‬‎)+962Kazakhstan (Казахстан)+7Kenya+254Kiribati+686Kosovo+383Kuwait (‫الكويت‬‎)+965Kyrgyzstan (Кыргызстан)+996Laos (ລາວ)+856Latvia (Latvija)+371Lebanon (‫لبنان‬‎)+961Lesotho+266Liberia+231Libya (‫ليبيا‬‎)+218Liechtenstein+423Lithuania (Lietuva)+370Luxembourg+352Macau (澳門)+853Macedonia (FYROM) (Македонија)+389Madagascar (Madagasikara)+261Malawi+265Malaysia+60Maldives+960Mali+223Malta+356Marshall Islands+692Martinique+596Mauritania (‫موريتانيا‬‎)+222Mauritius (Moris)+230Mayotte+262Mexico (México)+52Micronesia+691Moldova (Republica Moldova)+373Monaco+377Mongolia (Монгол)+976Montenegro (Crna Gora)+382Montserrat+1664Morocco (‫المغرب‬‎)+212Mozambique (Moçambique)+258Myanmar (Burma) (မြန်မာ)+95Namibia (Namibië)+264Nauru+674Nepal (नेपाल)+977Netherlands (Nederland)+31New Caledonia (Nouvelle-Calédonie)+687New Zealand+64Nicaragua+505Niger (Nijar)+227Nigeria+234Niue+683Norfolk Island+672North Korea (조선 민주주의 인민 공화국)+850Northern Mariana Islands+1670Norway (Norge)+47Oman (‫عُمان‬‎)+968Pakistan (‫پاکستان‬‎)+92Palau+680Palestine (‫فلسطين‬‎)+970Panama (Panamá)+507Papua New Guinea+675Paraguay+595Peru (Perú)+51Philippines+63Poland (Polska)+48Portugal+351Puerto Rico+1Qatar (‫قطر‬‎)+974Réunion (La Réunion)+262Romania (România)+40Russia (Россия)+7Rwanda+250Saint Barthélemy (Saint-Barthélemy)+590Saint Helena+290Saint Kitts and Nevis+1869Saint Lucia+1758Saint Martin (Saint-Martin (partie française))+590Saint Pierre and Miquelon (Saint-Pierre-et-Miquelon)+508Saint Vincent and the Grenadines+1784Samoa+685San Marino+378São Tomé and Príncipe (São Tomé e Príncipe)+239Saudi Arabia (‫المملكة العربية السعودية‬‎)+966Senegal (Sénégal)+221Serbia (Србија)+381Seychelles+248Sierra Leone+232Singapore+65Sint Maarten+1721Slovakia (Slovensko)+421Slovenia (Slovenija)+386Solomon Islands+677Somalia (Soomaaliya)+252South Africa+27South Korea (대한민국)+82South Sudan (‫جنوب السودان‬‎)+211Spain (España)+34Sri Lanka (ශ්‍රී ලංකාව)+94Sudan (‫السودان‬‎)+249Suriname+597Svalbard and Jan Mayen+47Swaziland+268Sweden (Sverige)+46Switzerland (Schweiz)+41Syria (‫سوريا‬‎)+963Taiwan (台灣)+886Tajikistan+992Tanzania+255Thailand (ไทย)+66Timor-Leste+670Togo+228Tokelau+690Tonga+676Trinidad and Tobago+1868Tunisia (‫تونس‬‎)+216Turkey (Türkiye)+90Turkmenistan+993Turks and Caicos Islands+1649Tuvalu+688U.S. Virgin Islands+1340Uganda+256Ukraine (Україна)+380United Arab Emirates (‫الإمارات العربية المتحدة‬‎)+971United Kingdom+44United States+1Uruguay+598Uzbekistan (Oʻzbekiston)+998Vanuatu+678Vatican City (Città del Vaticano)+39Venezuela+58Vietnam (Việt Nam)+84Wallis and Futuna+681Western Sahara (‫الصحراء الغربية‬‎)+212Yemen (‫اليمن‬‎)+967Zambia+260Zimbabwe+263Åland Islands+358
            

                        
    

    
        Sign-in Information
        
            
            
                
            
        
        
            
            
                Minimum length of this field must be equal or greater than 8 symbols. Leading and trailing spaces will be ignored.
                
                    
                        Password Strength:
                        Strong
                    
                
            
        
        
            
            
                
            
        
        
        
    Remember Me
    
        What's this?
        
            Check &quot;Remember Me&quot; to access your shopping cart on this computer even if you are not signed in.        
    

                
            
            Sign Up for Newsletter
        
            
    
        
            Create an Account
        
        
            Back
        
    



</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;maincontent&quot;)/div[@class=&quot;columns&quot;]/div[@class=&quot;column main&quot;]/div[@class=&quot;vaimo-cms row&quot;]/div[@class=&quot;col-xs-12 col-sm-12 col-md-12 col-lg-12&quot;]/div[@class=&quot;vcms-standard-output&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <value>//main[@id='maincontent']/div[2]/div/div/div/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Customer support'])[1]/following::div[10]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Vendor Sign Up'])[1]/following::div[10]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <value>//main/div[2]/div/div/div/div</value>
   </webElementXpaths>
</WebElementEntity>
