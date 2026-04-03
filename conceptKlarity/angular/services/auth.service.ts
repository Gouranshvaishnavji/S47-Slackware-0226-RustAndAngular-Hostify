import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable, of } from 'rxjs';
import { map, tap, catchError } from 'rxjs/operators';
import { HttpClient } from '@angular/common/http';

interface LoginResponse {
  token: string;
}

@Injectable({ providedIn: 'root' })
export class AuthService {
  private readonly loggedInSubject = new BehaviorSubject<boolean>(false);
  readonly isLoggedIn$ = this.loggedInSubject.asObservable();
  private readonly loginUrl = '/api/auth/login';

  constructor(private http: HttpClient) {
    const token = localStorage.getItem('auth_token');
    this.loggedInSubject.next(!!token);
  }

  isLoggedIn(): boolean {
    return this.loggedInSubject.getValue();
  }

  // Return an Observable<boolean> instead of a Promise so consumers use Observables
  login(username: string, password: string): Observable<boolean> {
    return this.http.post<LoginResponse>(this.loginUrl, { username, password }).pipe(
      tap(resp => {
        if (resp && resp.token) {
          localStorage.setItem('auth_token', resp.token);
          this.loggedInSubject.next(true);
        }
      }),
      map(resp => !!resp && !!resp.token),
      catchError(err => {
        console.error('Login failed', err);
        return of(false);
      })
    );
  }

  logout(): void {
    localStorage.removeItem('auth_token');
    this.loggedInSubject.next(false);
  }

  getToken(): string | null {
    return localStorage.getItem('auth_token');
  }
}
