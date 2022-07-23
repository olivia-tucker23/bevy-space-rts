use bevy::{prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::*;

// Available units and their names
pub enum UnitType {
    DefaultUnit,
    Tank,
    Plane,
    Fighter,
    Building,
}

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnitType::DefaultUnit => write!(f, "DefaultUnit"),
            UnitType::Tank => write!(f, "Tank"),
            UnitType::Fighter => write!(f, "Fighter"),
            UnitType::Plane => write!(f, "Plane"),
            UnitType::Building => write!(f, "Building"),
        }
    }
}

// Master decoder of units and their properties. TODO I/O
pub fn spawn_units_system(
    mut ev_spawn: EventReader<SpawnUnitEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_spawn.iter() {
        println!("Spawning unit owned by Player {}", ev.player.id);
        let mut ec = commands.spawn();
        ec.insert(Unit::new(ev.unit_type.to_string(), ev.player.clone()));
        ec.insert( Velocity { ..Default::default() } );
        match &ev.unit_type {

        UnitType::DefaultUnit => {
            // TODO bundle
            let unit_size = Vec2::new(1350., 762.);
            ec.insert( Hp { max: 100, current: 100 } );
            ec.insert( Body::new(ev.position, unit_size) );
            ec.insert( Targets::new() );
            if ev.player.id == USER_ID {
                ec.insert( Targeterable );
                ec.insert( Movable );
            }
            else {
                ec.insert( Targeteeable );
            }
            ec.insert( Selectable );
            ec.insert( UnitPath::new() );
            ec.insert_bundle( TransformBundle {
                local: Transform {
                    translation: Vec3::new( ev.position.x, ev.position.y, UNIT_ZORDER ),
                    scale: Vec3::ONE * SPRITE_SCALE,
                    rotation: Quat::from_rotation_z( ev.position.z ),
                    ..Default::default()
                },
                ..Default::default()
            });

            // Add children
            ec.with_children(|parent| {
                
                // TODO thrusters

                // Hardpoints
                add_turret(parent, &asset_server, Vec2::new(460., 0.));
                add_turret(parent, &asset_server, Vec2::new(-110., 0.));

                // Debug sprites
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1., 0., 0., 0.1),
                        custom_size: Some(unit_size),
                        ..Default::default()
                    },
                    transform: Transform { translation: Vec3::new(0., 0., -1.), ..Default::default() },
                    ..Default::default()
                }).insert(DebugRect);
                
                parent.spawn_bundle(GeometryBuilder::build_as(&shapes::RegularPolygon {
                    sides: 30,
                    feature: shapes::RegularPolygonFeature::Radius((unit_size[0] + unit_size[1]) / 4.),
                    ..shapes::RegularPolygon::default()
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::rgba(0., 1., 0., 0.2)),
                    outline_mode: StrokeMode::new(Color::rgba(0., 1., 0., 0.2), 2.),
                },
                Transform { translation: Vec3::new(0., 0., -2.), ..Default::default() },
                )).insert(DebugSelectionRadius);

                // Sprites
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("ship1.png"),
                    transform: Transform { translation: Vec3::new(0., 0., UNIT_ZORDER), ..Default::default() },
                    ..Default::default()
                }).insert(MainSprite);

            });
        }
        UnitType::Fighter => {
            // TODO bundle
            let unit_size = Vec2::new(207., 204.);
            ec.insert( Hp { max: 100, current: 100 } );
            ec.insert( Body::new(ev.position, unit_size) );
            ec.insert( Targets::new() );
            if ev.player.id == USER_ID {
                ec.insert( Targeterable );
                ec.insert( Movable );
            }
            else {
                ec.insert( Targeteeable );
            }
            ec.insert( Selectable );
            ec.insert( UnitPath::new() );
            ec.insert_bundle( TransformBundle {
                local: Transform {
                    translation: Vec3::new( ev.position.x, ev.position.y, UNIT_ZORDER ),
                    scale: Vec3::ONE * SPRITE_SCALE,
                    rotation: Quat::from_rotation_z( ev.position.z ),
                    ..Default::default()
                },
                ..Default::default()
            });

            // Add children
            ec.with_children(|parent| {
                
                // Debug sprites
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1., 0., 0., 0.1),
                        custom_size: Some(unit_size),
                        ..Default::default()
                    },
                    transform: Transform { translation: Vec3::new(0., 0., -1.), ..Default::default() },
                    ..Default::default()
                }).insert(DebugRect);
                
                parent.spawn_bundle(GeometryBuilder::build_as(&shapes::RegularPolygon {
                    sides: 30,
                    feature: shapes::RegularPolygonFeature::Radius((unit_size[0] + unit_size[1]) / 4.),
                    ..shapes::RegularPolygon::default()
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::rgba(0., 1., 0., 0.2)),
                    outline_mode: StrokeMode::new(Color::rgba(0., 1., 0., 0.2), 2.),
                },
                Transform { translation: Vec3::new(0., 0., -2.), ..Default::default() },
                )).insert(DebugSelectionRadius);

                // Sprites
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("fighter1.png"),
                    transform: Transform { translation: Vec3::new(0., 0., UNIT_ZORDER), ..Default::default() },
                    ..Default::default()
                }).insert(MainSprite);

            });
        }
        UnitType::Building => {
            println!("Building not spawned.\n");
        }

        other => {
            println!("Unit not spawned.\n");
        }

        };
    }
}

fn add_turret(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, displacement: Vec2) {
    parent.spawn_bundle(SpriteBundle {
        texture: asset_server.load("turret1.png"),
        transform: Transform { translation: Vec3::new(displacement.x, displacement.y, UNIT_ZORDER + 1.), ..Default::default() },
        ..Default::default()
    })
    .insert(
        Body::new(
            Vec3::new(displacement.x, displacement.y, 0.),
            Vec2::new(162., 168.)
        )
    )
    .insert(Unit::new("Turret".to_string(), Player { id: USER_ID }))
    .insert(Subunit)
    .insert(Velocity { ..Default::default() })
    .insert(Range { sight: 1000., fire: 800. })
    .insert(Targets::new())
    .insert(Turret {
        reload_time: 1.
    });

}