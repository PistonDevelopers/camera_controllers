#![crate_name = "camera_controllers"]
#![deny(missing_docs)]
#![feature(core, std_misc)]

//! A library for 3D camera control

#[macro_use]
extern crate bitflags;
extern crate event;
extern crate input;
extern crate vecmath;
extern crate quaternion;
extern crate cam;

pub use cam::{
    Camera,
    CameraPerspective,
    model_view_projection,
};

pub use first_person::{
    FirstPerson,
    FirstPersonSettings,
};

pub use orbit_zoom_camera::{
    OrbitZoomCamera,
    OrbitZoomCameraSettings,
};

mod first_person;
mod orbit_zoom_camera;
