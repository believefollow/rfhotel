mod simple_broker;

use async_graphql::{Context, Enum, Object, Result, Schema, Subscription, ID};
use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use simple_broker::SimpleBroker;
use slab::Slab;
use std::sync::Arc;
use std::time::Duration;

pub type RoomsSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Clone)]
pub struct Room {
    id: ID,
    name: String,
    author: String,
}

#[Object]
impl Room {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn author(&self) -> &str {
        &self.author
    }
}

pub type Storage = Arc<Mutex<Slab<Room>>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn rooms(&self, ctx: &Context<'_>) -> Vec<Room> {
        let rooms = ctx.data_unchecked::<Storage>().lock().await;
        rooms.iter().map(|(_, room)| room).cloned().collect()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_room(&self, ctx: &Context<'_>, name: String, author: String) -> ID {
        let mut rooms = ctx.data_unchecked::<Storage>().lock().await;
        let entry = rooms.vacant_entry();
        let id: ID = entry.key().into();
        let room = Room {
            id: id.clone(),
            name,
            author,
        };
        entry.insert(room);
        SimpleBroker::publish(RoomChanged {
            mutation_type: MutationType::Created,
            id: id.clone(),
        });
        id
    }

    async fn delete_room(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let mut rooms = ctx.data_unchecked::<Storage>().lock().await;
        let id = id.parse::<usize>()?;
        if rooms.contains(id) {
            rooms.remove(id);
            SimpleBroker::publish(RoomChanged {
                mutation_type: MutationType::Deleted,
                id: id.into(),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
enum MutationType {
    Created,
    Deleted,
}

#[derive(Clone)]
struct RoomChanged {
    mutation_type: MutationType,
    id: ID,
}

#[Object]
impl RoomChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> &ID {
        &self.id
    }

    async fn room(&self, ctx: &Context<'_>) -> Result<Option<Room>> {
        let rooms = ctx.data_unchecked::<Storage>().lock().await;
        let id = self.id.parse::<usize>()?;
        Ok(rooms.get(id).cloned())
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }

    async fn rooms(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = RoomChanged> {
        SimpleBroker::<RoomChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
