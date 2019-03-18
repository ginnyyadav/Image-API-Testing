<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create a new image for the specified user with only metadata. Content can be added later.The body of the request is the UTF-8 JSON encoded Metadata.</description>
   <name>Create Image (Metadata) Webservice Request</name>
   <tag></tag>
   <elementGuidId>2e8b90e3-6786-4f7c-880d-49c3f0bc4d51</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n        \&quot;name\&quot;: \&quot;Max Jpeg Original\&quot;,\n        \&quot;associations\&quot;: [{\n                \&quot;type\&quot;: \&quot;url\&quot;,\n                \&quot;url\&quot;: \&quot;https://stg-app.tidepool.org/\&quot;,\n                \&quot;reason\&quot;: \&quot;Tidepool Image Service Testing\&quot;\n        }],\n        \&quot;location\&quot;: {\n                \&quot;name\&quot;: \&quot;Tidepool Headquarters\&quot;,\n                \&quot;gps\&quot;: {\n                        \&quot;elevation\&quot;: {\n                                \&quot;units\&quot;: \&quot;feet\&quot;,\n                                \&quot;value\&quot;: 500\n                        },\n                        \&quot;floor\&quot;: 55,\n                        \&quot;horizontalAccuracy\&quot;: {\n                                \&quot;units\&quot;: \&quot;feet\&quot;,\n                                \&quot;value\&quot;: 3.25\n                        },\n                        \&quot;latitude\&quot;: {\n                                \&quot;units\&quot;: \&quot;degrees\&quot;,\n                                \&quot;value\&quot;: 37.420499\n                        },\n                        \&quot;longitude\&quot;: {\n                                \&quot;units\&quot;: \&quot;degrees\&quot;,\n                                \&quot;value\&quot;: -122.1156197\n                        },\n                        \&quot;origin\&quot;: {\n                                \&quot;id\&quot;: \&quot;EOFFJUG64JSRNK8\&quot;,\n                                \&quot;name\&quot;: \&quot;tidepool.org\&quot;,\n                                \&quot;time\&quot;: \&quot;2002-10-02T15:00:00Z\&quot;,\n                                \&quot;type\&quot;: \&quot;service\&quot;,\n                                \&quot;version\&quot;: \&quot;3.1.0\&quot;\n                        },\n                        \&quot;verticalAccuracy\&quot;: {\n                                \&quot;units\&quot;: \&quot;meters\&quot;,\n                                \&quot;value\&quot;: 5.1\n                        }\n                }\n        },\n        \&quot;metadata\&quot;: {\n                \&quot;art\&quot;: \&quot;red square\&quot;\n        },\n        \&quot;origin\&quot;: {\n                \&quot;id\&quot;: \&quot;7IDU976HDGHD\&quot;,\n                \&quot;name\&quot;: \&quot;Tidepool\&quot;,\n                \&quot;time\&quot;: \&quot;2002-10-02T15:00:00Z\&quot;,\n                \&quot;type\&quot;: \&quot;device\&quot;,\n                \&quot;version\&quot;: \&quot;1.11.0\&quot;\n        }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
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
   <restUrl>https://stg-api.tidepool.org/v1/users/d4b7e47052/images/metadata</restUrl>
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



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
