import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WS.comment('Given')

resp0 = WS.sendRequestAndVerify(findTestObject('Login to STG'))

WS.comment('IF requesting "available" image status AND "original" contentIntent WITH page 0 and pagination at 1,000')

resp1 = WS.sendRequest(findTestObject('List Images Webservice Requests/List Images'))

WS.comment('THEN')

WS.verifyResponseStatusCode(resp1, 200)

WS.comment('IF requesting "created" image status AND "original" contentIntent WITH page 0 and pagination at 1,000 THEN a code 200 success message should be received indicating the successfu pull of those images')

resp2 = WS.sendRequest(findTestObject('List Images Webservice Requests/List Images', [('status') : 'created', ('contentIntent') : 'original'
            , ('page') : 0, ('size') : 1000]))

WS.comment('THEN')

WS.verifyResponseStatusCode(resp2, 200)

WS.comment('IF requesting "available" image status AND "alternate" contentIntent WITH page 0 and pagination at 1,000 THEN a code 200 success message should be received indicating the successfu pull of those images')

resp3 = WS.sendRequest(findTestObject('List Images Webservice Requests/List Images', [('status') : 'available', ('contentIntent') : 'alternate'
            , ('page') : 0, ('size') : 1000]))

WS.comment('THEN')

WS.verifyResponseStatusCode(resp3, 200)

WS.comment('IF requesting "available" image status AND "original" contentIntent WITH page 0 and pagination at 1,000 THEN a code 200 success message should be received indicating the successfu pull of those images')

resp4 = WS.sendRequest(findTestObject('List Images Webservice Requests/List Images', [('status') : 'available', ('contentIntent') : 'original'
            , ('page') : 0, ('size') : 1000]))

WS.comment('THEN')

WS.verifyResponseStatusCode(resp4, 200)

