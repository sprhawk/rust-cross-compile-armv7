use ::vulkano;
use std::sync::Arc;
use vulkano::instance::{DeviceExtensions, Instance, InstanceExtensions,
                        PhysicalDevice};
use vulkano::swapchain::Capabilities;
use vulkano::swapchain::display::{Display, DisplayPlane};
                        
#[allow(dead_code)]
pub fn print_vk_info(instance: &Arc<Instance>) {
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
    println!("");
}

pub fn print_layers() {
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

pub fn print_all_displays(physical_device:PhysicalDevice) {
    println!("Displays:");
    for display in Display::enumerate(physical_device) {
        let dim = display.physical_dimensions();
        let resolution = display.physical_resolution();
        println!(
            "name: {} dimension({} x {}) resolution({} x {})",
            display.name(),
            dim[0],
            dim[1],
            resolution[0],
            resolution[1]
        );
        println!("modes:");

        for mode in display.display_modes() {
            let region = mode.visible_region();
            let rate = mode.refresh_rate();
            println!(
                "region({} x {}) refresh rate: {}",
                region[0], region[1], rate
            );
        }
    }
}

pub fn print_all_display_plane(physical_device : PhysicalDevice) {
    for plane in DisplayPlane::enumerate(physical_device) {
        println!("DisplayPlane: {}", plane.index());
    }
}

pub fn print_surface_capabilities(caps : Capabilities) {
    print!("Surface capabilities:");
    print!("min_image_count:({})", caps.min_image_count);
    print!("max_image_count({:?})", caps.max_image_count);
    if let Some(extent) = caps.current_extent {
        print!("current_extent({}x{})", extent[0], extent[1]);
    }
    print!("min_image_extent({}x{})", caps.min_image_extent[0], caps.min_image_extent[1]);
    print!("max_image_extent({}x{})", caps.max_image_extent[0], caps.max_image_extent[1]);
    print!("max_image_array_layers({})", caps.max_image_array_layers);
    print!("supprted_transform(");
    print!("identity:{} ", caps.supported_transforms.identity);
    print!("rotate90: {} ", caps.supported_transforms.rotate90);
    print!("rotate180: {} ", caps.supported_transforms.rotate180);
    print!("rotate279: {} ", caps.supported_transforms.rotate270);
    print!("horizontal_mirror: {} ", caps.supported_transforms.horizontal_mirror);
    print!("horizontal_mirror_rotate90: {} ", caps.supported_transforms.horizontal_mirror_rotate90);
    print!("horizontal_mirror_rotate180: {} ", caps.supported_transforms.horizontal_mirror_rotate180);
    print!("horizontal_mirror_rotate270: {} ", caps.supported_transforms.horizontal_mirror_rotate270);
    print!("inherit: {} ", caps.supported_transforms.inherit);

    print!("supported_format: {{");
    for f in caps.supported_formats.into_iter() {
        print!("({:?}, {:?}", f.0, f.1);
    }
    print!("}} ");

    print!("supported_present_mode: {{");
    print!("immediate: {} ", caps.present_modes.immediate);
    print!("fifo: {} ", caps.present_modes.fifo);
    print!("mailbox: {} ", caps.present_modes.mailbox);
    print!("relaxed: {}", caps.present_modes.relaxed);
    print!("}} ");

    print!("image_usage: {:?} ", caps.supported_usage_flags);
    print!("supported_composite_alpha: {:?}", caps.supported_composite_alpha);
    println!("");
}