use amethyst::{
    input::{InputHandler, StringBindings, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    assets::{AssetStorage, Handle, Loader},
    core::{
        geometry::Plane,
        math::{Point2, Vector2, Vector3},
        transform::{Transform},
        Named, WithNamed,
        Time,
        
    },
    ecs::{
        prelude::Entity, Entities, Join, Read, ReadExpect, ReadStorage, Write, System,
        WriteStorage,
    },
    renderer::{
        camera::{ActiveCamera, Camera},
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        ImageFormat, Texture,   

        debug_drawing::{DebugLines, DebugLinesParams},

    },
    ui::{ UiCreator, UiEvent, UiEventType, UiFinder, UiText },
    utils::fps_counter::FpsCounter,
    window::ScreenDimensions,    
    GameData, SimpleState, SimpleTrans, StateData, Trans,
    winit::MouseButton
};

#[derive(Debug)]
enum BuildingState {
    None,
    //BuildRoad,
    //DemolishRoad
}

/// Resource holding the current cursor position in world space
/// Defaults to i16::MAX, i16::MAX if not set
pub struct CursorPositionInWorldSpace {
    pub cursor_position: Point2<f32>
}

impl Default for CursorPositionInWorldSpace {
    fn default() -> CursorPositionInWorldSpace { CursorPositionInWorldSpace { cursor_position: Point2::new(std::f32::MAX, std::f32::MAX) } }
}

impl Default for BuildingState {         
    fn default() -> Self { BuildingState::None }
}

/// A dummy game state that shows 3 sprites.
#[derive(Default, Debug)]
pub struct GameState {
    current_building_state: BuildingState,
    ui_root: Option<Entity>,
    fps_display: Option<Entity>,
    button_create_road: Option<Entity>,
    button_demolish_road: Option<Entity>,

    handle: Option<Handle<SpriteSheet>>,
}

#[derive(Default, Debug)]
pub struct BuildRoadState {
    button_create_road: Option<Entity>,
    button_demolish_road: Option<Entity>,

    road_start: Option<Point2<f32>>,
    handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for BuildRoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Welcome to road building :)");


        self.handle = Some(load_sprite_sheet(
            data.world,
            "texture/cp437_20x20.png",
            "texture/cp437_20x20.ron",
        ));
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let world = data.world;

        match event {
            StateEvent::Window(event) => {
                // Check if the window should be closed
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit;
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit;
                } else {
                    Trans::None
                }
            },

            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_create_road {
                    return Trans::Pop;
                }

                Trans::None
            },

            StateEvent::Input(amethyst::input::InputEvent::MouseButtonPressed(MouseButton::Left)) => {
                self.road_start = Some(world.read_resource::<CursorPositionInWorldSpace>().cursor_position);

                Trans::None
            },

            StateEvent::Input(amethyst::input::InputEvent::MouseButtonReleased(MouseButton::Left)) => {
                let cursor_pos_ws = world.read_resource::<CursorPositionInWorldSpace>().cursor_position;

                if let Some(road_start) = self.road_start {     
                    let road_endpoints =  if cursor_pos_ws.y > road_start.y { (cursor_pos_ws, road_start) } else { (road_start, cursor_pos_ws) };           
                    let street_vector: Vector2<f32> = road_endpoints.1 - road_endpoints.0;
                    let street_length = street_vector.magnitude();
                    let street_center: Vector2<f32> = street_vector / street_length * (street_length / 2.0);
                    
                    
                    let rotation = -street_vector.angle(&Vector2::new(1.0, 0.0)); 

                    let mut transform = Transform::default();
                    transform.set_scale(Vector3::new(1.0 / 20.0 * street_length, 1.0, 1.0));
                    transform.set_translation(Vector3::new(
                        road_endpoints.0.x + street_center.x, 
                        road_endpoints.0.y + street_center.y, 
                        0.0)
                    );
                    transform.set_rotation_2d(rotation);
                    

                    let sprite_sheet_handle = self.handle.as_ref().unwrap();

                    init_sprite(Vector3::new(road_start.x, road_start.y, 0.0), "start", 7, world, sprite_sheet_handle);
                    init_sprite(Vector3::new(cursor_pos_ws.x, cursor_pos_ws.y, 0.0), "end", 7, world, sprite_sheet_handle);

                    let sprite = SpriteRender::new(sprite_sheet_handle.clone(), 12 * 16 + 4);
                    world
                        .create_entity()
                        .with(transform)
                        .with(sprite)
                        .named("Foo Street")
                        .build();
                
                    self.road_start = None;    
                }

                Trans::None
            }

            _ => Trans::None
        }
    }
}

const BUTTON_CREATE_ROAD: &str = "CreateRoad";
const BUTTON_DEMOLISH_ROAD: &str = "DemolishRoad";

//#[derive(SystemDesc)]
pub struct MouseRaycastSystem;

//#[derive(SystemDesc)]
pub struct MousePositionPrintSystem;

impl<'s> System<'s> for MousePositionPrintSystem  {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, CursorPositionInWorldSpace>
    );    

    fn run(
        &mut self,
        (
            cursor_pos_ws,
            input
        ): Self::SystemData,
    ) {
        /*
        if let Some(t) = ui_finder
            .find("mouse_position")
            .and_then(|e| ui_texts.get_mut(e)) 
        {
                t.text = format!("({:.0}, {:.0})", cursor_pos_ws.cursor_position.x, cursor_pos_ws.cursor_position.y);
            
            
        }
        */

        /*
                // Find any sprites which the mouse is currently inside
                let mut found_name = None;
                for (sprite, transform, name) in (&sprites, &transforms, &names).join() {
                    let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                    let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                    let (min_x, max_x, min_y, max_y) = {
                        // Sprites are centered on a coordinate, so we build out a bbox for the sprite coordinate
                        // and dimensions
                        // Notice we ignore z-axis for this example.
                        (
                            transform.translation().x - (sprite.width * 0.5),
                            transform.translation().x + (sprite.width * 0.5),
                            transform.translation().y - (sprite.height * 0.5),
                            transform.translation().y + (sprite.height * 0.5),
                        )
                    };
                    if mouse_world_position.x > min_x
                        && mouse_world_position.x < max_x
                        && mouse_world_position.y > min_y
                        && mouse_world_position.y < max_y
                    {
                        found_name = Some(&name.name);
                    }
                }

                if let Some(t) = ui_finder
                    .find("under_mouse")
                    .and_then(|e| ui_texts.get_mut(e))
                {
                    if let Some(name) = found_name {
                        t.text = format!("{}", name);
                    } else {
                        t.text = "".to_string();
                    }
                }
            }
            */        
    }
}
impl<'s> System<'s> for MouseRaycastSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, Named>,
        WriteStorage<'s, UiText>,
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
        UiFinder<'s>,
        Write<'s, CursorPositionInWorldSpace>
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            cameras,
            sprites,
            names,
            mut ui_texts,
            sprite_sheets,
            screen_dimensions,
            active_camera,
            input,
            ui_finder,
            mut cursor_pos_ws
        ): Self::SystemData,
    ) {
        // Get the mouse position if its available
        if let Some(mouse_position) = input.mouse_position() {
            // Get the active camera if it is spawned and ready
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                // Project a ray from the camera to the 0z axis
                let ray = camera.screen_ray(
                    Point2::new(mouse_position.0, mouse_position.1),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = *ray.at_distance(distance).xy();

                *cursor_pos_ws.cursor_position = mouse_world_position;

                // Find any sprites which the mouse is currently inside
                let mut found_name = None;
                for (sprite, transform, name) in (&sprites, &transforms, &names).join() {
                    let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                    let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                    let (min_x, max_x, min_y, max_y) = {
                        // Sprites are centered on a coordinate, so we build out a bbox for the sprite coordinate
                        // and dimensions
                        // Notice we ignore z-axis for this example.
                        (
                            transform.translation().x - (sprite.width * 0.5),
                            transform.translation().x + (sprite.width * 0.5),
                            transform.translation().y - (sprite.height * 0.5),
                            transform.translation().y + (sprite.height * 0.5),
                        )
                    };
                    if mouse_world_position.x > min_x
                        && mouse_world_position.x < max_x
                        && mouse_world_position.y > min_y
                        && mouse_world_position.y < max_y
                    {
                        found_name = Some(&name.name);
                    }
                }

                if let Some(t) = ui_finder
                    .find("under_mouse")
                    .and_then(|e| ui_texts.get_mut(e))
                {
                    if let Some(name) = found_name {
                        t.text = format!("{}", name);
                    } else {
                        t.text = "".to_string();
                    }
                }
            }
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.handle = Some(load_sprite_sheet(
            world,
            "texture/cp437_20x20.png",
            "texture/cp437_20x20.ron",
        ));

        let sprite_sheet_handle = self.handle.as_ref().unwrap();

        let _ = init_sprite(
            Vector3::new(0.0, 0.0, 0.0),
            "Sprite 1",
            7,
            world,
            &sprite_sheet_handle,
        );

        let _ = init_sprite(
            Vector3::new(100.0, 100.0, 0.0),
            "Sprite 2",
            7,
            world,
            &sprite_sheet_handle,
        );

        let _ = init_sprite(
            Vector3::new(-50.0, -50.0, 0.0),
            "Sprite 3",
            7,
            world,
            &sprite_sheet_handle,
        );

        self.ui_root =
        Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/game.ron", ())));

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let world = data.world;

        match event {
            StateEvent::Window(event) => {
                // Check if the window should be closed
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit;
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit;
                } else {
                    Trans::None
                }
            },
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_create_road {
                    return Trans::Push(Box::new(BuildRoadState {
                        button_create_road: self.button_create_road,
                        button_demolish_road: self.button_demolish_road,
                        ..Default::default()
                    }));
                    //log::info!("Trans::Switch Create Road");
                }

                if Some(target) == self.button_demolish_road {
                    log::info!("Trans::Switch Demolish Road");
                }

                Trans::None
            },

            _ => Trans::None
        }
    }


    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_create_road.is_none() || self.button_demolish_road.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_create_road = ui_finder.find(BUTTON_CREATE_ROAD);
                self.button_demolish_road = ui_finder.find(BUTTON_DEMOLISH_ROAD);
            });
        }

        if self.fps_display.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("fps") {
                    self.fps_display = Some(entity);
                }
            });
        }

        let mut ui_text = world.write_storage::<UiText>();
        if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
            if world.read_resource::<Time>().frame_number() % 20 == 0 {
                let fps = world.read_resource::<FpsCounter>().sampled_fps();
                fps_display.text = format!("FPS: {:.*}", 2, fps);
            }
        }

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }

        self.ui_root = None;
        self.button_create_road = None;
        self.button_demolish_road = None;
        self.fps_display = None;
    }    
}

// Initialize a sprite as a reference point at a fixed location
fn init_sprite(
    position: Vector3<f32>,
    name: &'static str,
    sprite_number: usize,
    world: &mut World,
    sprite_sheet: &Handle<SpriteSheet>,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation(position);

    let sprite = SpriteRender::new(sprite_sheet.clone(), sprite_number);
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .named(name)
        .build()
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    //transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);
    transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprite_sheet(
    world: &mut World,
    png_path: &str,
    ron_path: &str,
) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}