import z2edit
from z2edit.util import ObjectDict

def meta_update(edit, **kwargs):
    meta = edit.meta
    meta.update(kwargs)
    edit.meta = meta

def extract_palettes(project, edit, config):
    palettes = edit.create("PaletteGroup")
    data = ObjectDict.from_json(palettes.text)
    for p in config.palette:
        for n in p.palette:
            idpath = '/'.join((p.id, n.id))
            palette = edit.create('Palette', idpath)
            print("extract_palettes: created", idpath)
            data.data.append(ObjectDict.from_json(palette.text))
    palettes.text = data.to_json()
    palettes.unpack()
    meta_update(palettes, skip_pack=True)
    project.append(palettes)
    return palettes

def extract_enemies(project, edit, config):
    enemies = edit.create("EnemyGroup")
    data = ObjectDict.from_json(enemies.text)
    for group in config.enemy:
        for n in group.enemy:
            idpath = '/'.join((group.id, n.id))
            enemy = edit.create('Enemy', idpath)
            print("extract_enemies: created", idpath)
            data.data.append(ObjectDict.from_json(enemy.text))
    enemies.text = data.to_json()
    enemies.unpack()
    meta_update(enemies, skip_pack=True)
    project.append(enemies)
    return enemies

def extract_experience(project, edit, config):
    experience = edit.create("ExperienceTableGroup")
    data = ObjectDict.from_json(experience.text)
    for exp in config.experience.group:
        for entry in exp.table:
            idpath = '/'.join((exp.id, entry.id))
            expt = edit.create('ExperienceTable', idpath)
            print("extract_experience: created", idpath)
            data.data.append(ObjectDict.from_json(expt.text))
    experience.text = data.to_json()
    experience.unpack()
    meta_update(experience, skip_pack=True)
    project.append(experience)
    return experience

def extract_metatiles(project, edit, config):
    metatile = edit.create("MetatileGroup")
    print("extract_metatiles: created object")
    metatile.unpack()
    meta_update(metatile, skip_pack=True)
    project.append(metatile)
    return metatile

def extract_texttable(project, edit, config):
    texttable = edit.create("TextTable")
    print("extract_textable: created object")
    texttable.unpack()
    meta_update(texttable, skip_pack=True)
    project.append(texttable)
    return texttable

def extract_startvalues(project, edit, config):
    start = edit.create("Start")
    print("extract_startvalues: created object")
    start.unpack()
    meta_update(start, skip_pack=True)
    project.append(start)
    return start

def extract_overworld(project, edit, config):
    for ov in config.overworld.map:
        print("extract_overworld: created", ov.id)
        overworld = edit.create("Overworld", ov.id)
        overworld.unpack()
        meta_update(overworld, skip_pack=True)
        project.append(overworld)
    return edit

def extract_sideview(project, edit, config):
    for sv in config.sideview.group:
        for n in range(0, sv.length):
            idpath = "%s/%d" % (sv.id, n)
            print("extract_sideview: created", idpath)
            try:
                sideview = edit.create("Sideview", idpath)
                sideview.unpack()
                meta_update(sideview, skip_pack=True)
                project.append(sideview)
            except Exception as e:
                print(e)
    return edit

def extract(project):
    edit = project[-1]
    config = ObjectDict.from_json(z2edit.config[edit.meta['config']])
    
    edit = extract_palettes(project, edit, config)
    edit = extract_enemies(project, edit, config)
    edit = extract_experience(project, edit, config)
    edit = extract_metatiles(project, edit, config)
    edit = extract_texttable(project, edit, config)
    edit = extract_startvalues(project, edit, config)
    edit = extract_overworld(project, edit, config)
    edit = extract_sideview(project, edit, config)
