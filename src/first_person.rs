#![allow(dead_code)]

//! A first person camera.

use input::{ Button, GenericEvent };
use vecmath::traits::{ Float, Radians };

use Camera;

bitflags!(pub struct Keys: u8 {
    const MOVE_FORWARD  = 0b00000001;
    const MOVE_BACKWARD = 0b00000010;
    const STRAFE_LEFT   = 0b00000100;
    const STRAFE_RIGHT  = 0b00001000;
    const FLY_UP        = 0b00010000;
    const FLY_DOWN      = 0b00100000;
});

/// First person camera settings.
pub struct FirstPersonSettings<T=f32> {
    /// Which button to press to move forward.
    pub move_forward_button: Button,
    /// Which button to press to move backward.
    pub move_backward_button: Button,
    /// Which button to press to strafe left.
    pub strafe_left_button: Button,
    /// Which button to press to strafe right.
    pub strafe_right_button: Button,
    /// Which button to press to fly up.
    pub fly_up_button: Button,
    /// Which button to press to fly down.
    pub fly_down_button: Button,
    /// Which button to press to move faster.
    pub move_faster_button: Button,
    /// The horizontal movement speed.
    ///
    /// This is measured in units per second.
    pub speed_horizontal: T,
    /// The vertical movement speed.
    ///
    /// This is measured in units per second.
    pub speed_vertical: T,
    /// The horizontal mouse sensitivity.
    ///
    /// This is a multiplier applied to horizontal mouse movements.
    pub mouse_sensitivity_horizontal: T,
    /// The vertical mouse sensitivity.
    ///
    /// This is a multiplier applied to vertical mouse movements.
    pub mouse_sensitivity_vertical: T,
}

impl<T> FirstPersonSettings<T>
    where T: Float
{
    /// Creates new first person camera settings with wasd defaults.
    pub fn keyboard_wasd() -> FirstPersonSettings<T> {
        use input::Button::Keyboard;
        use input::Key;

        FirstPersonSettings {
            move_forward_button: Keyboard(Key::W),
            move_backward_button: Keyboard(Key::S),
            strafe_left_button: Keyboard(Key::A),
            strafe_right_button: Keyboard(Key::D),
            fly_up_button: Keyboard(Key::Space),
            fly_down_button: Keyboard(Key::LShift),
            move_faster_button: Keyboard(Key::LCtrl),
            speed_horizontal: T::one(),
            speed_vertical: T::one(),
            mouse_sensitivity_horizontal: T::one(),
            mouse_sensitivity_vertical: T::one(),
        }
    }

    /// Creates a new first person camera settings with esdf defaults.
    pub fn keyboard_esdf() -> FirstPersonSettings<T> {
        use input::Button::Keyboard;
        use input::Key;

        FirstPersonSettings {
            move_forward_button: Keyboard(Key::E),
            move_backward_button: Keyboard(Key::D),
            strafe_left_button: Keyboard(Key::S),
            strafe_right_button: Keyboard(Key::F),
            fly_up_button: Keyboard(Key::Space),
            fly_down_button: Keyboard(Key::Z),
            move_faster_button: Keyboard(Key::LShift),
            speed_horizontal: T::one(),
            speed_vertical: T::one(),
            mouse_sensitivity_horizontal: T::one(),
            mouse_sensitivity_vertical: T::one(),
        }
    }

    /// Creates new first person camera settings with zqsd defaults (azerty keyboard layout).
    pub fn keyboard_zqsd() -> FirstPersonSettings<T> {
        use input::Button::Keyboard;
        use input::Key;

        FirstPersonSettings {
            move_forward_button: Keyboard(Key::Z),
            move_backward_button: Keyboard(Key::S),
            strafe_left_button: Keyboard(Key::Q),
            strafe_right_button: Keyboard(Key::D),
            fly_up_button: Keyboard(Key::Space),
            fly_down_button: Keyboard(Key::LShift),
            move_faster_button: Keyboard(Key::LCtrl),
            speed_horizontal: T::one(),
            speed_vertical: T::one(),
            mouse_sensitivity_horizontal: T::one(),
            mouse_sensitivity_vertical: T::one(),
        }
    }
}

/// Models a flying first person camera.
pub struct FirstPerson<T=f32> {
    /// The first person camera settings.
    pub settings: FirstPersonSettings<T>,
    /// The yaw angle (in radians).
    pub yaw: T,
    /// The pitch angle (in radians).
    pub pitch: T,
    /// The direction we are heading.
    pub direction: [T; 3],
    /// The position of the camera.
    pub position: [T; 3],
    /// The velocity we are moving in the direction.
    pub velocity: T,
    /// The keys that are pressed.
    keys: Keys,
}

impl<T> FirstPerson<T>
    where T: Float
{
    /// Creates a new first person camera.
    pub fn new(
        position: [T; 3],
        settings: FirstPersonSettings<T>
    ) -> FirstPerson<T> {
        let _0: T = T::zero();
        FirstPerson {
            settings: settings,
            yaw: _0,
            pitch: _0,
            keys: Keys::empty(),
            direction: [_0, _0, _0],
            position: position,
            velocity: T::one(),
        }
    }

    /// Computes camera.
    pub fn camera(&self, dt: f64) -> Camera<T> {
        let dt = T::from_f64(dt);
        let dh = dt * self.velocity * self.settings.speed_horizontal;
        let (dx, dy, dz) = (self.direction[0], self.direction[1], self.direction[2]);
        let (s, c) = (self.yaw.sin(), self.yaw.cos());
        let mut camera = Camera::new([
            self.position[0] + (s * dx - c * dz) * dh,
            self.position[1] + dy * dt * self.settings.speed_vertical,
            self.position[2] + (s * dz + c * dx) * dh
        ]);
        camera.set_yaw_pitch(self.yaw, self.pitch);
        camera
    }

    /// Handles game event and updates camera.
    pub fn event<E>(&mut self, e: &E) where E: GenericEvent {
        e.update(|args| {
            let cam = self.camera(args.dt);
            self.position = cam.position;
        });

        let &mut FirstPerson {
            ref mut yaw,
            ref mut pitch,
            ref mut keys,
            ref mut direction,
            ref mut velocity,
            ref settings,
            ..
        } = self;

        let pi: T = Radians::_180();


        let _0 = T::zero();
        let _1 = T::one();
        let _2 = _1 + _1;
        let _3 = _2 + _1;
        let _4 = _3 + _1;
        let _360 = T::from_isize(360);
        let sqrt2 = _2.sqrt();

        e.mouse_relative(|dx, dy| {

            let dx = T::from_f64(dx) * settings.mouse_sensitivity_horizontal;
            let dy = T::from_f64(dy) * settings.mouse_sensitivity_vertical;

            *yaw = (*yaw - dx / _360 * pi / _4) % (_2 * pi);
            *pitch = *pitch + dy / _360 * pi / _4;
            *pitch = (*pitch).min(pi / _2).max(-pi / _2);
        });
        e.press(|button| {
            let (dx, dy, dz) = (direction[0], direction[1], direction[2]);
            let sgn = |x: T| if x == _0 { _0 } else { x.signum() };
            let mut set = |k, x: T, y: T, z: T| {
                let (x, z) = (sgn(x), sgn(z));
                let (x, z) = if x != _0 && z != _0 {
                    (x / sqrt2, z / sqrt2)
                } else {
                    (x, z)
                };
                *direction = [x, y, z];
                keys.insert(k);
            };
            match button {
                x if x == settings.move_forward_button =>
                    set(MOVE_FORWARD, -_1, dy, dz),
                x if x == settings.move_backward_button =>
                    set(MOVE_BACKWARD, _1, dy, dz),
                x if x == settings.strafe_left_button =>
                    set(STRAFE_LEFT, dx, dy, _1),
                x if x == settings.strafe_right_button =>
                    set(STRAFE_RIGHT, dx, dy, -_1),
                x if x == settings.fly_up_button =>
                    set(FLY_UP, dx, _1, dz),
                x if x == settings.fly_down_button =>
                    set(FLY_DOWN, dx, -_1, dz),
                x if x == settings.move_faster_button => *velocity = _2,
                _ => {}
            }
        });
        e.release(|button| {
            let (dx, dy, dz) = (direction[0], direction[1], direction[2]);
            let sgn = |x: T| if x == _0 { _0 } else { x.signum() };
            let mut set = |x: T, y: T, z: T| {
                let (x, z) = (sgn(x), sgn(z));
                let (x, z) = if x != _0 && z != _0 {
                    (x / sqrt2, z / sqrt2)
                } else {
                    (x, z)
                };
                *direction = [x, y, z];
            };
            let mut release = |key, rev_key, rev_val| {
                keys.remove(key);
                if keys.contains(rev_key) { rev_val } else { _0 }
            };
            match button {
                x if x == settings.move_forward_button =>
                    set(release(MOVE_FORWARD, MOVE_BACKWARD, _1), dy, dz),
                x if x == settings.move_backward_button =>
                    set(release(MOVE_BACKWARD, MOVE_FORWARD, -_1), dy, dz),
                x if x == settings.strafe_left_button =>
                    set(dx, dy, release(STRAFE_LEFT, STRAFE_RIGHT, -_1)),
                x if x == settings.strafe_right_button =>
                    set(dx, dy, release(STRAFE_RIGHT, STRAFE_LEFT, _1)),
                x if x == settings.fly_up_button =>
                    set(dx, release(FLY_UP, FLY_DOWN, -_1), dz),
                x if x == settings.fly_down_button =>
                    set(dx, release(FLY_DOWN, FLY_UP, _1), dz),
                x if x == settings.move_faster_button => *velocity = _1,
                _ => {}
            }
        });
    }
}
