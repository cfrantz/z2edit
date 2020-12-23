import json

class ObjectDict(dict):
    "ObjectDict is a `dict` with attribute access to the dictionary contents."

    def __getattr__(self, name):
        if name in self:
            return self[name]
        else:
            raise AttributeError('No such attribute: ' + name)

    def __setattr__(self, name, value):
        self[name] = value

    def __delattr__(self, name):
        if name in self:
            del self[name]
        else:
            raise AttributeError('No such attribute: ' + name)

    @staticmethod
    def from_json(text):
        return json.loads(text, object_hook=ObjectDict)

    def to_json(self):
        return json.dumps(self, indent=4)
