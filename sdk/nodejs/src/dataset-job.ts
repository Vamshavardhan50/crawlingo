import { JsDatasetJob } from './native.js';
import { Session } from './session';
import { callNative, callNativeAsync } from './errors';

/** Shared JSON configuration understood by the Rust engine, Python SDK and CLI. */
export interface JobField {
  name: string;
  selector: string;
  selector_type?: 'css' | 'xpath' | 'regex' | 'text' | 'after_text' | 'before_text';
  extract_type?: 'text' | 'price' | 'datetime' | 'url' | `attr:${string}`;
  required?: boolean;
  default?: string;
  data_type?: 'string' | 'integer' | 'float' | 'boolean';
  normalize_whitespace?: boolean;
  min_length?: number;
}
export interface JobConfig {
  urls_file: string;
  output_dir: string;
  fields: JobField[];
  row_selector?: string;
  concurrency?: number;
  checkpoint_pages?: number;
  checkpoint_bytes?: number;
  max_response_bytes?: number;
  max_records_per_page?: number;
  max_record_bytes?: number;
  deduplicate?: boolean;
  dedup_fields?: string[];
  respect_robots?: boolean;
  format?: 'jsonl' | 'csv';
}
export interface JobReport {
  source_lines: number;
  pages_fetched: number;
  pages_failed: number;
  duplicate_urls: number;
  records_written: number;
  records_rejected: number;
  duplicate_records: number;
  shards: number;
  complete: boolean;
  cancelled: boolean;
  elapsed_ms: number;
}

/** Streams files in Rust; only the final report crosses the language boundary. */
export class DatasetJob {
  private readonly inner: JsDatasetJob;
  constructor(config: JobConfig, session: Session = new Session()) {
    this.inner = callNative(() => new JsDatasetJob(JSON.stringify(config), session.inner));
  }
  async run(): Promise<JobReport> { return JSON.parse(await callNativeAsync(() => this.inner.run())) as JobReport; }
  cancel(): void { this.inner.cancel(); }
}
