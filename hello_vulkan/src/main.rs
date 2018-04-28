#[macro_use]
extern crate vulkano;

extern crate image;
extern crate vulkano_win;
extern crate winit;

#[macro_use]
extern crate vulkano_shader_derive;

use std::sync::Arc;

#[allow(unused_imports)]
use vulkano_win::VkSurfaceBuild;

#[allow(unused_imports)]
use vulkano::instance::{DeviceExtensions, Features, Instance, InstanceExtensions, Limits,
                        PhysicalDevice, QueueFamily};

use vulkano::device::Device;

use vulkano::format::{ClearValue, Format};

use vulkano::image::{Dimensions, StorageImage};

use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer, DynamicState};

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::framebuffer::{Framebuffer, Subpass};

use vulkano::instance::debug::DebugCallback;

use vulkano::sync::GpuFuture;

use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;

use image::{ImageBuffer, Rgba};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
impl_vertex!(Vertex, position);

#[allow(dead_code)]
mod default_vertex_shader {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
    #version 450
    layout(location = 0) in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "]
    #[allow(dead_code)]
    struct Dummy;
}
#[allow(dead_code)]
mod default_fragment_shader {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
    #version 450
    layout(location = 0) out vec4 f_color;

    void main() {
        f_color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "]
    #[allow(dead_code)]
    struct Dummy;
}
fn main() {
    println!("Hello, Vulkan!");

    let instance = {
        let app_info = app_info_from_cargo_toml!();
        // println!("Application Info:{:?}", app_info);
        let extensions = vulkano_win::required_extensions();
        Instance::new(Some(&app_info), &extensions, None).expect("failed to create Vulkan instance")
    };

    let _callback = DebugCallback::errors_and_warnings(&instance, |msg| {
        println!("Vulkan Debug: {:?}", msg.description);
    }).ok();

    // print_vk_info(&instance);

    let physical_device = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No device available");

    let queue_family = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("No graphical queue family");

    let (device, mut queues) = {
        Device::new(
            physical_device,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        ).expect("failed to create device")
    };

    let queue = queues.next().expect("No queues are found");

    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d {
            width: 1024,
            height: 1024,
        },
        Format::R8G8B8A8Unorm,
        Some(queue_family),
    ).unwrap();

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };

    let vertex_shader = default_vertex_shader::Shader::load(device.clone())
        .expect("Failed to create vertex shader module");
    let fragment_shader = default_fragment_shader::Shader::load(device.clone())
        .expect("Failed to create fragment shader module");

    let render_pass = Arc::new(
        single_pass_renderpass!(device.clone(),
    attachments: {
        color: {
            load: Clear,
            store: Store,
            format: Format::R8G8B8A8Unorm,
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
            .vertex_shader(vertex_shader.main_entry_point(), ())
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fragment_shader.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    let dynamic_state = DynamicState {
        viewports: Some(vec![
            Viewport {
                origin: [0.0, 0.0],
                dimensions: [1024.0, 1024.0],
                depth_range: 0.0..1.0,
            },
        ]),
        ..DynamicState::none()
    };

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        vec![vertex1, vertex2, vertex3].into_iter(),
    ).unwrap();

    let buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        (0..1024 * 1024 * 4).map(|_| 0u8),
    ).expect("Failed to create buffer");

    let command_buffer =
        AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
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
            ).unwrap()
            .end_render_pass()
            .unwrap()
            .copy_image_to_buffer(image.clone(), buffer.clone())
            .unwrap()
            .build()
            .unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let buffer_content = buffer.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();

    /*
    let mut events_loop = winit::EventsLoop::new();
    let _window = winit::WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, ..} => {
                winit::ControlFlow::Break
            },
            _ => winit::ControlFlow::Continue,
        }
    });
    */
}

#[allow(dead_code)]
fn print_vk_info(instance: &Arc<Instance>) {
    print_instance_extensions();
    print_layers();

    for device in PhysicalDevice::enumerate(instance) {
        print_vk_physical_device(&device);
    }
}

fn print_instance_extensions() {
    let exts = InstanceExtensions::supported_by_core().expect("No instance extensions");
    println!("Instance extensions:");
    print!("khr_surface:{} khr_display:{} khr_xlib_surface: {} khr_xcb_surface: {} \
    khr_wayland_surface: {} khr_mir_surface: {} khr_android_surface: {} khr_win32_surface: {} \
    ext_debug_report: {} mvk_ios_surface: {} mvk_macos_surface: {} mvk_moltenvk: {} nn_vi_surface: {} \
    ext_swapchain_colorspace: {} khr_get_phyiscal_device_properties2: {}",
    exts.khr_surface, exts.khr_display, exts.khr_xlib_surface, exts.khr_xcb_surface,
    exts.khr_wayland_surface, exts.khr_mir_surface, exts.khr_android_surface, exts.khr_win32_surface,
    exts.ext_debug_report, exts.mvk_ios_surface, exts.mvk_macos_surface, exts.mvk_moltenvk, exts.nn_vi_surface,
    exts.ext_swapchain_colorspace, exts.khr_get_physical_device_properties2);
    println!("");
}

fn print_physical_device_extensions(device: &PhysicalDevice) {
    let exts = DeviceExtensions::supported_by_device(*device);
    println!("Device extensions:");
    print!(
        "khr_swapchain: {} khr_display_swapchain: {} khr_sampler_mirror_clamp_to_edge: {} \
         khr_maintenance1: {} khr_get_memory_requirements: {} khr_dedicated_allocation: {} \
         khr_incremental_present: {} ext_debug_marker: {}",
        exts.khr_swapchain,
        exts.khr_display_swapchain,
        exts.khr_sampler_mirror_clamp_to_edge,
        exts.khr_maintenance1,
        exts.khr_get_memory_requirements2,
        exts.khr_dedicated_allocation,
        exts.khr_incremental_present,
        exts.ext_debug_marker
    );
}

fn print_layers() {
    if let Ok(layers_list) = vulkano::instance::layers_list() {
        println!("Available layers:");
        for layer in layers_list {
            println!("{} : {}", layer.name(), layer.description());
        }
    }
    println!("");
}

fn print_vk_physical_device(device: &PhysicalDevice) {
    print!("Device Info: ");
    print!("Name:{} ", device.name());
    print!("Type:{:?}", device.ty());
    print!("\n");
    print!(
        "Api: {:?} Driver: {}",
        device.api_version(),
        device.driver_version()
    );
    print!("\n");
    println!("Supported Features:");
    println!("{:?}", device.supported_features());
    println!("Queue families:");
    for queue_family in device.queue_families() {
        println!(
            "queue {}: count: {} graphics:{} compute:{} transfers:{} sparse_bind:{}",
            queue_family.id(),
            queue_family.queues_count(),
            queue_family.supports_graphics(),
            queue_family.supports_compute(),
            queue_family.supports_transfers(),
            queue_family.supports_sparse_binding()
        );
    }

    for mem_type in device.memory_types() {
        println!("memtype {}: local:{}, host_visible: {}, host_coherent: {}, host_cached: {}, lazily_allocated: {}", 
        mem_type.id(),
        mem_type.is_device_local(),
        mem_type.is_host_visible(),
        mem_type.is_host_coherent(),
        mem_type.is_host_cached(),
        mem_type.is_lazily_allocated()
        );
    }

    for mem_heap in device.memory_heaps() {
        println!(
            "memheap {}: size: {}, local: {}",
            mem_heap.id(),
            mem_heap.size(),
            mem_heap.is_device_local()
        );
    }

    let lim = device.limits();
    println!("Limits:");
    println!("max_image_dimension_1d: {}", lim.max_image_dimension_1d());
    println!("max_image_dimension_2d: {}", lim.max_image_dimension_2d());
    println!("max_image_dimension_3d: {}", lim.max_image_dimension_3d());
    println!(
        "max_image_dimension_cube: {}",
        lim.max_image_dimension_cube()
    );

    print_physical_device_extensions(device);
}
