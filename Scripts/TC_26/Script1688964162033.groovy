import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.saucedemo.com/')

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/div_Accepted usernames arestandard_userlock_483a53'))

WebUI.setText(findTestObject('Object Repository/Page_Swag Labs/input_standard_userlocked_out_userproblem_u_db77ac'), 'standard_user')

WebUI.doubleClick(findTestObject('Object Repository/Page_Swag Labs/div_Password for all userssecret_sauce'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/div_Password for all userssecret_sauce'))

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Swag Labs/input_standard_userlocked_out_userproblem_u_3423e9'), 
    'qcu24s4901FyWDTwXGr6XA==')

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/input_standard_userlocked_out_userproblem_u_0dff71'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/button_ADD TO CART'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/a_1'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/div_Description'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/div_Description'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/button_Continue Shopping'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/button_Continue Shopping'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/button_Add to cart_1'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/span_2'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/button_Checkout'))

WebUI.setText(findTestObject('Object Repository/Page_Swag Labs/input_Checkout Your Information_first-name'), 'GG')

WebUI.setText(findTestObject('Object Repository/Page_Swag Labs/input_Checkout Your Information_last-name'), 'GG')

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/input_Checkout Your Information_last-name'))

WebUI.setText(findTestObject('Object Repository/Page_Swag Labs/input_Checkout Your Information_postal-code'), '123456')

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/input_Cancel_continue'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/button_Finish'))

WebUI.click(findTestObject('Object Repository/Page_Swag Labs/button_Back Home'))

WebUI.closeBrowser()

