import z2edit
from z2edit.util import ObjectDict

def meta_update(edit, **kwargs):
    meta = edit.meta
    meta.update(kwargs)
    edit.meta = meta

def diff_palettes(prja, prjb, config):
    ea = prja[-1]; eb = prjb[-1]
    palettes = eb.create("PaletteGroup")
    data = ObjectDict.from_json(palettes.text)
    for p in config.palette:
        for n in p.palette:
            idpath = '/'.join((p.id, n.id))
            a = ea.create('Palette', idpath)
            a.unpack()
            b = eb.create('Palette', idpath)
            b.unpack()
            if b.text != a.text:
                print('diff_palettes:', idpath)
                data.data.append(ObjectDict.from_json(b.text))

    palettes.text = data.to_json()
    meta_update(palettes, skip_pack=True)
    prjb.append(palettes)

def diff_enemies(prja, prjb, config):
    ea = prja[-1]; eb = prjb[-1]
    enemies = eb.create("EnemyGroup")
    data = ObjectDict.from_json(enemies.text)
    for group in config.enemy:
        for n in group.enemy:
            idpath = '/'.join((group.id, n.id))

            a = ea.create('Enemy', idpath)
            a.unpack()
            b = eb.create('Enemy', idpath)
            b.unpack()
            if b.text != a.text:
                print('diff_enemies:', idpath)
                data.data.append(ObjectDict.from_json(b.text))

    enemies.text = data.to_json()
    meta_update(enemies, skip_pack=True)
    prjb.append(enemies)

def diff_experience(prja, prjb, config):
    ea = prja[-1]; eb = prjb[-1]
    experience = eb.create("ExperienceTableGroup")
    data = ObjectDict.from_json(experience.text)
    for exp in config.experience.group:
        for entry in exp.table:
            idpath = '/'.join((exp.id, entry.id))
            a = ea.create('ExperienceTable', idpath)
            a.unpack()
            b = eb.create('ExperienceTable', idpath)
            b.unpack()
            if b.text != a.text:
                print('diff_experience:', idpath)
                data.data.append(ObjectDict.from_json(b.text))

    experience.text = data.to_json()
    meta_update(experience, skip_pack=True)
    prjb.append(experience)

def diff_metatiles(prja, prjb, config):
    ea = prja[-1]; eb = prjb[-1]
    ma = ea.create("MetatileGroup")
    ma.unpack()
    aa = ObjectDict.from_json(ma.text)

    mb = eb.create("MetatileGroup")
    mb.unpack()
    bb = ObjectDict.from_json(mb.text)
    for (da, db) in zip(aa.data, bb.data):
        for k in da.tile:
            if da.tile[k] == db.tile.get(k):
                db.tile.pop(k, None)
        for k in da.palette:
            if da.palette[k] == db.palette.get(k):
                db.palette.pop(k, None)

    mb.text = bb.to_json()
    meta_update(mb, skip_pack=True)
    print("diff_metatiles")
    prjb.append(mb)

def diff_texttable(prja, prjb, config):
    ea = prja[-1]; eb = prjb[-1]
    ta = ea.create("TextTable")
    ta.unpack()
    aa = ObjectDict.from_json(ta.text)
    tb = eb.create("TextTable")
    tb.unpack()
    bb = ObjectDict.from_json(tb.text)

    da = aa.data; db = bb.data[:]
    bb.data = []
    for (a, b) in zip(da, db):
        if a != b:
            bb.data.append(b)

    tb.text = bb.to_json()
    meta_update(tb, skip_pack=True)
    print("diff_texttable")
    prjb.append(tb)

def diff_startvalues(prja, prjb, config):
    ea = prja[-1]; eb = prjb[-1]
    sa = ea.create("Start")
    sa.unpack()
    aa = ObjectDict.from_json(sa.text)
    sb = eb.create("Start")
    sb.unpack()
    bb = ObjectDict.from_json(sb.text)

    if aa != bb:
        sb.text = bb.to_json()
        meta_update(sb, skip_pack=True)
        print("diff_startvalues")
        prjb.append(sb)

def diff_overworld(prja, prjb, config):
    ea = prja[-1]; eb = prjb[-1]
    for ov in config.overworld.map:
        oa = ea.create("Overworld", ov.id)
        oa.unpack()
        ob = eb.create("Overworld", ov.id)
        ob.unpack()
        if oa.text != ob.text:
            print("diff_overworld:", ov.id)
            meta_update(ob, label=ov.id, skip_pack=True)
            prjb.append(ob)

def diff_sideview(prja, prjb, config):
    ea = prja[-1]; eb = prjb[-1]
    for sv in config.sideview.group:
        for n in range(0, sv.length):
            idpath = "%s/%d" % (sv.id, n)
            print("diff_sideview: created", idpath)
            try:
                sa = ea.create("Sideview", idpath)
                sa.unpack()
                sb = eb.create("Sideview", idpath)
                sb.unpack()
                if sa.text != sb.text:
                    print("diff_sideview:", idpath)
                    meta_update(sb, skip_pack=True, label=idpath)
                    prjb.append(sb)
            except Exception as e:
                print(e)

def diff(prja, prjb):
    edit = prjb[-1]
    config = ObjectDict.from_json(z2edit.config[edit.meta['config']])
    
    diff_palettes(prja, prjb, config)
    diff_enemies(prja, prjb, config)
    diff_experience(prja, prjb, config)
    diff_metatiles(prja, prjb, config)
    diff_texttable(prja, prjb, config)
    diff_startvalues(prja, prjb, config)
    diff_overworld(prja, prjb, config)
    diff_sideview(prja, prjb, config)
