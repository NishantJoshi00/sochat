from typing import Union
from pydantic import BaseModel

class Message(BaseModel):
    encrypted: bool
    data: str
    from_: str
    to: str

    def dict(self, *, include: Union['AbstractSetIntStr', 'MappingIntStrAny'] = None, exclude: Union['AbstractSetIntStr', 'MappingIntStrAny'] = None, by_alias: bool = False, skip_defaults: bool = None, exclude_unset: bool = False, exclude_defaults: bool = False, exclude_none: bool = False) -> 'DictStrAny':
        di = super().dict(include=include, exclude=exclude, by_alias=by_alias, skip_defaults=skip_defaults, exclude_unset=exclude_unset, exclude_defaults=exclude_defaults, exclude_none=exclude_none)
        di['from'] = di['from_']
        del di['from_']
        return di


a = Message(encrypted=True, data="hello", from_="Alice", to="Bob")

b = a.dict()
print(b)