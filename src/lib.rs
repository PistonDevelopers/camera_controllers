#![crate_name = "camera_controllers"]
#![deny(missing_docs)]

//! A library for 3D camera control

#[macro_use]
extern crate bitflags;
extern crate piston;
extern crate vecmath;
extern crate quaternion;
extern crate cam;
extern crate num;

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
