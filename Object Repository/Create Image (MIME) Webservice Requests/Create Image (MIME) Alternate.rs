<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Image (MIME) Alternate</name>
   <tag></tag>
   <elementGuidId>c11acc53-45a8-4832-97d8-ab6decaf5cd4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;filePath&quot;: &quot;G:\\My Drive\\QS Issues\\qs0078-Implement-image-service\\JPEG_Oversize_Test 2.jpg&quot;,
  &quot;fileSize&quot;: 0,
  &quot;contentType&quot;: &quot;image/jpeg&quot;
}</httpBodyContent>
   <httpBodyType>file</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>image/jpeg</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkdXIiOjI1OTIwMDAsImV4cCI6MTU1NDU2ODAzOCwic3ZyIjoibm8iLCJ1c3IiOiJkNGI3ZTQ3MDUyIn0.IJhlfTXMrJHh_VLrgqMB0Ly31aZ9vJqXi6pg99ccP9k		</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://stg-api.tidepool.org/v1/users/d4b7e47052/images?contentIntent=alternate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
