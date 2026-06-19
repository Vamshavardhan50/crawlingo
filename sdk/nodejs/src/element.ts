import { JsElementCollection, JsElement } from './native.js';

export class Element {
  constructor(private readonly inner: JsElement) {}

  public get text(): string {
    return this.inner.text();
  }

  public get html(): string {
    return this.inner.html();
  }

  public attr(name: string): string | null {
    return this.inner.attr(name);
  }
}

export class ElementCollection implements Iterable<Element> {
  constructor(private readonly inner: JsElementCollection) {}

  public get length(): number {
    return this.inner.length();
  }

  public get text(): string[] {
    return this.inner.text();
  }

  public get html(): string[] {
    return this.inner.html();
  }

  public attr(name: string): (string | null)[] {
    return this.inner.attr(name).map(a => a ?? null);
  }

  public first(): Element | null {
    const raw = this.inner.at(0);
    return raw ? new Element(raw) : null;
  }

  public at(index: number): Element | null {
    const raw = this.inner.at(index);
    return raw ? new Element(raw) : null;
  }

  public *[Symbol.iterator](): Iterator<Element> {
    const len = this.length;
    for (let i = 0; i < len; i++) {
      yield this.at(i)!;
    }
  }
}
