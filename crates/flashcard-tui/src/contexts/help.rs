#[derive(Clone, Default)]
pub struct HelpContext {
    help_descs: Vec<Option<String>>,
}

// the maximum depth on which a new component
const MAX_COMPONENT_DEPTH: usize = 8;

impl HelpContext {
    pub fn new() -> Self {
        Self {
            help_descs: vec![None; MAX_COMPONENT_DEPTH],
        }
    }

    pub fn into_help_string(&self) -> String {
        let mut help_descs = Vec::new();
        for help_desc in self.help_descs.iter().flatten() {
            help_descs.push(help_desc.clone());
        }
        help_descs.join(", ")
    }

    pub fn update_desc_at_level(&mut self, desc: &str, level: usize) {
        if level >= MAX_COMPONENT_DEPTH {
            return;
        }
        self.help_descs[level] = Some(desc.into());
    }

    // removes all help comments below this level
    // usefull to remove help messages when moving uppwards in the component
    // tree
    pub fn clear_below_level(&mut self, level: usize) {
        for i in (level + 1)..MAX_COMPONENT_DEPTH {
            self.help_descs[i] = None;
        }
    }
}
