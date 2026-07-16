from enum import Enum
from typing import Dict
from ._crawlingo_core import FieldType as _CoreFieldType, FieldConstraint as _CoreFieldConstraint, DatasetSchema as _CoreDatasetSchema

class FieldType(Enum):
    """Supported field data types for schema validation."""
    String = _CoreFieldType.String
    Integer = _CoreFieldType.Integer
    Float = _CoreFieldType.Float
    Boolean = _CoreFieldType.Boolean


class FieldConstraint:
    """A field constraint in a dataset schema."""
    def __init__(self, name: str, field_type: FieldType, required: bool):
        self._core_constraint = _CoreFieldConstraint(name, field_type.value, required)

    @property
    def name(self) -> str:
        return self._core_constraint.name

    @property
    def field_type(self) -> FieldType:
        return FieldType(self._core_constraint.field_type)

    @property
    def required(self) -> bool:
        return self._core_constraint.required


class DatasetSchema:
    """A dataset schema defining expected fields and their constraints."""
    def __init__(self, _core_schema=None):
        self._core_schema = _core_schema or _CoreDatasetSchema()

    def add_field(self, name: str, field_type: FieldType, required: bool) -> "DatasetSchema":
        """Add a field constraint to the schema."""
        self._core_schema.add_field(name, field_type.value, required)
        return self

    def validate(self, record: Dict[str, str]) -> Dict[str, str]:
        """Validate a field map against this schema."""
        return self._core_schema.validate(record)
