use crate::xmltv_error::Result;
use xml_builder::XMLElement;

use crate::XMLTVProgramActor;

pub struct XMLTVProgramCredits {
    directors: Vec<String>,
    actors: Vec<XMLTVProgramActor>,
    writers: Vec<String>,
    adapters: Vec<String>,
    producers: Vec<String>,
    composers: Vec<String>,
    editors: Vec<String>,
    presenters: Vec<String>,
    commentators: Vec<String>,
    guests: Vec<String>,
}

impl XMLTVProgramCredits {
    pub fn new() -> Self {
        XMLTVProgramCredits {
            directors: vec![],
            actors: vec![],
            writers: vec![],
            adapters: vec![],
            producers: vec![],
            composers: vec![],
            editors: vec![],
            presenters: vec![],
            commentators: vec![],
            guests: vec![],
        }
    }

    pub fn add_director(&mut self, director: String) {
        self.directors.push(director);
    }

    pub fn add_actor(&mut self, actor: XMLTVProgramActor) {
        self.actors.push(actor);
    }

    pub fn add_writer(&mut self, writer: String) {
        self.writers.push(writer);
    }

    pub fn add_adapter(&mut self, adapter: String) {
        self.adapters.push(adapter);
    }

    pub fn add_producer(&mut self, producer: String) {
        self.producers.push(producer);
    }

    pub fn add_composer(&mut self, composer: String) {
        self.composers.push(composer);
    }

    pub fn add_editor(&mut self, editor: String) {
        self.editors.push(editor);
    }

    pub fn add_presenter(&mut self, presenter: String) {
        self.presenters.push(presenter);
    }

    pub fn add_commentator(&mut self, commentator: String) {
        self.commentators.push(commentator);
    }

    pub fn add_guest(&mut self, guest: String) {
        self.guests.push(guest);
    }

    pub fn to_xmlelement(&self) -> Result<XMLElement> {
        let mut element = XMLElement::new("credits");

        for director in &self.directors {
            let mut elem = XMLElement::new("director");
            elem.add_text(director.to_owned())?;
            element.add_child(elem)?;
        }

        for actor in &self.actors {
            let mut elem = XMLElement::new("actor");
            if let Some(role) = actor.role() {
                elem.add_attribute("role", role);
            }

            elem.add_text(actor.actor().to_owned())?;
            element.add_child(elem)?;
        }

        for writer in &self.writers {
            let mut elem = XMLElement::new("writer");
            elem.add_text(writer.to_owned())?;
            element.add_child(elem)?;
        }

        for adapter in &self.adapters {
            let mut elem = XMLElement::new("adapter");
            elem.add_text(adapter.to_owned())?;
            element.add_child(elem)?;
        }

        for producer in &self.producers {
            let mut elem = XMLElement::new("producer");
            elem.add_text(producer.to_owned())?;
            element.add_child(elem)?;
        }

        for composer in &self.composers {
            let mut elem = XMLElement::new("composer");
            elem.add_text(composer.to_owned())?;
            element.add_child(elem)?;
        }

        for editor in &self.editors {
            let mut elem = XMLElement::new("editor");
            elem.add_text(editor.to_owned())?;
            element.add_child(elem)?;
        }

        for presenter in &self.presenters {
            let mut elem = XMLElement::new("presenter");
            elem.add_text(presenter.to_owned())?;
            element.add_child(elem)?;
        }

        for commentator in &self.commentators {
            let mut elem = XMLElement::new("commentator");
            elem.add_text(commentator.to_owned())?;
            element.add_child(elem)?;
        }

        for guest in &self.guests {
            let mut elem = XMLElement::new("guest");
            elem.add_text(guest.to_owned())?;
            element.add_child(elem)?;
        }

        Ok(element)
    }
}
