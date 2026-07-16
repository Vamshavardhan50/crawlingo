import { JsDatasetSchema, JsFieldType } from './native.js';

export enum FieldType {
  String = 0,
  Integer = 1,
  Float = 2,
  Boolean = 3,
}

export class FieldConstraint {
  public readonly name: string;
  public readonly fieldType: FieldType;
  public readonly required: boolean;

  constructor(name: string, fieldType: FieldType, required: boolean) {
    this.name = name;
    this.fieldType = fieldType;
    this.required = required;
  }
}

export class DatasetSchema {
  public readonly inner: JsDatasetSchema;

  constructor() {
    this.inner = new JsDatasetSchema();
  }

  public addField(name: string, fieldType: FieldType, required: boolean): this {
    this.inner.addField(name, fieldType as unknown as JsFieldType, required);
    return this;
  }

  public validate(record: Record<string, string>): Record<string, string> {
    return this.inner.validate(record);
  }
}
