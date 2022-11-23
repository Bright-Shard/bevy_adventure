use bevy::prelude::World;
use std::sync::{Arc, Mutex};

// ========== BASIC EVENT TRAITS ==========

/// A generic trait for events (components that store handlers)
pub trait Event {
    // Make a new event from an EventHandler
    fn new(handler: Arc<Mutex<dyn EventHandler>>) -> Self;
}

/// A generic trait to store event handlers with any parameters.
pub trait EventHandler: 'static + Sync + Send {
    // Run the handler function
    fn fire(&mut self, world: &mut World);
    // Set up the EventHandler (FunctionSystems msut be initialized before use)
    fn init(&mut self, world: &mut World);
}

/// A generic trait to convert types into EventHandlers.
pub trait IntoEventHandler<Params>: Sync + Send + 'static {
    // The method conver the struct to an EventHandler
    fn into_event(self) -> Arc<Mutex<dyn EventHandler>> where Self: Sized;
}



// ========== IMPL EVENT HANDLER FOR SYSTEM PARAMETER FUNCTIONS ==========

use bevy::ecs::system::{SystemParamFunction, FunctionSystem, SystemParam, System};
use bevy::prelude::IntoSystem;

/// Implement [EventHandler] for [FunctionSystem]s, so they can be fired as events.
impl <Fn, Params> EventHandler for FunctionSystem<(), (), Params, (), Fn>
where
    Params: SystemParam + 'static,
    Fn: SystemParamFunction<(), (), Params, ()>
{
    fn fire(&mut self, world: &mut World) {
        self.run((), world);
    }
    fn init(&mut self, world: &mut World) {
        self.initialize(world);
    }
}

/// Implement [IntoEventHandler] for [SystemParamFunction], so they can be converted into events.
/// 
/// [SystemParamFunction] is impl for all functions that can be systems, so we need a converter
/// from that to EventHandler to use systems as event handlers.
impl <Params, F> IntoEventHandler<Params> for F
where
    Params: SystemParam + 'static,
    F: SystemParamFunction<(), (), Params, ()>
{
    fn into_event(self) -> Arc<Mutex<dyn EventHandler>> where Self: Sized {
        Arc::new(Mutex::new(IntoSystem::into_system(self)))
    }
}
