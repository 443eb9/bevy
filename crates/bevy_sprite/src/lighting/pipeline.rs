use bevy_ecs::{
    system::Resource,
    world::{FromWorld, World},
};
use bevy_render::{
    render_resource::{
        BindGroupLayout, BindGroupLayoutEntry, BindingType, SamplerBindingType, ShaderStages,
        StorageTextureAccess, TextureFormat, TextureSampleType, TextureViewDimension,
    },
    renderer::RenderDevice,
    texture::BevyDefault,
};

#[derive(Resource)]
pub struct LightingPipeline {
    pub screen_layout: BindGroupLayout,
}

impl FromWorld for LightingPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let screen_layout = render_device.create_bind_group_layout(
            Some("lighting_2d_screen_layout"),
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadWrite,
                        format: TextureFormat::bevy_default(),
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        );

        Self { screen_layout }
    }
}
