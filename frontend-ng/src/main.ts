import 'zone.js';
import { enableProdMode } from '@angular/core';
import { bootstrapApplication } from '@angular/platform-browser';
import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  template: `
    <h1>E-commerce Vector Search</h1>

    <input [value]="query" (input)="query = ($any($event.target)).value" placeholder="0.1,0.2,0.3,0.4"/>
    <button (click)="search()" [disabled]="loading">{{ loading ? 'Searching…' : 'Search' }}</button>

    <p *ngIf="error" style="color:crimson">{{ error }}</p>
    <pre *ngIf="results?.length">{{ results | json }}</pre>
  `
})
class RootComponent {
  query = '0.1,0.2,0.3,0.4';
  loading = false;
  error = '';
  results: any[] = [];

  async search() {
    this.loading = true;
    this.error = '';
    this.results = [];
    try {
      const vec = this.query.split(',').map(s => Number(s.trim())).filter(n => !Number.isNaN(n));
      const res = await fetch('/search', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ vector: vec })
      });
      if (!res.ok) throw new Error(await res.text());
      this.results = await res.json();
    } catch (e: any) {
      this.error = e?.message ?? String(e);
    } finally {
      this.loading = false;
    }
  }
}

if (process.env['NODE_ENV'] === 'production') enableProdMode();
bootstrapApplication(RootComponent);
