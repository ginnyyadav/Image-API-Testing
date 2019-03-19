<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>List Images</name>
   <tag></tag>
   <elementGuidId>46a6cf0e-5382-4896-94d4-b5fbf6e2d058</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;filePath&quot;: &quot;G:\\My Drive\\QS Issues\\qs0078-Implement-image-service\\JPEG_Oversize_Test 2.jpg&quot;,
  &quot;fileSize&quot;: 5037358,
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
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://stg-api.tidepool.org/v1/users/d4b7e47052/images?status=${status}&amp;contentIntent=${contentIntent}&amp;page=${page}&amp;size=${size}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'available'</defaultValue>
      <description>can be available or created</description>
      <id>25fd9b8c-6d2b-492a-a4c2-76a91953e8af</id>
      <masked>false</masked>
      <name>status</name>
   </variables>
   <variables>
      <defaultValue>'original'</defaultValue>
      <description>can be original or alternate</description>
      <id>b24e770b-4478-4c31-b833-91269b050294</id>
      <masked>false</masked>
      <name>contentIntent</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>33e57caf-33a2-4fd0-8153-b752fc028960</id>
      <masked>false</masked>
      <name>page</name>
   </variables>
   <variables>
      <defaultValue>1000</defaultValue>
      <description></description>
      <id>97f37427-9e50-411e-a3aa-60e1d6a84fea</id>
      <masked>false</masked>
      <name>size</name>
   </variables>
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
