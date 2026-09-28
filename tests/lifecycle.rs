//! Verifies opt-in lifecycle traits without imposing them on every module.

use std::cell::Cell;
use std::rc::Rc;

use rustclamp_core::{
    ApplicationId, Drain, Initialize, LifecycleContext, Module, ModuleId, ProcessId, Ready, Start,
    Stop,
};

const APP: ApplicationId = ApplicationId::new("test.lifecycle.application");
const PROCESS: ProcessId = ProcessId::new("test.lifecycle.worker");

struct PlainModule;

impl Module for PlainModule {
    const ID: ModuleId = ModuleId::new("test.module.plain");
}

struct Database {
    open: bool,
}

impl Module for Database {
    const ID: ModuleId = ModuleId::new("test.module.database");
}

impl Initialize for Database {
    type Error = &'static str;

    fn initialize(&mut self, context: &LifecycleContext) -> Result<(), Self::Error> {
        if (context.application(), context.process()) != (APP, PROCESS) {
            return Err("wrong lifecycle context");
        }
        self.open = true;
        Ok(())
    }
}

impl Ready for Database {
    type Error = &'static str;

    fn ready(&mut self, context: &LifecycleContext) -> Result<(), Self::Error> {
        if self.open && context.process() == PROCESS {
            Ok(())
        } else {
            Err("database not initialized for this process")
        }
    }
}

impl Stop for Database {
    type Error = &'static str;

    fn stop(&mut self, _context: &LifecycleContext) -> Result<(), Self::Error> {
        self.open = false;
        Ok(())
    }
}

struct Users {
    events: Rc<Cell<u8>>,
}

impl Module for Users {
    const ID: ModuleId = ModuleId::new("test.module.users");
}

impl Start for Users {
    type Error = &'static str;

    fn start(&mut self, _context: &LifecycleContext) -> Result<(), Self::Error> {
        self.events.set(self.events.get() + 1);
        Ok(())
    }
}

impl Drain for Users {
    type Error = &'static str;

    fn drain(&mut self, _context: &LifecycleContext) -> Result<(), Self::Error> {
        self.events.set(self.events.get() + 1);
        Ok(())
    }
}

#[test]
fn module_can_omit_every_lifecycle_contract() {
    fn accepts_module<T: Module>() {}

    accepts_module::<PlainModule>();
    assert_eq!(PlainModule::ID.as_str(), "test.module.plain");
}

#[test]
fn lifecycle_participation_is_phase_specific_and_receives_identity() {
    let context = LifecycleContext::new(APP, PROCESS);
    let mut database = Database { open: false };
    database.initialize(&context).unwrap();
    database.ready(&context).unwrap();
    assert!(database.open);
    database.stop(&context).unwrap();
    assert!(!database.open);

    let events = Rc::new(Cell::new(0));
    let mut users = Users {
        events: events.clone(),
    };
    users.start(&context).unwrap();
    users.drain(&context).unwrap();
    assert_eq!(events.get(), 2);
}
