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

WebUI.navigateToUrl('https://magento.softwaretestingboard.com/')

WebUI.click(findTestObject('Object Repository/Page_Home Page/div_S'))

WebUI.click(findTestObject('Object Repository/Page_Home Page/div_XL_option-label-color-93-item-53'))

WebUI.click(findTestObject('Object Repository/Page_Home Page/span_Add to Cart'))

WebUI.click(findTestObject('Object Repository/Page_Home Page/span_1'))

WebUI.click(findTestObject('Object Repository/Page_Home Page/span_See Details'))

WebUI.click(findTestObject('Object Repository/Page_Home Page/span_View and Edit Cart'))

WebUI.click(findTestObject('Object Repository/Page_Shopping Cart/span_Update Shopping Cart'))

WebUI.click(findTestObject('Object Repository/Page_Shopping Cart/div_Apply Discount Code'))

WebUI.setText(findTestObject('Object Repository/Page_Shopping Cart/input_Enter discount code_coupon_code'), 'HADD')

WebUI.click(findTestObject('Object Repository/Page_Shopping Cart/span_Apply Discount'))

WebUI.click(findTestObject('Object Repository/Page_Shopping Cart/span_Proceed to Checkout'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/span_Next'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/input_Carrier Title_ko_unique_1'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/button_Next'))

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Email Address_username'), 'GG')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_First Name_firstname'), 'GG')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Last Name_lastname'), 'HH')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Street Address Line 1_street0'), 'YY')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_City_city'), 'UU')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_ZipPostal Code_postcode'), '123456')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Phone Number_telephone'), '9999999999')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_ZipPostal Code_postcode'), '123456')

WebUI.click(findTestObject('Object Repository/Page_Checkout/input_ZipPostal Code_postcode'))

WebUI.doubleClick(findTestObject('Object Repository/Page_Checkout/input_ZipPostal Code_postcode'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/div_Email Address                          _f5ea29'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/button_Next'))

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Email Address_username'), 'GG@GMAIL.COM')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_First Name_firstname'), 'GG')

WebUI.click(findTestObject('Object Repository/Page_Checkout/span_Next'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/span_Next'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/span_Place Order'))

