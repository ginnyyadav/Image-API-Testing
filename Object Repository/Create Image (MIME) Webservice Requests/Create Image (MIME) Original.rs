<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Image (MIME) Original</name>
   <tag></tag>
   <elementGuidId>80c7121d-d10d-4870-b967-6c3ebf09ea71</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;metadata&quot;,
      &quot;value&quot;: &quot;G:\\My Drive\\QS Issues\\qs0078-Implement-image-service\\metadata1.json&quot;,
      &quot;type&quot;: &quot;File&quot;
    },
    {
      &quot;name&quot;: &quot;contentIntent&quot;,
      &quot;value&quot;: &quot;G:\\My Drive\\QS Issues\\qs0078-Implement-image-service\\TextFile2.txt&quot;,
      &quot;type&quot;: &quot;File&quot;
    },
    {
      &quot;name&quot;: &quot;content&quot;,
      &quot;value&quot;: &quot;G:\\My Drive\\QS Issues\\qs0078-Implement-image-service\\Max_JPEG_Test.jpg&quot;,
      &quot;type&quot;: &quot;File&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
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
   <restUrl>https://stg-api.tidepool.org/v1/users/d4b7e47052/images</restUrl>
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
