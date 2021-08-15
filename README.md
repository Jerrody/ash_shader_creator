[![](https://img.shields.io/badge/github.com-24292E?style=for-the-badge&logo=github)](https://github.com/Jerrody/ash_shader_creator) ![](https://img.shields.io/tokei/lines/github/Jerrody/ash_shader_creator?style=for-the-badge) ![](https://img.shields.io/github/last-commit/Jerrody/ash_shader_creator?style=for-the-badge) ![](https://img.shields.io/github/license/Jerrody/ash_shader_creator?style=for-the-badge)
![](https://img.shields.io/github/issues/Jerrody/ash_shader_creator?style=for-the-badge) ![](https://img.shields.io/github/issues-closed/Jerrody/ash_shader_creator?style=for-the-badge) ![](https://img.shields.io/crates/v/ash_shader_creator?label=version&style=for-the-badge) [![](https://img.shields.io/badge/crates.io-E6B14C?style=for-the-badge&logo=rust&logoColor=000000)](https://crates.io/crates/ash_shader_creator)

# ash_shader_creator

A library for easy to way automatically create multiple shader stages from the directory path.

```rust
use ash::{Device, PipelineShaderStageCreateFlags, PipelineShaderStageCreateInfo};
use std::path::Path;

let shader_stage_flags = PipelineShaderStageCreateFlags::RESERVED_2_NV | PipelineShaderStageCreateFlags::ALLOW_VARYING_SUBGROUP_SIZE_EXT;
let shader_stages_create_info: Vec<PipelineShaderStageCreateInfo> =
    ShaderStage::new(&device, &Path::new("example_path/compiled_shaders"))
        .with_shader_stage_flags(shader_stage_flags)
        .build();
```

### What the library can do?

- [x] Supports GLSL
- [x] Supports HLSL
- [ ] Creating shaders from multiple directories

### Important

For today, library can creates only shader stages from the names of compiled shaders that have:
- For the GLSL: <file_name>.vert.spv for the vertex shader and <file_name>.frag.spv for the fragment shader.
- For the HLSL: <file_name>.vs for the vertex shader and <file_name>.fs for the fragment shader.

#### Contacts
Discord: Жоржик#1991