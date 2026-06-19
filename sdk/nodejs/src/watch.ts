import { JsWatch } from './native.js';
import { Session } from './session';

export interface WatchChangeEvent {
  url: string;
  field: string;
  changeType: string;
  oldValue?: string;
  newValue?: string;
}

export class Watch {
  private readonly inner: JsWatch;

  constructor(url: string, session?: Session) {
    const activeSession = session ?? new Session();
    this.inner = new JsWatch(url, activeSession.inner);
  }

  public field(name: string, selector: string): this {
    this.inner.field(name, selector);
    return this;
  }

  public interval(seconds: number): this {
    this.inner.interval(seconds);
    return this;
  }

  public run(callback: (err: Error | null, event: WatchChangeEvent) => void): void {
    this.inner.run((err, evt) => {
      if (err) {
        callback(err, null as any);
        return;
      }
      callback(null, {
        url: evt.url,
        field: evt.field,
        changeType: evt.changeType ?? (evt as any).change_type,
        oldValue: evt.oldValue ?? (evt as any).old_value ?? undefined,
        newValue: evt.newValue ?? (evt as any).new_value ?? undefined,
      });
    });
  }

  public stop(): void {
    this.inner.stop();
  }
}
