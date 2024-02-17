use bevy_app::{App, Plugin};
use bevy_asset::{load_internal_asset, Handle};
use bevy_core_pipeline::core_2d::graph::{Core2d, Node2d};
use bevy_render::{
    render_graph::{RenderGraphApp, ViewNodeRunner},
    render_resource::Shader,
    RenderApp,
};

use self::render::{LightingLabel, LightingNode};

pub mod components;
pub mod pipeline;
pub mod render;

pub const JFA_SDF_SHADER: Handle<Shader> = Handle::weak_from_u128(84363212315634875452134385435643);

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            JFA_SDF_SHADER,
            "shaders/jfa_sdf.wgsl",
            Shader::from_wgsl
        );

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<LightingNode>>(Core2d, LightingLabel)
            .add_render_graph_edges(Core2d, (Node2d::MainPass, LightingLabel, Node2d::Bloom))
            .add_render_graph_edges(
                Core2d,
                (Node2d::MainPass, LightingLabel, Node2d::Tonemapping),
            );
        // TODO add edge (MainPass - LightingLabel - UiPass) when using bevy_ui
    }
}
