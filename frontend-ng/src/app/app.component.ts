import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { FormsModule } from '@angular/forms';
import { NgIf, NgFor } from '@angular/common';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [FormsModule, NgIf, NgFor],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  query = '0.1,0.2,0.3,0.4';
  loading = false;
  error = '';
  results: Array<{ id: number; score: number }> = [];

  constructor(private http: HttpClient) {}

  search() {
    this.loading = true;
    this.error = '';
    this.results = [];
    const vector = this.query.split(',').map(v => Number(v.trim())).filter(v => !Number.isNaN(v));
    this.http.post<Array<{ id: number; score: number }>>('/search', { vector })
      .subscribe({
        next: data => { this.results = data; this.loading = false; },
        error: err => { this.error = String(err?.message || err); this.loading = false; }
      });
  }
}
