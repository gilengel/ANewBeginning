use bevy::prelude::*;

struct SingleActionSelection;
trait UiWidget<T> {
    fn create(&self, child_builder: &mut ChildBuilder, materials: &Res<ButtonMaterials>, asset_server: &Res<AssetServer>);
}

trait UiContainerWidget<T> {
    fn create(&self, commands: &mut Commands, materials: &Res<ButtonMaterials>, asset_server: &Res<AssetServer>);
}


impl UiContainerWidget<RoadActions> for SingleActionSelection{
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
    let action_container = SingleActionSelection;
    action_container.create(&mut commands, &button_materials, &asset_server);
}