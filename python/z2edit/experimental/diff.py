# Experimental utility to diff projects

def update_meta(edit, **kwargs):
    meta = edit.meta
    for (k, v) in kwargs.items():
        setattr(meta, k, v)
    edit.meta = meta

def z2diff(vanilla, other):
    for key in other.edits():
        x = other[key]
        try:
            v = vanilla[key]
            if v.data != x.data:
                print(f"Item {key} is different")
                update_meta(x, timestamp=1)
        except KeyError:
            print(f"Item {key} doesn't exist in vanilla")
            update_meta(x, timestamp=1)
