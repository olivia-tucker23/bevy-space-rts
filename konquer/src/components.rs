use std::{collections::VecDeque, marker::PhantomData};
use bevy::{prelude::{Component, Entity}, math::{Vec2, Vec3}, ecs::{archetype::Archetypes, component::ComponentId}};
use std::{sync::atomic::{AtomicU8, Ordering}};
use crate::{Player, SPRITE_SCALE};


pub fn get_components_for_entity<'a>(
    entity: &Entity,
    archetypes: &'a Archetypes,
) -> Option<impl Iterator<Item = ComponentId> + 'a> {
    for archetype in archetypes.iter() {
        if archetype.entities().contains(entity) {
            return Some(archetype.components());
        }
    }
    None
}

#[derive(Component, Clone, Copy)]
pub struct Map {
    pub w: i32,  // The human-readable name of the unit
    pub h: i32,  // The player of the unit
}

#[derive(Component)]
pub struct SelectionRect;

#[derive(Component)]
pub struct UnitSelectedCircle;

#[derive(Component)]
pub struct DebugRect;

#[derive(Component)]
pub struct DebugSelectionRadius;

#[derive(Component)]
pub struct GridLine;

#[derive(Component)]
pub struct MainSprite;

#[derive(Component)]
pub struct UnitPathDisplay;

#[derive(Component)]
pub struct Turret {
    pub reload_time: f32
}

#[derive(Component)]
pub struct Body {
	pub position: Vec3,  // x, y, w
    pub size: Vec2, // x, y
    pub selection_radius: f32
}

#[derive(Component)]
pub struct Thruster {
	pub unidirectional_thrust: f32,
    pub omnidirectional_thrust: f32
}

impl Body {
    pub fn new(position: Vec3, size: Vec2) -> Body {
        Body {
            position: position,
            size: size,
            selection_radius: (size.x + size.y) / 4. * SPRITE_SCALE
        }
    }
}

#[derive(Component)]
pub struct Velocity {
	pub dx: f32,
	pub dy: f32,
    pub dw: f32,  // Angular velocity
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            dx: 0.,
            dy: 0.,
            dw: 0.,
        }
    }
}

#[derive(Component)]
pub struct Hp {
    pub max: u8,
    pub current: u8,
}

#[derive(Component)]
pub struct Shield {
    pub max: u8,
    pub current: u8,
}

#[derive(Component)]
pub struct UnitControls {
    pub is_selected: bool,
    pub is_clickable: bool,
    pub is_movable: bool,
}

// TODO Selected by an player?
#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Selectable;


/*
Movable means can be given path via manual UI modification. Some units like
strikecraft move along paths but are not "Movable" by players.
*/
#[derive(Component)]
pub struct Movable;


#[derive(Component)]
pub struct UnitPath {
    pub path: VecDeque<Vec2>,
}

impl UnitPath {
    pub fn new() -> UnitPath {
        UnitPath { path: VecDeque::new() }
    }
}

// Wrapper for Unit references
pub struct KindedEntity<T>(Entity, PhantomData<T>);

#[derive(Component)]
pub struct Targets {
    pub deque: VecDeque<Entity>  // Deque of targets
}

impl Targets {
    pub fn new() -> Targets {
        Targets { deque: VecDeque::new() }
    }
}

#[derive(Component)]
pub struct Range {
    pub sight: f32,  // The human-readable name of the unit
    pub fire: f32  // Range at which the unit can fire
}


static NUMBER_OF_UNITS: AtomicU8 = AtomicU8::new(0);


#[derive(Component)]
pub struct Unit {
    pub name: String,  // The human-readable name of the unit
    pub player: Player,  // The player of the unit
    pub id: u8,  // The global identifying number of the unit
}

impl Unit {
    pub fn new(name: String, player: Player) -> Unit {
        Unit {
            name: name,
            player: player,
            id: NUMBER_OF_UNITS.fetch_add(1, Ordering::Relaxed)
        }
    }
}

#[derive(Component)]
pub struct Subunit;
