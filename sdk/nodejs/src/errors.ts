/** Optional diagnostic context attached to SDK errors. */
export interface CrawlingoErrorOptions extends ErrorOptions {
  code?: string;
  url?: string;
  stage?: string;
  recoverable?: boolean;
  suggestion?: string;
  seconds?: number;
  host?: string;
}

/** Base class for failures reported by the Crawlingo engine. */
export class CrawlingoError extends Error {
  static readonly defaultCode: string = 'GENERIC_FAILURE';
  readonly code: string;
  readonly url?: string;
  readonly stage?: string;
  readonly recoverable: boolean;
  readonly suggestion?: string;
  readonly seconds?: number;
  readonly host?: string;

  constructor(message = 'Crawlingo operation failed', options: CrawlingoErrorOptions = {}) {
    super(message, { cause: options.cause });
    this.name = new.target.name;
    this.code = options.code ?? (new.target as typeof CrawlingoError).defaultCode;
    this.url = options.url;
    this.stage = options.stage;
    this.recoverable = options.recoverable ?? false;
    this.suggestion = options.suggestion;
    this.seconds = options.seconds;
    this.host = options.host;
  }
}

export class FetchError extends CrawlingoError { static readonly defaultCode = 'FETCH_FAILED'; }
export class ParseError extends CrawlingoError { static readonly defaultCode = 'INVALID_HTML'; }
export class SelectorError extends CrawlingoError { static readonly defaultCode = 'SELECTOR_FAILED'; }
export class AutoMatchFailed extends CrawlingoError { static readonly defaultCode = 'AUTO_MATCH_FAILED'; }
export class TimeoutError extends CrawlingoError { static readonly defaultCode = 'TIMEOUT'; }
export class RateLimitError extends CrawlingoError { static readonly defaultCode = 'RATE_LIMIT'; }
export class ChangeDetectionError extends CrawlingoError { static readonly defaultCode = 'CHANGE_DETECTION_FAILED'; }
export class ExportError extends CrawlingoError { static readonly defaultCode = 'EXPORT_FAILED'; }
export class DnsError extends CrawlingoError { static readonly defaultCode = 'DNS_FAILED'; }
export class FingerprintStoreError extends CrawlingoError { static readonly defaultCode = 'FINGERPRINT_STORE_FAILED'; }

type ErrorClass = typeof CrawlingoError;
const classes = new Map<string, ErrorClass>([
  CrawlingoError, FetchError, ParseError, SelectorError, AutoMatchFailed, TimeoutError,
  RateLimitError, ChangeDetectionError, ExportError, DnsError, FingerprintStoreError,
].map(cls => [cls.name, cls]));
const codes = new Map<string, ErrorClass>([...classes.values()].map(cls => [cls.defaultCode, cls]));

/** Internal FFI boundary: turn the native diagnostic envelope into a real Error. */
export function toCrawlingoError(error: unknown, fallback: ErrorClass = CrawlingoError): CrawlingoError {
  if (error instanceof CrawlingoError) return error;
  const message = error instanceof Error ? error.message : String(error);
  let details: Record<string, unknown> | undefined;
  try {
    const parsed: unknown = JSON.parse(message);
    if (parsed && typeof parsed === 'object' && 'success' in parsed && parsed.success === false) {
      details = parsed as Record<string, unknown>;
    }
  } catch { /* Plain native errors use the operation's fallback class. */ }
  if (!details) return new fallback(message, { cause: error });
  const ErrorType = classes.get(String(details.error_type)) ?? codes.get(String(details.error_code)) ?? fallback;
  return new ErrorType(typeof details.message === 'string' ? details.message : message, {
    cause: error,
    code: typeof details.error_code === 'string' ? details.error_code : undefined,
    url: typeof details.url === 'string' ? details.url : undefined,
    stage: typeof details.stage === 'string' ? details.stage : undefined,
    recoverable: details.recoverable === true,
    suggestion: typeof details.suggestion === 'string' ? details.suggestion : undefined,
    seconds: typeof details.seconds === 'number' ? details.seconds : undefined,
    host: typeof details.host === 'string' ? details.host : undefined,
  });
}

export function callNative<T>(operation: () => T, fallback?: ErrorClass): T {
  try { return operation(); } catch (error) { throw toCrawlingoError(error, fallback); }
}

export async function callNativeAsync<T>(operation: () => Promise<T>, fallback?: ErrorClass): Promise<T> {
  try { return await operation(); } catch (error) { throw toCrawlingoError(error, fallback); }
}
