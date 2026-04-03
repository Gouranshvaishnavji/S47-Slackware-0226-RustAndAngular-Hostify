import { Component } from '@angular/core';

@Component({
  selector: 'app-binding-demo',
  templateUrl: './binding-demo.component.html',
  styleUrls: ['./binding-demo.component.css']
})
export class BindingDemoComponent {
  // Interpolation
  title = 'Binding Demo Component';

  // Property binding
  isDisabled = false;
  imgUrl = 'https://picsum.photos/150';

  // Event binding
  count = 0;
  increment(): void {
    this.count++;
  }

  // Two-way binding
  username = '';
}
