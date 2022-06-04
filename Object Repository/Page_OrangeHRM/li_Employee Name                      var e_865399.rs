<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>li_Employee Name                      var e_865399</name>
   <tag></tag>
   <elementGuidId>6cad458d-8f01-4bcb-86a1-2621ecaa351f</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//form[@id='search_form']/fieldset/ol/li</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>ol > li</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>li</value>
      <webElementGuid>ee89bc83-5239-4b21-98d0-42d83caa16bf</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>Employee Name
  





        

            var employees_empsearch_employee_name = [];

            $(document).ready(function() {
            
                var nameField = $(&quot;#empsearch_employee_name_empName&quot;);
                var idStoreField = $(&quot;#empsearch_employee_name_empId&quot;);
                var typeHint = 'Type for hints...';
                var hintClass = 'inputFormatHint';
                var loadingMethod = 'ajax';
                var loadingHint = 'Loading';
            
                if (idStoreField.val() != '') {
                    idStoreField.data('item.name', nameField.val());
                }
                
                nameField.data('typeHint', typeHint);
                nameField.data('loadingHint', loadingHint);
                
                nameField.one('focus', function() {

                        if ($(this).hasClass(hintClass)) {
                            $(this).val(&quot;&quot;);
                            $(this).removeClass(hintClass);
                        }

                    });
                    
                if( loadingMethod != 'ajax'){
                    if (nameField.val() == '' || nameField.val() == typeHint) {
                        nameField.val(typeHint).addClass(hintClass);
                    }

                    

                    nameField.autocomplete(employees_empsearch_employee_name, {

                        formatItem: function(item) {
                            return $('&lt;div/>').text(item.name).html();
                        },
                        formatResult: function(item) {
                            return item.name
                        }
                      ,matchContains:true
                        }).result(function(event, item) {
                            idStoreField.val(item.id);
                            idStoreField.data('item.name', item.name);
                        }

                    );
                 }else{
                        var value = nameField.val().trim();
                        nameField.val(loadingHint).addClass('ac_loading');
                        $.ajax({
                               url: &quot;/index.php/pim/getEmployeeListAjax&quot;,
                               data: '',
                               dataType: 'json',
                               success: function(employeeList){

                                     nameField.autocomplete(employeeList, {

                                                formatItem: function(item) {
                                                    return $('&lt;div/>').text(item.name).html();
                                                },
                                                formatResult: function(item) {
                                                    return item.name
                                                }
                                                
                                                ,matchContains:true
                                            }).result(function(event, item) {
                                                idStoreField.val(item.id);
                                                idStoreField.data('item.name', item.name);
                                            }

                                        );
                                         nameField.removeClass('ac_loading'); 
                                        
                                         if(value==''){
                                            nameField.val(typeHint).addClass(hintClass);
                                         } else {
                                            nameField.val(value).addClass();
                                         }
                                    }
                             });
                 }
                
            }); // End of $(document).ready

                 
        


</value>
      <webElementGuid>91c947ff-2527-46ec-82b0-028f36c8cfb0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;search_form&quot;)/fieldset[1]/ol[1]/li[1]</value>
      <webElementGuid>8c1f55cd-f477-449d-856a-376236026d50</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='search_form']/fieldset/ol/li</value>
      <webElementGuid>4e1cdc3a-43c4-4ac8-a759-0e33c026ab00</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Employee Information'])[1]/following::li[1]</value>
      <webElementGuid>cacf5330-13e7-491d-a5b6-a324562bda9d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Buzz'])[1]/following::li[2]</value>
      <webElementGuid>28181867-cabb-45e3-baf5-73dc76f153b8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Id'])[1]/preceding::li[1]</value>
      <webElementGuid>5ba5312d-0c47-437c-a06b-05b9c1762435</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//ol/li</value>
      <webElementGuid>b87e7294-4bc8-474f-8917-e32d6bbf8095</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//li[(text() = concat(&quot;Employee Name
  





        

            var employees_empsearch_employee_name = [];

            $(document).ready(function() {
            
                var nameField = $(&quot;#empsearch_employee_name_empName&quot;);
                var idStoreField = $(&quot;#empsearch_employee_name_empId&quot;);
                var typeHint = &quot; , &quot;'&quot; , &quot;Type for hints...&quot; , &quot;'&quot; , &quot;;
                var hintClass = &quot; , &quot;'&quot; , &quot;inputFormatHint&quot; , &quot;'&quot; , &quot;;
                var loadingMethod = &quot; , &quot;'&quot; , &quot;ajax&quot; , &quot;'&quot; , &quot;;
                var loadingHint = &quot; , &quot;'&quot; , &quot;Loading&quot; , &quot;'&quot; , &quot;;
            
                if (idStoreField.val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, nameField.val());
                }
                
                nameField.data(&quot; , &quot;'&quot; , &quot;typeHint&quot; , &quot;'&quot; , &quot;, typeHint);
                nameField.data(&quot; , &quot;'&quot; , &quot;loadingHint&quot; , &quot;'&quot; , &quot;, loadingHint);
                
                nameField.one(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {

                        if ($(this).hasClass(hintClass)) {
                            $(this).val(&quot;&quot;);
                            $(this).removeClass(hintClass);
                        }

                    });
                    
                if( loadingMethod != &quot; , &quot;'&quot; , &quot;ajax&quot; , &quot;'&quot; , &quot;){
                    if (nameField.val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; || nameField.val() == typeHint) {
                        nameField.val(typeHint).addClass(hintClass);
                    }

                    

                    nameField.autocomplete(employees_empsearch_employee_name, {

                        formatItem: function(item) {
                            return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
                        },
                        formatResult: function(item) {
                            return item.name
                        }
                      ,matchContains:true
                        }).result(function(event, item) {
                            idStoreField.val(item.id);
                            idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, item.name);
                        }

                    );
                 }else{
                        var value = nameField.val().trim();
                        nameField.val(loadingHint).addClass(&quot; , &quot;'&quot; , &quot;ac_loading&quot; , &quot;'&quot; , &quot;);
                        $.ajax({
                               url: &quot;/index.php/pim/getEmployeeListAjax&quot;,
                               data: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                               dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
                               success: function(employeeList){

                                     nameField.autocomplete(employeeList, {

                                                formatItem: function(item) {
                                                    return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
                                                },
                                                formatResult: function(item) {
                                                    return item.name
                                                }
                                                
                                                ,matchContains:true
                                            }).result(function(event, item) {
                                                idStoreField.val(item.id);
                                                idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, item.name);
                                            }

                                        );
                                         nameField.removeClass(&quot; , &quot;'&quot; , &quot;ac_loading&quot; , &quot;'&quot; , &quot;); 
                                        
                                         if(value==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                                            nameField.val(typeHint).addClass(hintClass);
                                         } else {
                                            nameField.val(value).addClass();
                                         }
                                    }
                             });
                 }
                
            }); // End of $(document).ready

                 
        


&quot;) or . = concat(&quot;Employee Name
  





        

            var employees_empsearch_employee_name = [];

            $(document).ready(function() {
            
                var nameField = $(&quot;#empsearch_employee_name_empName&quot;);
                var idStoreField = $(&quot;#empsearch_employee_name_empId&quot;);
                var typeHint = &quot; , &quot;'&quot; , &quot;Type for hints...&quot; , &quot;'&quot; , &quot;;
                var hintClass = &quot; , &quot;'&quot; , &quot;inputFormatHint&quot; , &quot;'&quot; , &quot;;
                var loadingMethod = &quot; , &quot;'&quot; , &quot;ajax&quot; , &quot;'&quot; , &quot;;
                var loadingHint = &quot; , &quot;'&quot; , &quot;Loading&quot; , &quot;'&quot; , &quot;;
            
                if (idStoreField.val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, nameField.val());
                }
                
                nameField.data(&quot; , &quot;'&quot; , &quot;typeHint&quot; , &quot;'&quot; , &quot;, typeHint);
                nameField.data(&quot; , &quot;'&quot; , &quot;loadingHint&quot; , &quot;'&quot; , &quot;, loadingHint);
                
                nameField.one(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {

                        if ($(this).hasClass(hintClass)) {
                            $(this).val(&quot;&quot;);
                            $(this).removeClass(hintClass);
                        }

                    });
                    
                if( loadingMethod != &quot; , &quot;'&quot; , &quot;ajax&quot; , &quot;'&quot; , &quot;){
                    if (nameField.val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; || nameField.val() == typeHint) {
                        nameField.val(typeHint).addClass(hintClass);
                    }

                    

                    nameField.autocomplete(employees_empsearch_employee_name, {

                        formatItem: function(item) {
                            return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
                        },
                        formatResult: function(item) {
                            return item.name
                        }
                      ,matchContains:true
                        }).result(function(event, item) {
                            idStoreField.val(item.id);
                            idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, item.name);
                        }

                    );
                 }else{
                        var value = nameField.val().trim();
                        nameField.val(loadingHint).addClass(&quot; , &quot;'&quot; , &quot;ac_loading&quot; , &quot;'&quot; , &quot;);
                        $.ajax({
                               url: &quot;/index.php/pim/getEmployeeListAjax&quot;,
                               data: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                               dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
                               success: function(employeeList){

                                     nameField.autocomplete(employeeList, {

                                                formatItem: function(item) {
                                                    return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
                                                },
                                                formatResult: function(item) {
                                                    return item.name
                                                }
                                                
                                                ,matchContains:true
                                            }).result(function(event, item) {
                                                idStoreField.val(item.id);
                                                idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, item.name);
                                            }

                                        );
                                         nameField.removeClass(&quot; , &quot;'&quot; , &quot;ac_loading&quot; , &quot;'&quot; , &quot;); 
                                        
                                         if(value==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                                            nameField.val(typeHint).addClass(hintClass);
                                         } else {
                                            nameField.val(value).addClass();
                                         }
                                    }
                             });
                 }
                
            }); // End of $(document).ready

                 
        


&quot;))]</value>
      <webElementGuid>99ee7cb1-ee2a-4613-ac8f-b30f6a2f186b</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
