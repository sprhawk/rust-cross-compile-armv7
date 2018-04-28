mod info;
mod shaders;

#[cfg(feature = "win")]
mod win;

#[cfg(feature = "fbdev")]
mod fbdev;

use std::sync::Arc;

#[allow(unused_imports)]
use vulkano::instance::{DeviceExtensions, Features, Instance, InstanceExtensions, Limits,
                        PhysicalDevice, QueueFamily};

use vulkano::device::{Device, Queue};

#[allow(unused_imports)]
use vulkano::format::{ClearValue, Format};

// use vulkano::image::{Dimensions, StorageImage};

use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};

#[allow(unused_imports)]
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, ImmutableBuffer};
use vulkano::framebuffer::{Framebuffer, Subpass};

use vulkano::instance::debug::DebugCallback;

use vulkano::sync::{now, GpuFuture};

use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;

use vulkano::swapchain;

use self::shaders::Vertex;

use self::shaders::default_fragment_shader::Shader as FragmentShader;
use self::shaders::default_vertex_shader::Shader as VertexShader;

#[cfg(feature = "win")]
use self::win::{create_swapchain, required_extensions, run_loop};

#[cfg(feature = "fbdev")]
use self::fbdev::{create_swapchain, required_extensions, run_loop};

/*
use ::image;
#[allow(unused_imports)]
use image::{ImageBuffer, Rgba};
*/

pub struct VulkanStruct {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub vertex_shader: VertexShader,
    pub fragment_shader: FragmentShader,
}

pub fn create_vk_instance() -> Arc<Instance> {
    let app_info = app_info_from_cargo_toml!();
    // println!("Application Info:{:?}", app_info);
    let extensions = required_extensions();
    Instance::new(Some(&app_info), &extensions, None).expect("failed to create Vulkan instance")
}

fn create_vk_struct() -> Arc<VulkanStruct> {
    #[cfg(feature = "win")]
    let instance = create_vk_instance();

    #[cfg(feature = "fbdev")]
    let instance = create_vk_instance();

    let _callback = DebugCallback::errors_and_warnings(&instance, |msg| {
        println!("Vulkan Debug: {:?}", msg.description);
    }).ok();

    info::print_vk_info(&instance);

    let physical_device = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No device available");

    let queue_family = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("No graphical queue family");

    let (device, mut queues) = {
        let ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };

        Device::new(
            physical_device,
            &Features::none(),
            &ext,
            [(queue_family, 0.5)].iter().cloned(),
        ).expect("failed to create device")
    };

    let queue = queues.next().expect("No queues are found");

    let vertex_shader = shaders::default_vertex_shader::Shader::load(device.clone())
        .expect("Failed to create vertex shader module");
    let fragment_shader = shaders::default_fragment_shader::Shader::load(device.clone())
        .expect("Failed to create fragment shader module");

    let vs = VulkanStruct {
        device: device,
        queue: queue,
        vertex_shader: vertex_shader,
        fragment_shader: fragment_shader,
    };

    return Arc::<_>::new(vs);
}

pub fn run() {
    let vulkan_obj = create_vk_struct();

    info::print_all_displays(vulkan_obj.device.physical_device());
    info::print_all_display_plane(vulkan_obj.device.physical_device());

    let (swap_chain, images) = create_swapchain(vulkan_obj.clone()).unwrap();

    /*
    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d {
            width: 1024,
            height: 1024,
        },
        Format::R8G8B8A8Unorm,
        Some(queue_family),
    ).unwrap();
    */

    let (image_index, swapchain_acquire_future) =
        swapchain::acquire_next_image(swap_chain.clone(), None).unwrap();
    let image = images[image_index].clone();

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };

    let render_pass = Arc::new(
        single_pass_renderpass!(vulkan_obj.device.clone(),
    attachments: {
        color: {
            load: Clear,
            store: Store,
            // format: Format::B8G8R8A8Srgb,
            // R8G8B8A8Unorm is not supported under Linux Intel driver
            // format: Format::R8G8B8A8Unorm,
            format: Format::B8G8R8A8Unorm,
            samples: 1,
        }
    },
    pass: {
        color: [color],
        depth_stencil: {}
    }).unwrap(),
    );

    let framebuffer = Arc::new(
        Framebuffer::start(render_pass.clone())
            .add(image.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vulkan_obj.vertex_shader.main_entry_point(), ())
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(vulkan_obj.fragment_shader.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(vulkan_obj.device.clone())
            .unwrap(),
    );

    let dynamic_state = DynamicState {
        viewports: Some(vec![Viewport {
            origin: [0.0, 0.0],
            dimensions: [1024.0, 1024.0],
            depth_range: 0.0..1.0,
        }]),
        ..DynamicState::none()
    };

    let vertices = vec![vertex1, vertex2, vertex3];
    let (vertex_buffer, vertex_buffer_future) = ImmutableBuffer::from_iter(
        vertices.into_iter(),
        BufferUsage::all(),
        vulkan_obj.queue.clone(),
    ).unwrap();

    let f = vertex_buffer_future.then_signal_fence_and_flush().unwrap();
    f.wait(None).unwrap();

    let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(vulkan_obj.device.clone(), vulkan_obj.queue.family())
            .unwrap()
            .begin_render_pass(
                framebuffer.clone(),
                false,
                vec![[0.0, 0.0, 1.0, 1.0].into()],
            )
            .unwrap()
            .draw(
                pipeline.clone(),
                dynamic_state,
                vertex_buffer.clone(),
                (),
                (),
            )
            .unwrap()
            .end_render_pass()
            .unwrap()
            // .copy_image_to_buffer(image.clone(), buffer.clone())
            // .unwrap()
            .build()
            .unwrap();

    let mut previous_frame_end = Box::new(now(vulkan_obj.device.clone())) as Box<GpuFuture>;

    previous_frame_end.cleanup_finished();

    let future = previous_frame_end
        .join(swapchain_acquire_future)
        .then_execute(vulkan_obj.queue.clone(), command_buffer)
        .unwrap()
        .then_swapchain_present(vulkan_obj.queue.clone(), swap_chain.clone(), image_index)
        .then_signal_fence_and_flush();

    let _future = future.unwrap();
    // swapchain::present(swap_chain, finished, queue.clone(), image_index);

    #[cfg(feature = "win")]
    run_loop(&mut events_loop);

    #[cfg(feature = "fbdev")]
    run_loop();
}
