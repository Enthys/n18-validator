package main

import (
	"math"
)

// RecalculateRefunds fixes the amount of refunded orders and the cost of all refunds
func RecalculateRefunds(audit Audit) Audit {
	var total float64 = 0

	for _, refund := range audit.Return.ReturnedOrders {
		total += refund.ReturnedAmount
	}

	audit.ReturnedOrdersCount = len(audit.Return.ReturnedOrders)
	audit.ReturnedOrdersTotal = (math.Round(total*100) / 100)

	return audit
}

// AssignDocumentNumber assigns the OrderEnum Document number to equal the Order number if the document number is empty
func AssignDocumentNumber(audit Audit) Audit {
	for _, order := range audit.Order.OrderEnums {
		if order.DocumentNumber == "" {
			order.DocumentNumber = order.OrderNumber
		}
	}

	return audit
}
