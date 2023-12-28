use crate::{agent::Agent, Backend};
use std::{
    collections::HashMap,
    sync::{
        atomic::{self, AtomicU16},
        Arc, RwLock,
    },
};

pub type EntityId = u16;

// pub type AgentEntityManager<'agent, 'backend> =
//     Arc<EntityManager<'agent, Arc<dyn Backend<'backend>>>>;

pub struct EntityManager<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    id_generator: IdGenerator,
    agents: RwLock<HashMap<EntityId, Agent<'agent, B>>>,
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

    pub fn insert(&self, agent: Agent<'agent, B>) -> Option<EntityId> {
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

    pub fn allocate(&self) -> Option<EntityId> {
        match self.0.fetch_add(1, atomic::Ordering::Relaxed) {
            id if id == u16::MAX => None,
            id => Some(id),
        }
    }
}

struct Entity<'agent, B, const DEPENDENTS: usize>
where
    for<'backend> B: Backend<'backend>,
{
    agent: Agent<'agent, B>,
    dependents: [Relationship<'agent, B>; DEPENDENTS],
    // dependents: Vec<Relationship<'agent, B>>,
}

enum Relationship<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    Publisher(LazyAgent<'agent, B>),
    Subscriber(LazyAgent<'agent, B>),
}

enum LazyAgent<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    Unloaded(EntityId),
    Loaded(Arc<Agent<'agent, B>>),
}
