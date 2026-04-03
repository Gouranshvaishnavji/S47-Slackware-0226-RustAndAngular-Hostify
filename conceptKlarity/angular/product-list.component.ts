import { Component, OnInit } from '@angular/core';
import { ProductService } from './product.service';
import { Product } from './src/app/models/product.model';

@Component({
  selector: 'app-product-list',
  templateUrl: './product-list.component.html'
})
export class ProductListComponent implements OnInit {
  items: Product[] = [];
  newName = '';
  newPrice = 0;
  newDescription = '';
  loading = false;
  error = '';

  constructor(private svc: ProductService) {}

  ngOnInit(): void {
    this.load();
  }

  load(): void {
    this.loading = true;
    this.svc.getProducts().subscribe({
      next: (data) => { this.items = data; console.log('Typed products:', data); this.loading = false; },
      error: () => { this.error = 'Failed to load items'; this.loading = false; }
    });
  }

  add(): void {
    if (!this.newName) { this.error = 'Name required'; return; }
    this.loading = true;
    this.svc.createProduct({ name: this.newName, price: this.newPrice, description: this.newDescription || undefined }).subscribe({
      next: (item) => { this.items.push(item); this.newName = ''; this.newPrice = 0; this.newDescription = ''; this.loading = false; },
      error: () => { this.error = 'Create failed'; this.loading = false; }
    });
  }
}
