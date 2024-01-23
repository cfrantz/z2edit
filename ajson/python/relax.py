from ajson.python._relax import (
    CommentFormat,
    StringFormat,
    Base,
    Type,
    Location,
    Comment,
    String,
    Boolean,
    Int,
    Real,
    Null,
    Mapping,
    Sequence,
    Compact,
    Fragment,
    Document,
    Relax
)

def _deserialize(doc):
    ty = doc.type()
    if ty == Type.STRING:
        return doc.string.value
    elif ty == Type.BOOLEAN:
        return doc.boolean.value
    elif ty == Type.INT:
        return doc.integer.value
    elif ty == Type.REAL:
        return doc.real.value
    elif ty == Type.NULL:
        return None
    elif ty == Type.SEQUENCE:
        return _deserialize_sequence(doc)
    elif ty == Type.MAPPING:
        return _deserialize_mapping(doc)
    else:
        raise Exception(f"Cannot deserialize type {ty}")

def _deserialize_sequence(doc):
    ret = []
    for item in doc.mapping.value:
        if not item.has_value():
            continue
        value = None
        if item.type() == Type.FRAGMENT:
            for f in item.fragment.value:
                if not f.has_value():
                    continue
                if value:
                    raise Exception(f"Too many values at {f.location}")
                value = f
        else:
            value = item
        ret.append(_deserialize(value))
    return ret

def _deserialize_mapping(doc):
    ret = {} 
    for item in doc.mapping.value:
        if not item.has_value():
            continue
        key = None
        value = None
        if item.type() != Type.FRAGMENT:
            raise Exception(f"Expected a key-value fragment at {item.location}")

        for f in item.fragment.value:
            if not f.has_value():
                continue
            if not key:
                key = f
            elif not value:
                value = f
            else:
                raise Exception(f"Too many values at {f.location}")
        if not key or not value:
            raise Exception(f"Too few values at {item.location}")
        ret[_deserialize(key)] = _deserialize(value)
    return ret

Document.to_object = _deserialize
