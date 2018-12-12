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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.masoko.com/')

WebUI.click(findTestObject('Object Repository/addToCart_OR/Page_Shop Online in Kenya - Pay Con/button_Add to Cart'))

WebUI.click(findTestObject('Object Repository/addToCart_OR/Page_Shop Online in Kenya - Pay Con/a_My Cart'))

WebUI.click(findTestObject('Object Repository/addToCart_OR/Page_Shop Online in Kenya - Pay Con/button_Go to Checkout'))

WebUI.setText(findTestObject('Object Repository/addToCart_OR/Page_Shop Online in Kenya - Pay Con/input_Email Address_username'), 
    'mbitiem@yahoo.com')

WebUI.setEncryptedText(findTestObject('Object Repository/addToCart_OR/Page_Shop Online in Kenya - Pay Con/input_Password_password'), 
    '8guqDZyDLVMAP+uswE/FBQ==')

WebUI.click(findTestObject('Object Repository/addToCart_OR/Page_Shop Online in Kenya - Pay Con/span_Sign In'))

