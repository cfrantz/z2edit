#!/usr/bin/env python3

import argparse
import re
import sys
from pprint import pprint

flags = argparse.ArgumentParser(description="Code generator")
flags.add_argument('--proto', type=str, help="Proto definitions")
flags.add_argument('--message', '-m', type=str, help="Message to process")
flags.add_argument('--kind', '-k', choices=['flags', 'style_var', 'color'], help="Kind to generate")

def as_bool(s):
    if not s:
        return False
    s = s.lower()
    if s in ('true', 'yes', '1'):
        return True
    return False

def studly_caps(word):
    return ''.join(w.capitalize() for w in word.split('_'))

class ProtoFile(object):
    def __init__(self, filename):
        self.content = open(filename).read()

    def message(self, name):
        ret = []
        rx = r'message\s+' + name + r'\s+{(.*?)}'
        rx = r'message\s+' + name + r'\s+{([^}]*)}'
        m = re.search(rx, self.content, re.MULTILINE)
        if not m:
            return None
        m = m.group(1)
        for field in re.finditer(r'\s*(?:repeated\s+)?(\w+)\s+(\w+)\s*=\s*(\d+);\s*(//.*)?', m):
            extra = {}
            if field.group(4):
                for kv in re.finditer(r'\s*(\w+)=([^ ]*)', field.group(4)[2:]):
                    extra[kv.group(1)] = kv.group(2)
            if as_bool(extra.get('ignore')):
                continue;
            ret.append((field.group(1), field.group(2), field.group(3), extra))
        return ret

class Codegen(object):
    @staticmethod
    def flags(msgname, fields):
        code = []
        imname = f'ImGui{msgname}'
        code.append(f'{imname} Get{msgname}(const proto::{msgname}& flags) {{')
        code.append(f'    {imname} f = 0;')
        for (ty, name, *_) in fields:
            flagname = (studly_caps(name
                                   .replace('float_', 'float'))
                                   .replace('Rgb', 'RGB')
                                   .replace('Hsv', 'HSV')
                                   .replace('Hdr', 'HDR'))
            code.append(f'    if (flags.{name}()) f |= {imname}_{flagname};')
        code.append(f'    return f;')
        code.append('}')
        return '\n'.join(code)

    @staticmethod
    def style_var(msgname, fields):
        code = []
        imname = f'ImGui{msgname}'
        code.append(f'int Push{msgname}(const proto::{msgname}& v) {{')
        code.append('    int pushes = 0;')
        for (ty, name, field_num, extra) in fields:
            flagname = studly_caps(name)
            if extra['type'] == 'float':
                num = 1
                items = 'item'
            elif extra['type'] == 'ImVec2':
                num = 2;
                items = 'items'
            else:
                raise Exception(f'Unknown type: {extra}')
            code.append(f'    if (v.{name}_size() != 0) {{')
            code.append(f'        if (v.{name}_size() == {num}) {{')
            if num == 1:
                code.append(f'            float val = v.{name}(0);')
            elif num == 2:
                code.append(f'            ImVec2 val(v.{name}(0), v.{name}(1));')
            varname = studly_caps(name)
            code.append(f'            ImGui::PushStyleVar({imname}_{varname}, val);')
            code.append('            pushes +=1;')
            code.append('        } else {')
            code.append(f'            LOG(ERROR) << "{name}: expecting exactly {num} {items}, but got " << v.{name}_size();')
            code.append('        }')
            code.append('    }')
        code.append(f'    return pushes;')
        code.append('}')
        return '\n'.join(code)

    @staticmethod
    def color(msgname, fields):
        code = []
        imname = f'ImGui{msgname}'
        code.append(f'int PushStyleColor(const proto::{msgname}& v) {{')
        code.append('    int pushes = 0;')
        for (ty, name, field_num, extra) in fields:
            col = studly_caps(name)
            code.append(f'    if (v.{name}() != 0) {{')
            code.append(f'        ImGui::PushStyleColor({imname}_{col}, v.{name}());')
            code.append('        pushes +=1;')
            code.append('    }')
        code.append(f'    return pushes;')
        code.append('}')
        return '\n'.join(code)


def main(args):
    p = ProtoFile(args.proto)
    m = p.message(args.message)
    if args.kind == 'flags':
        code = Codegen.flags(args.message, m)
    elif args.kind == 'style_var':
        code = Codegen.style_var(args.message, m)
    elif args.kind == 'color':
        code = Codegen.color(args.message, m)
    else:
        print("Unknown!")
    print(code)


if __name__ == '__main__':
    args = flags.parse_args()
    sys.exit(main(args))
