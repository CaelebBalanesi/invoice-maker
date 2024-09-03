import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { FormsModule, NgForm } from '@angular/forms';
import { InvoiceService } from './invoice.service';

interface Bill {
  description: string;
  amount: number;
  extraParagraphs: string[];
}

interface Invoice {
  businessName: string;
  invoicerName: string;
  address: string;
  city: string;
  state: string;
  zip: string;
  phone: string;
  email: string;
  invoiceNumber: string;
  date: string;
  invoiceeName: string;
  bills: Bill[];
}

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, FormsModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {

  title = 'invoice_maker_frontend';
  invoice: Invoice = {
    businessName: '',
    invoicerName: '',
    address: '',
    city: '',
    state: '',
    zip: '',
    phone: '',
    email: '',
    invoiceNumber: '',
    date: '',
    invoiceeName: '',
    bills: [{ description: '', amount: 0, extraParagraphs: [''] }]
  };

  constructor(private invoiceService: InvoiceService) {}

  addBill(): void {
    this.invoice.bills.push({ description: '', amount: 0, extraParagraphs: [''] });
  }

  removeBill(index: number): void {
    this.invoice.bills.splice(index, 1);
  }

  addParagraph(billIndex: number): void {
    this.invoice.bills[billIndex].extraParagraphs.push('');
  }

  removeParagraph(billIndex: number, paragraphIndex: number): void {
    this.invoice.bills[billIndex].extraParagraphs.splice(paragraphIndex, 1);
  }

  trackByIndex(index: number, item: any): any {
    return index;
  }

  onSubmit(form: NgForm): void {
    if (form.valid) {
      const invoiceData = {
        company_name: this.invoice.businessName,
        contact_name: this.invoice.invoicerName,
        address: this.invoice.address,
        city: this.invoice.city,
        state: this.invoice.state,
        zip: this.invoice.zip,
        phone: this.invoice.phone,
        email: this.invoice.email,
        bill_to: this.invoice.invoiceeName,
        invoice_number: this.invoice.invoiceNumber,
        invoice_date: this.invoice.date,
        bills: this.invoice.bills.map(bill => ({
          description: bill.description,
          amount: bill.amount,
          extra_paragraphs: bill.extraParagraphs
        }))
      };

      this.invoiceService.createInvoice(invoiceData).subscribe(response => {
        const contentType = response.headers.get('content-type');
        if (contentType === 'application/pdf' && response.body !== null) {
          const blob = new Blob([response.body], { type: 'application/pdf' });
          const url = window.URL.createObjectURL(blob);
          window.open(url);
        } else {
          console.error('Invalid response content type', contentType);
        }
      }, error => {
        console.error('Error creating invoice', error);
      });
    }
  }
}
