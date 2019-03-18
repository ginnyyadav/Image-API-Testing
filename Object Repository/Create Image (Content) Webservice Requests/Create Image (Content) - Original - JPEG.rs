<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create a new image for the specified user with only content using the specified content intent.</description>
   <name>Create Image (Content) - Original - JPEG</name>
   <tag></tag>
   <elementGuidId>67cc9935-bc96-4f11-ba47-9047c8f678b5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;filePath&quot;: &quot;G:\\My Drive\\QS Issues\\qs0078-Implement-image-service\\Max_JPEG_Test.jpg&quot;,
  &quot;fileSize&quot;: 1749440,
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
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkdXIiOjI1OTIwMDAsImV4cCI6MTU1NDU2ODAzOCwic3ZyIjoibm8iLCJ1c3IiOiJkNGI3ZTQ3MDUyIn0.IJhlfTXMrJHh_VLrgqMB0Ly31aZ9vJqXi6pg99ccP9k</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://stg-api.tidepool.org/v1/users/d4b7e47052/images/content/original</restUrl>
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
