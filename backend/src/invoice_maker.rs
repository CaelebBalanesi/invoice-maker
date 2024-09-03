use std::{fs::File, io::Write, process::Command};
use horrorshow::{helper::doctype, html};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Invoice {
    pub company_name: String,
    pub contact_name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub phone: String,
    pub email: String,
    pub bill_to: String,
    pub invoice_number: String,
    pub invoice_date: String,
    pub bills: Vec<Bill>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Bill {
    pub description: String,
    pub amount: i32,
    pub extra_paragraphs: Option<Vec<String>>,
}

pub fn create_invoice(invoice: Invoice) -> Result<()> {
    let invoice_html = format!("{}", html! {
        : doctype::HTML;
        html {
            head {
                style {
                    : r#"
                        * {
                            font-family: Helvetica;
                        }
                        
                        body {
                            padding: 50px;
                        }
        
                        .title {
                            color: rgb(50, 50, 200);
                            font-size: 70px;
                            margin-top: 10px;
                        }
        
                        .contact p {
                            margin: 2px;
                        }
        
                        .bill-info {
                            display: flex;
                            justify-content: space-between;
                        }
        
                        .invoice-data {
                            display: flex;
                            justify-content: space-between;
                        }
        
                        .invoice-data h2 {
                            margin-right: 10px;
                        }
        
                        .invoice-data-entry {
                            justify-content: center;
                            line-height: 35px;
                        }
        
                        .bills-header {
                            display: flex;
                            justify-content: space-between;
                        }
        
                        .bill {
                            display: flex;
                            justify-content: space-between;
                            flex-direction: column;
                        }

                        .bill-header {
                            display: flex;
                            justify-content: space-between;
                        }

                        .extra-info {
                            padding-top: 2px;
                            margin-top: 6px;
                            margin-left: 24px;
                            font-size: 16px;
                            color: rgb(100, 100, 100);
                        }
                    "#;
                }
            }
            body {
                h1(class="title") { : "INVOICE" }
                div(class="contact") {
                    h3 { : invoice.clone().company_name }
                    p { : invoice.clone().contact_name }
                    p { : invoice.clone().address }
                    p { : format!("{}, {} {}", invoice.clone().city, invoice.clone().state, invoice.clone().zip) }
                    p { : invoice.clone().phone }
                    p { : invoice.clone().email }
                }
                div(class="bill-info") {
                    div(class="bill-to") {
                        h2 { : "BILL TO" }
                        p { : invoice.clone().bill_to }
                    }
                    div {
                        div(class="invoice-data") {
                            h2 { : "INVOICE #" }
                            p(class="invoice-data-entry") { : invoice.clone().invoice_number }
                        }
                        div(class="invoice-data") {
                            h2 { : "INVOICE DATE" }
                            p(class="invoice-data-entry") { : invoice.clone().invoice_date }
                        }
                    }
                }
                div(class="bills") {
                    div(class="bills-header") {
                        h2 { : "DESCRIPTION" }
                        h2 { : "AMOUNT" }
                    }
                    @ for i in 0..invoice.bills.len() {
                        div(class="bill") {
                            div(class="bill-header") {
                                p { : &invoice.bills[i].description }
                                p { : String::from("$") + &invoice.bills[i].amount.to_string() }
                            }
                            @ if let Some(extra_paragraphs) = &invoice.bills[i].extra_paragraphs {
                                ul(class="extra-info") {
                                    @ for paragraph in extra_paragraphs {
                                        @ if !paragraph.is_empty() {
                                            li { : paragraph }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div(class="total") {
                    h2 { : String::from("TOTAL $") + &invoice.bills.iter().map(|x| x.amount).sum::<i32>().to_string() }
                }
            }
        }
    });

    let mut html_file = File::create("foo.html")?;
    html_file.write_all(invoice_html.as_bytes())?;

    let output = Command::new("html2pdf")
        .args(&[
            "--margin", "0.4",
            "--output", "output.pdf",
            "foo.html"
        ])
        .output()?;

    if output.status.success() {
        println!("PDF created successfully.");
        Ok(())
    } else {
        println!("Error: {:?}", output);
        Err(anyhow::anyhow!("Failed to create PDF"))
    }
}
