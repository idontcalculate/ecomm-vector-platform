import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

export interface SearchReq { vector: number[]; top_k?: number }
export interface SearchRes { id: number; score: number }

@Injectable({ providedIn: 'root' })
export class SearchService {
  private http = inject(HttpClient);

  search(body: SearchReq): Observable<SearchRes[]> {
    // IMPORTANT: relative path, starts with /search (no host, no /api)
    return this.http.post<SearchRes[]>('/search', body);
  }
}
