use bevy::prelude::*;
use bevy::ecs::system::{FunctionSystem, SystemParam};
use std::sync::{Arc, Mutex};
use bevy_adventure_derive::Event;



// ========== BASIC EVENT TRAIT ==========

/// A generic trait for constructing, initializing, and firing events.
pub trait Event: 'static + Sync + Send + Sized {
    // Construct the event
    fn new<Handler: EventHandler, F: IntoEventHandler<Handler = Handler>>(handler: F) -> Self;
    // Borrow the EventHandler
    fn get_handler(&self) -> Arc<Mutex<dyn EventHandler>>;
}

/// A generic trait to store event handlers of any type.
pub trait EventHandler: 'static + Sync + Send {
    // Run the handler function
    fn fire(&mut self, world: &mut World);
    // Set up the EventHandler (FunctionSystems msut be initialized before use)
    fn init(&mut self, world: &mut World);
}

/// A generic trait to convert types into EventHandlers.
pub trait IntoEventHandler: Sync + Send {
    // The struct we're going to convert into an EventHandler
    type Handler: EventHandler;

    // The method conver the struct to an EventHandler
    fn into_event(this: Self) -> Self::Handler where Self: Sized;
}



// ========== IMPL EVENT FOR FUNCTIONSYSTEMS ==========

/// A struct to store a FunctionSystem.
/// 
/// To be thread-safe, the FunctionSystem must be in an Arc<Mutex<T>>.
/// This struct stores the FunctionSystem in an Arc<Mutex<T>>.
pub struct FunctionSystemHandler <Param: SystemParam + 'static, F>
where
    F: SystemParamFunction<(), (), Param, ()> + Sized
{
    handler: Arc<Mutex<FunctionSystem<(), (), Param, (), F>>>
}

/// Implement EventHandler for FunctionSystemHandler,
/// so we can make an IntoEventHandler for it and use .fire() on it.
impl <Param, F> EventHandler for FunctionSystemHandler<Param, F>
where
    Param: SystemParam + Sized + 'static,
    F: SystemParamFunction<(), (), Param, ()> + Sized
{
    fn fire(&mut self, world: &mut World) {
        let mut system = self.handler
            .lock()
            .unwrap();
        system.run((), world);
        system.apply_buffers(world);
    }
    fn init(&mut self, world: &mut World) {
        self.handler
            .lock()
            .unwrap()
            .initialize(world);
    }
}

/// Implement IntoEventHandler for FunctionSystems.
/// This will build a FunctionSystemHandler with the FunctionSystem
/// stored inside of it.
impl <Param, F> IntoEventHandler for FunctionSystem<(), (), Param, (), F>
where
    Param: SystemParam + Sized + 'static,
    F: SystemParamFunction<(), (), Param, ()> + Sized
{
    type Handler = FunctionSystemHandler<Param, F>;

    fn into_event(this: Self) -> Self::Handler {
        return Self::Handler {
            handler: Arc::new(Mutex::new(this))
        }
    }
}



// ========== EVENTS ==========

// The event handler each event stores
type Handler = Arc<Mutex<dyn EventHandler>>;

// When an entity dies
#[derive(Event)]
#[derive(Component)]
pub struct OnDeath(Handler);

// When an entity is interacted with
#[derive(Event)]
#[derive(Component)]
pub struct OnInteract(Handler);

// When a player enters a room
#[derive(Event)]
#[derive(Component)]
pub struct OnEnterRoom(Handler);
