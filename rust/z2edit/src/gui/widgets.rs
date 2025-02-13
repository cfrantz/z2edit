use indexmap::IndexMap;
use std::borrow::Cow;
use std::cmp::Eq;
use std::hash::Hash;

pub trait Combo<K: Eq + Hash + Clone, V> {
    fn combo<F>(&self, ui: &imgui::Ui, label: impl AsRef<str>, selected: &mut K, f: F) -> bool
    where
        F: for<'a> Fn(&'a K, &'a V) -> Cow<'a, str>;
}

impl<K: Eq + Hash + Clone, V> Combo<K, V> for IndexMap<K, V> {
    fn combo<F>(&self, ui: &imgui::Ui, label: impl AsRef<str>, selected: &mut K, f: F) -> bool
    where
        F: for<'a> Fn(&'a K, &'a V) -> Cow<'a, str>,
    {
        let preview = if let Some(sel) = self.get(selected) {
            f(selected, sel)
        } else {
            "<unknown>".into()
        };

        let mut changed = false;
        if let Some(_combo) = ui.begin_combo(label, preview) {
            for (k, v) in self.iter() {
                if selected == k {
                    ui.set_item_default_focus();
                }
                if ui
                    .selectable_config(f(k, v))
                    .selected(selected == k)
                    .build()
                {
                    selected.clone_from(k);
                    changed = true;
                }
            }
        }
        changed
    }
}
