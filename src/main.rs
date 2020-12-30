extern crate three;

use std::env;
use three::Object;

extern crate nalgebra as na;

use na::{Point3, RealField, Vector3};
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground, RigidBodyDesc,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

mod physics;
mod entity;
mod vector3d;
mod model_manager;

use crate::physics::init_world;

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


    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        controls.update(&win.input);
        win.render(&cam);
    }
}
