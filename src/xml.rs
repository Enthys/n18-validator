use std::fs;
use quick_xml::de::{from_str};
use quick_xml::se::to_string;
use rust_decimal::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "audit")]
pub struct Audit {
    pub eik: XMLValue<String>,
    pub e_shop_n: XMLValue<String>,
    pub e_shop_type: XMLValue<u16>,
    pub domain_name: XMLValue<String>,
    pub creation_date: XMLValue<String>,
    pub mon: XMLValue<String>,
    pub god: XMLValue<String>,
    pub order: Order,
    pub rorder: Returns,

    pub r_ord: XMLValue<i32>,
    pub r_total: XMLValue<String>,
}

impl Audit {
    pub fn document_numbers(&self) -> Vec<&String> {
        self.order.orderenum.iter().map(|order| &order.doc_n.body).collect::<Vec<&String>>()
    }

    pub fn recalculate_returns(&mut self) {
        let mut result_total = Decimal::new(0, 2);
        self.rorder.rorderenum.iter().for_each(|order| {
            result_total += Decimal::from_str(order.r_amount.body.as_str()).unwrap();
        });

        self.r_total.body = result_total.to_string();
        self.r_ord.body = self.rorder.rorderenum.len() as i32;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub orderenum: Vec<OrderEnum>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderEnum {
    pub doc_n: XMLValue<String>,
    pub doc_date: XMLValue<String>,

    pub ord_n: XMLValue<String>,
    pub ord_d: XMLValue<String>,
    pub ord_total1: XMLValue<f32>,
    pub ord_total2: XMLValue<f32>,
    pub ord_vat: XMLValue<f32>,
    pub ord_disc: XMLValue<String>,

    pub paym: XMLValue<u16>,
    pub trans_n: XMLValue<String>,
    pub pos_n: XMLValue<String>,
    pub proc_id: XMLValue<String>,
    pub art: Article,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub artenum: Vec<ArticleEnum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleEnum {
    pub art_name: XMLValue<String>,
    pub art_quant: XMLValue<u16>,
    pub art_price: XMLValue<f32>,
    pub art_vat_rate: XMLValue<u16>,
    pub art_vat: XMLValue<f32>,
    pub art_sum: XMLValue<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Returns {
    pub rorderenum: Vec<ReturnOrder>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnOrder {
    pub r_ord_n: XMLValue<String>,
    pub r_amount: XMLValue<String>,
    pub r_date: XMLValue<String>,
    pub r_paym: XMLValue<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XMLValue<T> {
    #[serde(rename = "$value", default)]
    pub body: T
}

pub fn audit_from_xml_file(file: String) -> Audit {
    println!("Reading file: {:?}", file);

    let xml_content = fs::read_to_string(&file)
        .expect(format!("Failed to load file: {}", &file).as_str());

    from_str(&xml_content).unwrap()
}

pub fn audit_to_xml(audit: Audit) -> String {
    to_string(&audit).unwrap()
}

pub fn generate_xml_header(version: String, encoding: String) -> String {
    format!("<?xml version=\"{}\" encoding=\"{}\"?>", version, encoding)
}
