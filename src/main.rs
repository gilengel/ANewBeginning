use bevy::{
    prelude::*,
    render::pass::ClearColor,
};




//use bevy_prototype_lyon::prelude::*;

mod input;
mod city;
mod roadsystem;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

struct TempStraightStreet;

// Transforms the mouse position from screen into world coordinate system
fn mouse_pos_ws(mouse_pos: Vec2) -> Vec2 {
    Vec2::new(
        mouse_pos.x() - (WINDOW_WIDTH as f32) / 2.0,
        mouse_pos.y() - (WINDOW_HEIGHT as f32) / 2.0      
    )
}


fn spawn_temp_street(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
        // create temp street for visualization
        commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 0.1, 0.0).into()),            
            transform: Transform::from_translation_rotation(Vec3::new(std::f32::MIN, std::f32::MIN, 0.0), Quat::from_rotation_z(0.0)),            
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(city::StraightStreet{
            ..Default::default()
        })
        .with(TempStraightStreet); 
}


fn road_network_change_tracking_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q1: Query<Mutated<roadsystem::RoadSystem>>,
) {
    for a in &mut q1.iter() {
        a.update(&mut commands, &mut materials);
        println!("{}", *a);
    }
    
}

fn build_street( 
    mut commands: Commands,    
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<input::MouseState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut street_query: Query<Without<TempStraightStreet, (&mut Sprite, &mut Transform, &mut city::StraightStreet)>>,
    mut temp_query: Query<With<TempStraightStreet, (Entity, &mut Sprite, &mut Transform, &mut city::StraightStreet)>>,
    mut intersection_query: Query<With<roadsystem::RoadIntersection, Entity>>,
    mut road_query: Query<With<roadsystem::Road, Entity>>,
    mut graph_query: Query<(&Graph, &mut roadsystem::RoadSystem)>
) {     
    let mouse_pos_ws = mouse_pos_ws(state.mouse_position);

    if mouse_button_input.just_pressed(MouseButton::Left) {
        state.last_mouse_left_pressed_position = mouse_pos_ws;

        spawn_temp_street(&mut commands, &mut materials);
    }

    let street_vector = mouse_pos_ws - state.last_mouse_left_pressed_position;
    let street_length = street_vector.length();
    let street_center = state.last_mouse_left_pressed_position + street_vector / street_length * street_length / 2.0;
    let rotation = -street_vector.angle_between(Vec2::new(1.0, 0.0)); 

    // update temp street length, orientation and position
    for (_, mut sprite, mut transform, mut temp_street) in &mut temp_query.iter() {
        *transform = Transform::from_translation_rotation(Vec3::new(street_center.x(), street_center.y(), 0.0), Quat::from_rotation_z(rotation));
        sprite.size.set_x(street_length);

        temp_street.set_start(street_center - street_vector.normalize() * street_length / 2.0);
        temp_street.set_end(street_center + street_vector.normalize() * street_length / 2.0); 
    }

    
    if mouse_button_input.just_released(MouseButton::Left) {
        // remove temp street entity
        for (entity, _, _, temp_street) in &mut temp_query.iter() { 
            commands.despawn(entity);
        }

        //let connected_streets = vec![new_street_entity];

        for (_, mut road_system) in &mut graph_query.iter() { 
            for entity in &mut intersection_query.iter() {
                commands.despawn(entity);
            }

            for entity in &mut road_query.iter() {
                commands.despawn(entity);
            }
            
            let node1_index = road_system.insert_intersection(roadsystem::RoadIntersection::new(state.last_mouse_left_pressed_position));
            let node2_index = road_system.insert_intersection(roadsystem::RoadIntersection::new(mouse_pos_ws));

            road_system.connect_intersections(node1_index, node2_index);

            println!("{}", intersection_query.iter().into_iter().len());
        }
        
        /*
        // Place Start Node
        let _start_intersection = commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),            
                transform: Transform::from_translation(Vec3::new(state.last_mouse_left_pressed_position.x(), state.last_mouse_left_pressed_position.y(), 2.0)),            
                sprite: Sprite::new(Vec2::new(40.0, 40.0)),
                ..Default::default()
            })
            .with(city::Intersection{
                connected_streets: connected_streets.clone()
            })
            .current_entity()
            .unwrap();

        let _end_intersection = commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),            
                transform: Transform::from_translation(Vec3::new(mouse_pos_ws.x(), mouse_pos_ws.y(), 2.0)),            
                sprite: Sprite::new(Vec2::new(40.0, 40.0)),
                ..Default::default()
            })
            .with(city::Intersection{
                connected_streets: connected_streets
            })
            .current_entity()
            .unwrap();
        */
    }       
}

struct StreetBuildingPlugin {
    //street_start: Vec2
}

impl Default for StreetBuildingPlugin {
    fn default() -> StreetBuildingPlugin {
        StreetBuildingPlugin {
            //street_start: Vec2::new(0.0, 0.0)
        }
    }
}

impl Plugin for StreetBuildingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(build_street.system()); 
    }    
}

/// Closes the application if Escape is pressed
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut exit_event: ResMut<Events<bevy::app::AppExit>>) {
    if keyboard_input.just_released(KeyCode::Escape) {
        exit_event.send(bevy::app::AppExit);
    }
}

struct Graph;

fn setup(
    mut commands: Commands,
) {
    commands
        // cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn((Graph, roadsystem::RoadSystem::new()));
}

fn main() {
    App::build()
    .add_resource(WindowDescriptor {
        title: "I am a window!".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        vsync: true,
        resizable: false,
        ..Default::default()
    })
    .add_default_plugins()    
    .add_plugin(StreetBuildingPlugin { ..Default::default() })
    .add_event::<bevy::app::AppExit>()
    .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .init_resource::<input::MouseState>()

    .add_system(keyboard_input_system.system())
    .add_system(input::print_mouse_events_system.system())
    .add_system(road_network_change_tracking_system.system())
    .add_startup_system(setup.system())
    .run();
}