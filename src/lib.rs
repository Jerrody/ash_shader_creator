//! # ash_shader_creator
//!
//! A library for easy to way automatically create multiple shader stages from the directory path.

use std::{
    collections::HashMap,
    ffi::{c_void, CString},
    fs::{read_dir, File},
    io::Read,
    path::{Path, PathBuf},
    ptr,
};

use ash::{
    vk::{
        PipelineShaderStageCreateFlags, PipelineShaderStageCreateInfo, ShaderModule,
        ShaderModuleCreateFlags, ShaderModuleCreateInfo, ShaderStageFlags, SpecializationInfo,
        StructureType,
    },
    Device,
};

pub struct ShaderStage<'a> {
    pub device: &'a Device,
    pub dir_path: &'a Path,
    pub shader_flags: ShaderModuleCreateFlags,
    pub shader_p_next: *const c_void,
    pub main_function_name: CString,
    pub shader_stage_flags: PipelineShaderStageCreateFlags,
    pub shader_stage_p_next: *const c_void,
    pub spec_info: *const SpecializationInfo,
}

impl<'a> ShaderStage<'a> {
    /// Initiating the instance of ShaderStage struct, requires only the device and directory path, after that can be build shader stages.
    /// Can be customized flags and pointers to structs if it needed.
    /// # Examples
    ///
    /// ```
    /// use ash::Device;
    /// use std::path::Path;
    ///
    /// let shader_stage_flags = PipelineShaderStageCreateFlags::RESERVED_2_NV | PipelineShaderStageCreateFlags::ALLOW_VARYING_SUBGROUP_SIZE_EXT;
    /// let shader_stages_create_info: Vec<PipelineShaderStageCreateInfo> =
    ///    ShaderStage::new(&device, &Path::new("example_path/compiled_shaders"))
    ///        .with_shader_stage_flags(shader_stage_flags)
    ///        .build();
    /// ```
    pub fn new(device: &'a Device, dir_path: &'a Path) -> Self {
        Self {
            device,
            dir_path,
            shader_flags: ShaderModuleCreateFlags::empty(),
            shader_p_next: ptr::null(),
            shader_stage_flags: PipelineShaderStageCreateFlags::empty(),
            shader_stage_p_next: ptr::null(),
            spec_info: ptr::null(),
            main_function_name: CString::new("main").unwrap(),
        }
    }

    /// Specifies `ShaderModuleCreateFlags` for the `self.shader_flags` field.
    pub fn with_shader_flags(&mut self, shader_flags: ShaderModuleCreateFlags) {
        self.shader_flags = shader_flags;
    }

    /// Specifies `pointer` to the struct for the `self.shader_p_next` field.
    pub fn with_shader_p_next(&mut self, p_next: *const c_void) {
        self.shader_p_next = p_next;
    }

    /// Specifies `PipelineShaderStageCreateFlags` for the `self.shader_stage_flags` field.
    pub fn with_shader_stage_flags(&mut self, shader_stage_flags: PipelineShaderStageCreateFlags) {
        self.shader_stage_flags = shader_stage_flags;
    }

    /// Specifies `pointer` to the struct for the `self.shader_stage_p_next` field.
    pub fn with_shader_stage_p_next(&mut self, p_next: *const c_void) {
        self.shader_stage_p_next = p_next;
    }

    /// Specifies `SpecializationInfo` for the `self.spec_info` field.
    pub fn with_spec_info(&mut self, spec_info: *const SpecializationInfo) {
        self.spec_info = spec_info;
    }

    /// Specifies `main function name` for the `self.main_function_name` field.
    pub fn with_main_function_name(&mut self, main_function_name: &str) {
        self.main_function_name = CString::new(main_function_name).unwrap();
    }

    /// Consumes struct's `instance` and builds vector of shader stages.
    pub fn build(self) -> Vec<PipelineShaderStageCreateInfo> {
        let shader_modules = create_shader_modules(
            self.device,
            self.dir_path,
            self.shader_flags,
            self.shader_p_next,
        );

        let file_paths = read_dir(self.dir_path)
            .unwrap()
            .into_iter()
            .filter(|file_name| {
                file_name
                    .as_ref()
                    .unwrap()
                    .path()
                    .to_str()
                    .unwrap()
                    .contains(".spv")
            })
            .map(|path| path.unwrap().path());

        let shader_path: HashMap<&ShaderModule, PathBuf> =
            shader_modules.iter().zip(file_paths.into_iter()).collect();

        shader_modules
            .iter()
            .map(|module| {
                let path = shader_path.get(&module).unwrap().to_str().unwrap();

                PipelineShaderStageCreateInfo {
                    s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                    p_next: self.shader_stage_p_next,
                    flags: self.shader_stage_flags,
                    stage: if path.contains("vert.spv") || path.contains(".vs") {
                        ShaderStageFlags::VERTEX
                    } else if path.contains("frag.spv") || path.contains(".fs") {
                        ShaderStageFlags::FRAGMENT
                    } else {
                        panic!("Failed to define shader type!")
                    },
                    module: *module,
                    p_name: self.main_function_name.as_ptr(),
                    p_specialization_info: self.spec_info,
                }
            })
            .collect()
    }
}

fn create_shader_modules(
    device: &Device,
    dir_path: &Path,
    flags: ShaderModuleCreateFlags,
    p_next: *const c_void,
) -> Vec<ShaderModule> {
    let compiled_shader_path =
        read_dir(dir_path).unwrap_or_else(|_| panic!("Failed to find spv file at {:?}", dir_path));
    let files_path_buf: Vec<PathBuf> = compiled_shader_path
        .into_iter()
        .filter(|file_name| {
            let file_name = file_name
                .as_ref()
                .unwrap()
                .path()
                .to_str()
                .unwrap()
                .to_owned();

            file_name.contains(".spv") || file_name.contains(".vs") || file_name.contains(".fs")
        })
        .map(|compiled_shader| compiled_shader.unwrap().path())
        .collect();

    let files = files_path_buf.iter().map(|path_buf| {
        File::open(path_buf).unwrap_or_else(|_| panic!("Failed to find compiled shader file at {:?}", path_buf))
    });

    let shader_code = files.map(|file| {
        file.bytes()
            .filter_map(|byte| byte.ok())
            .collect::<Vec<u8>>()
    });

    shader_code
        .map(|shader_code| {
            let shader_module_create_info = ShaderModuleCreateInfo {
                s_type: StructureType::SHADER_MODULE_CREATE_INFO,
                p_next,
                flags,
                code_size: shader_code.len(),
                p_code: shader_code.as_ptr() as *const u32,
            };

            unsafe {
                device
                    .create_shader_module(&shader_module_create_info, None)
                    .expect("Failed to create shader module!")
            }
        })
        .collect::<Vec<ShaderModule>>()
}
