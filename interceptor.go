package main

import (
	"bytes"
	"encoding/xml"
	"fmt"
	"io/ioutil"
	"os"
)

// Audit root of xml file
type Audit struct {
	XMLName             xml.Name `xml:"audit"`
	Eik                 string   `xml:"eik"`
	EShopNumber         string   `xml:"e_shop_n"`
	EShopType           int8     `xml:"e_shop_type"`
	DomainName          string   `xml:"domain_name"`
	CreationDate        string   `xml:"creation_date"`
	Month               string   `xml:"mon"`
	Year                string   `xml:"god"`
	Order               Order    `xml:"order"`
	Return              Returns  `xml:"rorder"`
	ReturnedOrdersCount int      `xml:"r_ord"`
	ReturnedOrdersTotal float64  `xml:"r_total"`
}

// DocumentNumbers retrieves the DocN field of every OrderEnum
func (audit Audit) DocumentNumbers() []string {
	result := []string{}

	for _, i := range audit.Order.OrderEnums {
		result = append(result, i.DocumentNumber)
	}

	return result
}

// Order root of order documents
type Order struct {
	XMLName    xml.Name    `xml:"order"`
	OrderEnums []OrderEnum `xml:"orderenum"`
}

// OrderEnum order document structure
type OrderEnum struct {
	XMLName        xml.Name `xml:"orderenum"`
	DocumentNumber string   `xml:"doc_n"`
	DocumentDate   string   `xml:"doc_date"`

	OrderNumber     string  `xml:"ord_n"`
	OrderDate       string  `xml:"ord_d"`
	OrderTotalNoVAT float32 `xml:"ord_total1"`
	OrderTotal      float32 `xml:"ord_total2"`
	OrderVATAmount  float32 `xml:"ord_vat"`
	OrderDiscount   string  `xml:"ord_disc"`

	PaymentMethod     int8    `xml:"paym"`
	TransactionNumber string  `xml:"trans_n"`
	POSNumber         string  `xml:"pos_n"`
	PROCId            string  `xml:"proc_id"`
	Articles          Article `xml:"art"`
}

// Article wrapper for ArticleEnums
type Article struct {
	XMLName        xml.Name      `xml:"art"`
	ArticleObjects []ArticleEnum `xml:"artenum"`
}

// ArticleEnum refers to the products in the order
type ArticleEnum struct {
	XMLName   xml.Name `xml:"artenum"`
	Name      string   `xml:"art_name"`
	Quantity  int8     `xml:"art_quant"`
	Price     float32  `xml:"art_price"`
	VATRate   int8     `xml:"art_vat_rate"`
	VATAmount float32  `xml:"art_vat"`
	Total     float32  `xml:"art_sum"`
}

// Returns is the wrapper for the returned orders
type Returns struct {
	XMLName        xml.Name        `xml:"rorder"`
	ReturnedOrders []ReturnedOrder `xml:"rorderenum"`
}

// ReturnedOrder represents a single order for returning a product
type ReturnedOrder struct {
	XMLName        xml.Name `xml:"rorderenum"`
	DocumentNumber string   `xml:"r_ord_n"`
	ReturnedAmount float64  `xml:"r_amount"`
	ReturnDate     string   `xml:"r_date"`
	PaymentMethod  int8     `xml:"r_paym"`
}

// InterceptFiles Find document numbers which occur in both files
func InterceptFiles(mainFile string, interceptionFiles []string) []string {
	latestAuditDocument := generateAuditFromFile(mainFile)
	latestAuditDocumentNumbers := latestAuditDocument.DocumentNumbers()

	result := []string{}

	for _, interceptioninterceptionFile := range interceptionFiles {
		interceptionFileDocumentNumbers := generateAuditFromFile(interceptioninterceptionFile).DocumentNumbers()
		for _, i := range latestAuditDocumentNumbers {
			if InSlice(interceptionFileDocumentNumbers, i) {
				result = append(result, i)
			}
		}
	}

	return result
}

func generateAuditFromFile(file string) Audit {
	xmlFile, err := os.Open(file)

	if err != nil {
		fmt.Println(err)
	}

	defer xmlFile.Close()

	byteValue, _ := ioutil.ReadAll(xmlFile)

	var audit Audit
	xmlObject := bytes.Replace(byteValue, []byte("WINDOWS-1251"), []byte("UTF-8"), 1)

	xml.Unmarshal([]byte(xmlObject), &audit)

	return audit
}

// RemoveInterceptedNumbers Removes the numbers which were found in other documents
func RemoveInterceptedNumbers(auditFile string, documentNumbers []string) Audit {
	audit := generateAuditFromFile(auditFile)

	for _, documentNumber := range documentNumbers {
		for documentIndex, document := range audit.Order.OrderEnums {
			if document.DocumentNumber == documentNumber {
				audit.Order.OrderEnums = append(audit.Order.OrderEnums[:documentIndex], audit.Order.OrderEnums[documentIndex+1:]...)
			}
		}

		for documentIndex, document := range audit.Return.ReturnedOrders {
			if document.DocumentNumber == documentNumber {
				audit.Return.ReturnedOrders = append(audit.Return.ReturnedOrders[:documentIndex], audit.Return.ReturnedOrders[documentIndex+1:]...)
			}
		}
	}

	return audit
}
