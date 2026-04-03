import { Component, OnDestroy } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { AuthService } from './services/auth.service';
import { take } from 'rxjs/operators';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnDestroy {
  username = '';
  password = '';
  error = '';
  loading = false;
  returnUrl = '/dashboard';

  constructor(private auth: AuthService, private router: Router, private route: ActivatedRoute) {
    const rq = this.route.snapshot.queryParamMap.get('returnUrl');
    if (rq) this.returnUrl = rq;
  }
  private sub?: Subscription;

  submit(): void {
    this.loading = true;
    this.error = '';
    this.sub = this.auth.login(this.username, this.password).pipe(take(1)).subscribe(ok => {
      this.loading = false;
      if (ok) {
        this.router.navigateByUrl(this.returnUrl);
      } else {
        this.error = 'Login failed';
      }
    }, () => {
      this.loading = false;
      this.error = 'Login failed';
    });
  }

  ngOnDestroy(): void {
    this.sub?.unsubscribe();
  }
}
