pub fn add_obj(model_roots: &Vec<three::Group>, win: &three::Window, path: &str) {

	let obj_path = concat!(env!("CARGO_MANIFEST_DIR"), &path);
    let npath = args.nth(1).unwrap_or(obj_path.into());

	let (mut group_map, _meshes) = win.factory.load_obj(&npath);
    for g in group_map.values_mut() {
        root.add(g);
    }


}


