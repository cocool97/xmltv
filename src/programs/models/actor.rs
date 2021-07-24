pub struct XMLTVProgramActor {
    actor: String,
    role: Option<String>,
}

impl XMLTVProgramActor {
    pub fn new(actor: String, role: Option<String>) -> Self {
        Self { actor, role }
    }

    pub fn actor(&self) -> &String {
        &self.actor
    }

    pub fn role(&self) -> Option<&String> {
        self.role.as_ref()
    }

    pub fn set_actor(&mut self, actor: String) {
        self.actor = actor;
    }

    pub fn set_role(&mut self, role: String) {
        self.role = Some(role);
    }
}
