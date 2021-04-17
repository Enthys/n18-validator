use crate::xml::Audit;

pub fn intercept_audit_document_numbers(audit: &Audit, interception_audit: &Audit) -> Vec<String> {
    let main_audit_document_numbers = audit.document_numbers();
    interception_audit.document_numbers().iter().fold(vec![], |mut result, document_number| {
        if main_audit_document_numbers.contains(document_number) {
            result.push(document_number.to_string());
        }

        result
    })
}

pub fn remove_duplicated_numbers(audit: &mut Audit, documents: Vec<String>) {
    documents
        .into_iter()
        .for_each(|document| {
            if let Some(position) = audit.order.orderenum.iter().position(|order| *order.doc_n.body == document) {
                audit.order.orderenum.remove(position);
            }
            if let Some(position) = audit.rorder.rorderenum.iter().position(|rorder| *rorder.r_ord_n.body == document) {
                audit.rorder.rorderenum.remove(position);
            }
        })
}


