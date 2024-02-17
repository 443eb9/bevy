use bevy_ecs::{query::QueryItem, system::lifetimeless::Read, world::World};
use bevy_render::{
    render_graph::{NodeRunError, RenderGraphContext, RenderLabel, ViewNode},
    render_resource::ComputePassDescriptor,
    renderer::RenderContext,
    view::ViewTarget,
};

#[derive(Default)]
pub struct LightingNode;

#[derive(RenderLabel, Hash, PartialEq, Eq, Clone, Debug)]
pub struct LightingLabel;

impl ViewNode for LightingNode {
    type ViewQuery = Read<ViewTarget>;

    fn run<'w>(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        view_target: QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        // let post_process = view_target.post_process_write();
        // render_context
        //     .command_encoder()
        //     .begin_compute_pass(&ComputePassDescriptor::default());

        Ok(())
    }
}
