use bevy::prelude::*;

pub struct Action<T> {
    what: T,
    text: String,
    description: String
}

pub struct SingleActionSelection<T> {
    pub actions: Vec<Action<T>>
}

impl SingleActionSelection<RoadActions> {
    pub fn new() -> Self {
        SingleActionSelection{
            actions: Vec::new()
        }
    }
}

impl Action<RoadActions> {
    pub fn new(what: RoadActions, text: String, description: String) -> Self {
        Action {
            what: what,
            text: text,
            description: description
        }
    }
}

trait UiWidget<T> {
    fn create(&self, child_builder: &mut ChildBuilder, materials: &Res<ButtonMaterials>, asset_server: &Res<AssetServer>);
}

trait UiContainerWidget<T> {
    fn create(&self, commands: &mut Commands, materials: &Res<ButtonMaterials>, asset_server: &Res<AssetServer>);
}

/*
impl UiWidget<RoadActions> for Action<RoadActions> {
    fn create(&self, value:child_builder: &mut ChildBuilder, materials: &Res<ButtonMaterials>, asset_server: &Res<AssetServer>) {
        child_builder.spawn(
            ButtonComponents {
                style: button_style(),
                material: materials.normal.clone(),
                ..Default::default()
            })
            .with(ToggleButton { 
                state: ToggleState::Normal
            })
            .with(RoadActions)
            .with_children(|parent| {
                parent.spawn(TextComponents {
                    text: button_text(self.text.to_string(), asset_server),
                    ..Default::default()
                });
            });        
    }
}
*/

fn button_style() -> Style {
    Style {
        size: Size::new(Val::Px(80.0), Val::Px(45.0)),
        margin: Rect::all(Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    }
}

fn button_text(text: String, asset_server: &Res<AssetServer>) -> Text {
    Text {
        value: text.to_string(),
        font: asset_server.load("fonts/FiraSans-Bold.ttf").unwrap(),
        style: TextStyle {
            font_size: 16.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    }
}

impl UiContainerWidget<RoadActions> for SingleActionSelection<RoadActions> {
    fn create(&self, commands: &mut Commands, materials: &Res<ButtonMaterials>, asset_server: &Res<AssetServer>) {
        commands.spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(30.0), Val::Px(60.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,                
                ..Default::default()
            },
            material: materials.background.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            icon_toggle_button(RoadActions::Build, "Build", parent, materials, asset_server);
            icon_toggle_button(RoadActions::Demolish, "Remove", parent, materials, asset_server);
        });
    }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum RoadActions {
    Nothing,
    Build,
    Demolish
}

pub enum ToggleState {
    Normal,
    Toggled
}

pub struct ToggleButton {
    pub state: ToggleState
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
    pub background: Handle<ColorMaterial>
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            background: materials.add(Color::rgb(0.3, 0.4, 0.8).into()),
        }
    }
}

fn icon_toggle_button(value: RoadActions, text: &str, child_builder: &mut ChildBuilder, button_materials: &Res<ButtonMaterials>, asset_server: &Res<AssetServer>) {
    child_builder.spawn(
    ButtonComponents {
        style: Style {
            size: Size::new(Val::Px(80.0), Val::Px(45.0)),
            // center button
            margin: Rect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button_materials.normal.clone(),
        ..Default::default()
    })
    .with(ToggleButton { 
        state: ToggleState::Normal
    })
    .with(value)
    .with_children(|parent| {
        parent.spawn(TextComponents {
            text: Text {
                value: text.to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf").unwrap(),
                style: TextStyle {
                    font_size: 16.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            },
            ..Default::default()
        });
    });
}

pub fn ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>
) {
    let action_container = SingleActionSelection::new();
    action_container.create(&mut commands, &button_materials, &asset_server);
}