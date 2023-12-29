use crate::{agent::Agent, Backend};
use std::{
    collections::HashMap,
    sync::{
        atomic::{self, AtomicU16},
        Arc, RwLock,
    },
};

pub type EntityId = u16;

mod manager {
    use super::{Entity, EntityId};
    use crate::{agent::Agent, Backend};
    use std::{
        collections::HashMap,
        sync::{
            atomic::{self, AtomicU16},
            Arc, RwLock,
        },
    };

    pub struct EntityManager<'agent, B>
    where
        for<'backend> B: Backend<'backend>,
    {
        id_generator: IdGenerator,
        agents: RwLock<HashMap<EntityId, Agent<'agent, B>>>,
    }

    pub struct EntityManagerTemp<'agent, B>
    where
        for<'backend> B: Backend<'backend>,
    {
        id_generator: IdGenerator,
        entities: RwLock<HashMap<EntityId, Entity<'agent, B>>>,
    }

    impl<'agent, B> EntityManagerTemp<'agent, B>
    where
        for<'backend> B: Backend<'backend>,
    {
        pub fn new(
            id_generator: IdGenerator,
            entities: RwLock<HashMap<EntityId, Entity<'agent, B>>>,
        ) -> Self {
            Self {
                id_generator,
                entities: RwLock::new(HashMap::new()),
            }
        }
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
}

pub struct Entity<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    agent: Agent<'agent, B>,
    dependents: Vec<Relationship<'agent, B>>,
}

impl<'agent, B> Entity<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    pub fn new(agent: Agent<'agent, B>, dependents: Vec<Relationship<'agent, B>>) -> Self {
        Self { agent, dependents }
    }
}

pub enum Relationship<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    Publisher(LazyAgent<'agent, B>),
    Subscriber(LazyAgent<'agent, B>),
}

pub enum LazyAgent<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    Unloaded(EntityId),
    Loaded(Arc<Agent<'agent, B>>),
}
