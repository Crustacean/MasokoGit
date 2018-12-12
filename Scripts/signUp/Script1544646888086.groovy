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

WebUI.click(findTestObject('Object Repository/signUp_OR/Page_Shop Online in Kenya - Pay Con/span_Register'))

WebUI.setText(findTestObject('Object Repository/signUp_OR/Page_Create New Customer Account/input_Personal Information_fir'), 
    Fname)

WebUI.setText(findTestObject('Object Repository/signUp_OR/Page_Create New Customer Account/input_Personal Information_las'), 
    Lname)

WebUI.setText(findTestObject('Object Repository/signUp_OR/Page_Create New Customer Account/input_land Islands_mobile'), 
    'Phone')

WebUI.setText(findTestObject('Object Repository/signUp_OR/Page_Create New Customer Account/input_Sign-in Information_emai'), 
    Email)

WebUI.setEncryptedText(findTestObject('Object Repository/signUp_OR/Page_Create New Customer Account/input_Sign-in Information_pass'), 
    Password)

WebUI.setEncryptedText(findTestObject('Object Repository/signUp_OR/Page_Create New Customer Account/input_Strong_password_confirma'), 
    PasswordConf)

WebUI.click(findTestObject('Object Repository/signUp_OR/Page_Create New Customer Account/div_Create New Customer Accoun'))

WebUI.click(findTestObject('Object Repository/signUp_OR/Page_Create New Customer Account/span_Create an Account'))

