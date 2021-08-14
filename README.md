# ash_shader_creator

A library for easy to way automatically create multiple shader stages from the directory path.

```rust
let shader_stage_instance: Vec<PipelineShaderStageCreateInfo> =
    ShaderStage::new(&device, &Path::new("example_path/compiled_shaders"))
        .with_shader_stage_flags(shader_stage_flags)
        .build();
```

### What the library can do?

- [x] Supports GLSL
- [ ] Supports HLSL
- [ ] Creating shaders from multiple directories