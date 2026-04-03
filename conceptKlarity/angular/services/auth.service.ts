import { Injectable } from '@angular/core';
import { BehaviorSubject, firstValueFrom } from 'rxjs';
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

  async login(username: string, password: string): Promise<boolean> {
    try {
      const resp = await firstValueFrom(this.http.post<LoginResponse>(this.loginUrl, { username, password }));
      if (resp && resp.token) {
        localStorage.setItem('auth_token', resp.token);
        this.loggedInSubject.next(true);
        return true;
      }
      return false;
    } catch (e) {
      console.error('Login failed', e);
      return false;
    }
  }

  logout(): void {
    localStorage.removeItem('auth_token');
    this.loggedInSubject.next(false);
  }

  getToken(): string | null {
    return localStorage.getItem('auth_token');
  }
}
