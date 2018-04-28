use super::VulkanStruct;
#[allow(unused_imports)]
use vulkano_win::VkSurfaceBuild;
use winit;
use winit::EventsLoop;

use std::sync::Arc;
use vulkano_win;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::instance::InstanceExtensions;
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};

pub fn required_extensions() -> InstanceExtensions {
    let extensions = vulkano_win::required_extensions();
    extensions
}

pub fn create_swapchain(
    vulkan_obj: Arc<VulkanStruct>,
) -> Result<
    (
        Arc<Swapchain<winit::Window>>,
        Vec<Arc<SwapchainImage<winit::Window>>>,
        winit::EventsLoop,
    ),
    SwapchainCreationError,
> {
    let events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .build_vk_surface(
            &events_loop,
            vulkan_obj.device.physical_device().instance().clone(),
        )
        .unwrap();

    // if do not call is_supported, validation layer will report warnings
    let _r = window.is_supported(vulkan_obj.queue.family()).unwrap();

    let _win = window.window();
    let caps = window
        .capabilities(vulkan_obj.device.physical_device())
        .expect("failed to get surface capabalities");

    let dim = caps.current_extent.unwrap_or([1280, 1024]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;

    let (swap_chain, images) = Swapchain::new(
        vulkan_obj.device.clone(),
        window.clone(),
        caps.min_image_count,
        format,
        dim,
        1,
        caps.supported_usage_flags,
        &vulkan_obj.queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        true,
        None,
    ).expect("Failed to create swapchain");

    Ok((swap_chain, images, events_loop))
}

pub fn run_loop(events_loop: &mut EventsLoop) {
    events_loop.run_forever(|event| match event {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::Closed,
            ..
        } => winit::ControlFlow::Break,
        _ => winit::ControlFlow::Continue,
    });
}
