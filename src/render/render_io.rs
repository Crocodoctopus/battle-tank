use std::collections::HashMap;
use std::ops::Index;

// loads all textures from the top level of a directory
pub(super) fn load_textures_from(
    path: &'static str,
) -> impl Index<&'static str, Output = ezgl::Texture2D> {
    // load textures
    use std::iter::FromIterator;
    HashMap::<String, _>::from_iter(
        std::fs::read_dir(crate::io::get_root().join(path))
            .unwrap()
            .filter_map(|direntry_res| {
                let path = direntry_res.ok()?.path();
                let name = path.file_name()?.to_string_lossy().into_owned();
                let texture = ezgl::Texture2D::from_file(&path).ok()?;
                println!("Texture loaded: {}", name);
                Some((name, texture))
            }),
    )
}

// loads all glsl programs from the top level of a directory
pub(super) fn load_programs_from(
    path: &'static str,
) -> impl Index<&'static str, Output = ezgl::Program> {
    // load programs
    let mut program_builders = HashMap::new();
    use std::iter::FromIterator;
    std::fs::read_dir(crate::io::get_root().join(path))
        .unwrap()
        .filter_map(|direntry_res| direntry_res.ok())
        .for_each(|direntry| {
            let stem = direntry
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let path = direntry.path();

            let shader = ezgl::Shader::from_file(&path).unwrap();
            println!("Shader loaded: {:?}", path);

            let (k, v) = program_builders
                .remove_entry(&stem)
                .unwrap_or_else(|| (stem, ezgl::ProgramBuilder::new()));
            let (k, v) = (k, v.with(shader));
            program_builders.insert(k, v);
        });
    HashMap::<String, _>::from_iter(program_builders.into_iter().map(|(k, v)| {
        let v = v.build().unwrap();
        println!("Program linked: {}", k);
        (k, v)
    }))
}
