extern crate three;

use std::env;
use three::Object;

use std::path::Path;


use rapier3d::dynamics::{JointSet, RigidBodySet, IntegrationParameters, BodyStatus, RigidBodyBuilder};
use rapier3d::geometry::{BroadPhase, NarrowPhase, ColliderSet, ColliderBuilder, ColliderShape, Ball};
use rapier3d::pipeline::PhysicsPipeline;
use rapier3d::na::{Vector3, Isometry3};


fn main() {



    let mut args = env::args();

    let mut win = three::Window::new("Bruh Engine 3");
    let cam = win.factory.perspective_camera(60.0, 1.0 .. 1000.0);
    let mut controls = three::controls::FirstPerson::builder(&cam)
        .position([0.0, 2.0, -5.0])
        //.target([0.0, 0.0, 0.0])
        .build();

    let model_roots: Vec<three::Group> = Vec::new();


    let dir_light = win.factory.directional_light(0xffffff, 0.9);
    dir_light.look_at([15.0, 35.0, 35.0], [0.0, 0.0, 2.0], None);
    win.scene.add(&dir_light);

    win.scene.background = three::Background::Color(0xC6F0FF);

    let root = win.factory.group();
    win.scene.add(&root);



    let obj_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/Ak47.obj");
    let npath = args.nth(1).unwrap_or(obj_path.into());

    let (mut group_map, _meshes) = win.factory.load_obj(&npath);
    for g in group_map.values_mut() {
        root.add(g);
    }




    
    // Here the gravity is -9.81 along the y axis.
    let mut pipeline = PhysicsPipeline::new();
    let gravity = Vector3::new(0.0, -9.81, 0.0);
    let integration_parameters = IntegrationParameters::default();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut joints = JointSet::new();
    // We ignore contact events for now.
    let event_handler = ();



    let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .translation(0.0, 5.0, 1.0)
        .rotation(Vector3::z() * 5.0)
        .position(Isometry3::new(Vector3::new(1.0, 3.0, 2.0), Vector3::y() * 0.4))
        .linvel(1.0, 3.0, 4.0)
        .angvel(Vector3::x() * 3.0)
        .can_sleep(true)
        .build();

    let rigid_handle = bodies.insert(rigid_body);


    let collider = ColliderBuilder::new(ColliderShape::ball(0.5))
        .translation(1.0, 2.0, 3.0)
        .rotation(Vector3::y())
        .density(1.3)
        .friction(0.8)
        .sensor(true)
        .build();

    let collider_handle = colliders.insert(collider,rigid_handle,&mut bodies);
    


    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        controls.update(&win.input);
        win.render(&cam);

        
        pipeline.step(
            &gravity,
            &integration_parameters,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
            None,
            None,
            &event_handler
        );
        
    }


}














