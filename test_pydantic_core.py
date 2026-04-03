import pydantic_core
from pydantic_core import core_schema

validator = pydantic_core.SchemaValidator(core_schema.typed_dict_schema(fields={}))
try:
    validator.validate_json("not valid json")
except pydantic_core.ValidationError as e:
    print(e.errors())

try:
    validator.validate_json("[]")
except pydantic_core.ValidationError as e:
    print(e.errors())

