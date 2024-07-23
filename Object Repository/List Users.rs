<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>List Users</name>
   <tag></tag>
   <elementGuidId>6bbacaab-1dbe-473c-bafe-9f1ada082b54</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

import com.kms.katalon.core.testobject.ConditionType

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

// Start time of API request
def startTime = System.currentTimeMillis()

// End time of API request
def endTime = System.currentTimeMillis()

// Calculate response time in milliseconds
def responseTime = endTime - startTime

// Define expected response time threshold (in milliseconds)
def expectedResponseTime = 10

// Assert that the response time is within the expected threshold
assert responseTime &lt;= expectedResponseTime : &quot;Response time (${responseTime} ms) exceeded expected threshold (${expectedResponseTime} ms)&quot;

def jsonSlurper = new groovy.json.JsonSlurper()
def jsonResponse = jsonSlurper.parseText(response.getResponseText())

assert jsonResponse.page == 2
assert jsonResponse.per_page == 6
assert jsonResponse.total == 12
assert jsonResponse.total_pages == 2

// Example verification for the first user
def firstUser = jsonResponse.data[0]
assert firstUser.id == 7
assert firstUser.email == &quot;michael.lawson@reqres.in&quot;
assert firstUser.first_name == &quot;Michael&quot;
assert firstUser.last_name == &quot;Lawson&quot;
assert firstUser.avatar == &quot;https://reqres.in/img/faces/7-image.jpg&quot;</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
