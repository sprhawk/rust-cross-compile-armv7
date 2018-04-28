use super::VulkanStruct;

use std::sync::Arc;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::instance::InstanceExtensions;
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError, Surface};

use vulkano::swapchain::display::{Display, DisplayPlane};
use vulkano::instance::PhysicalDevice;

use super::info::print_surface_capabilities;

pub fn required_extensions() -> InstanceExtensions {
    let extensions = InstanceExtensions {
        khr_display: true,
        ..InstanceExtensions::none()
    };
    extensions
}

pub fn create_swapchain(
    vulkan_obj: Arc<VulkanStruct>,
) -> Result<
    (
        Arc<Swapchain<()>>,
        Vec<Arc<SwapchainImage<()>>>
    ),
    SwapchainCreationError
> {
    let display = Display::enumerate(vulkan_obj.device.physical_device()).next().expect("There is no display available");
    let display_mode = display.display_modes().next().unwrap();
    let region = display_mode.visible_region();
    let display_plane = DisplayPlane::enumerate(vulkan_obj.device.physical_device()).next().expect("There is no display plane available");
    let surface = Surface::<()>::from_display_mode(&display_mode, &display_plane).expect("failed to create surface from display mode");
    let caps = surface.capabilities(vulkan_obj.device.physical_device()).expect("failed to get surface capabilities");

    print_surface_capabilities(caps.clone());

    let format = caps.supported_formats[0].0;
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    Swapchain::new(
        vulkan_obj.device.clone(),
        surface.clone(),
        caps.min_image_count,
        format,
        region,
        1,
        caps.supported_usage_flags,
        &vulkan_obj.queue,
        SurfaceTransform::Identity,
        alpha, 
        PresentMode::Fifo,
        true,
        None
    )
}

pub fn run_loop() {
    loop {}
}
