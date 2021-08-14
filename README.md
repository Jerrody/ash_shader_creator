# ash_shader_creator

A library for easy to way automatically create multiple shader stages from the directory path.

```rust
use ash::Device;
use std::path::Path;

let shader_stage_flags = PipelineShaderStageCreateFlags::RESERVED_2_NV | PipelineShaderStageCreateFlags::ALLOW_VARYING_SUBGROUP_SIZE_EXT;
let shader_stages_create_info: Vec<PipelineShaderStageCreateInfo> =
    ShaderStage::new(&device, &Path::new("example_path/compiled_shaders"))
        .with_shader_stage_flags(shader_stage_flags)
        .build();
```

### What the library can do?

- [x] Supports GLSL
- [ ] Supports HLSL
- [ ] Creating shaders from multiple directories