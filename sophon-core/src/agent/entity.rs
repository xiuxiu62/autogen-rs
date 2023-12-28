use super::{
    message::{Message, MessagePublisher},
    Agent, Backend,
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{self, AtomicU16},
        Arc, RwLock,
    },
};

pub type AgentEntityManager<'agent, 'backend> =
    Arc<EntityManager<'agent, Box<dyn Backend<'backend>>>>;

pub struct EntityManager<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    id_generator: IdGenerator,
    agents: RwLock<HashMap<u16, Agent<'agent, B>>>,
}

impl<'agent, B> EntityManager<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    fn new() -> Self {
        Self {
            id_generator: IdGenerator::new(),
            agents: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, agent: Agent<'agent, B>) -> Option<u16> {
        let id = self.id_generator.allocate()?;
        self.agents.write().unwrap().insert(id, agent);

        Some(id)
    }
}

struct IdGenerator(AtomicU16);

impl IdGenerator {
    #[inline]
    pub const fn new() -> Self {
        Self(AtomicU16::new(0))
    }

    pub fn allocate(&self) -> Option<u16> {
        match self.0.fetch_add(1, atomic::Ordering::Relaxed) {
            id if id == u16::MAX => None,
            id => Some(id),
        }
    }
}

impl<'agent> Backend<'agent> for Box<dyn Backend<'agent>> {
    fn query(&'agent self, message: Message<'agent>, publisher: Arc<MessagePublisher<'agent>>) {
        self.query(message, publisher)
    }
}

// fn temp() {
//     agent_entity_manager().insert(Agent::new("temp", Box::new(ExampleBackend)));
// }

// struct ExampleBackend;

// unsafe impl Send for ExampleBackend {}
// unsafe impl Sync for ExampleBackend {}

// impl Backend for ExampleBackend {
//     fn query(&self) {
//         todo!()
//     }

//     fn write(&self) {
//         todo!()
//     }
// }
