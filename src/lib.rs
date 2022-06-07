/// an easing function that eases in and out by constant acceleration in either direction (parabolic), that allows the user to specify a starting velocty. This allows a user to interrupt the animation partway through and send it in a different direction without any sudden jerks.

#[inline(always)]
fn sq(a: f32) -> f32 {
    a * a
}

//I'm including these because the way the constant acceleration functions handle cases where initial_velocity > 2 doesn't look very good, it overshoots the target, then comes back to it. You will probably prefer to use these linear acceleration-deceleration functions for those cases instead. You might like to use them instead of constant acceleration methods altogether, but I find they look a bit too jerky.
#[inline(always)]
fn linear_acceleration_ease_in_out_with_initial_velocity(t: f32, initial_velocity: f32) -> f32 {
    t * (t * ((initial_velocity - 2f32) * t + (3f32 - 2f32 * initial_velocity)) + initial_velocity)
}
#[inline(always)]
fn velocity_of_linear_acceleration_ease_in_out_with_initial_velocity(
    t: f32,
    initial_velocity: f32,
) -> f32 {
    t * ((3f32 * initial_velocity - 6f32) * t + (6f32 - 4f32 * initial_velocity)) + initial_velocity
}

//these variants, as the ones above, act on the domain [0,1]
#[inline(always)]
fn constant_acceleration_ease_in_out_with_initial_velocity(t: f32, initial_velocity: f32) -> f32 {
    if t >= 1f32 {
        return 1f32;
    }
    let sqrt_part = (2f32 * sq(initial_velocity) - 4f32 * initial_velocity + 4f32).sqrt();
    let m = (2f32 - initial_velocity
        + if initial_velocity < 2f32 {
            sqrt_part
        } else {
            -sqrt_part
        })
        / 2f32;
    let ax = -initial_velocity / (2f32 * m);
    let ay = initial_velocity * ax / 2f32;
    let h = (ax + 1f32) / 2f32;
    if t < h {
        return m * sq(t - ax) + ay;
    } else {
        return -m * sq(t - 1f32) + 1f32;
    }
}
#[inline(always)]
fn velocity_of_constant_acceleration_ease_in_out_with_initial_velocity(
    t: f32,
    initial_velocity: f32,
) -> f32 {
    if t >= 1f32 {
        return 0f32;
    }
    let sqrt_part = (2f32 * sq(initial_velocity) - 4f32 * initial_velocity + 4f32).sqrt();
    let m = (2f32 - initial_velocity
        + if initial_velocity < 2f32 {
            sqrt_part
        } else {
            -sqrt_part
        })
        / 2f32;
    let ax = -initial_velocity / (2f32 * m);
    // let ay = initial_velocity * ax / 2f32;
    let h = (ax + 1f32) / 2f32;
    if t < h {
        return 2f32 * m * (t - ax);
    } else {
        return 2f32 * m * (1f32 - t);
    }
}

pub fn ease(
    start_value: f32,
    end_value: f32,
    start_time: f32,
    end_time: f32,
    current_time: f32,
    initial_velocity: f32,
) -> f32 {
    if start_time == f32::NEG_INFINITY {
        return end_value;
    }
    if start_value == end_value {
        //having difficulty solving for this case. Screw it. It basically never happens anyway.
        return start_value;
    } else {
        let normalized_time = (current_time - start_time) / (end_time - start_time);
        let normalized_velocity =
            initial_velocity / (end_value - start_value) * (end_time - start_time);
        // if velocity is too high, it just resorts to linear acceleration
        let normalized_output = if normalized_velocity > 2f32 {
            linear_acceleration_ease_in_out_with_initial_velocity(
                normalized_time,
                normalized_velocity,
            )
        } else {
            constant_acceleration_ease_in_out_with_initial_velocity(
                normalized_time,
                normalized_velocity,
            )
        };
        start_value + normalized_output * (end_value - start_value)
    }
}

pub fn vel_ease(
    start_value: f32,
    end_value: f32,
    start_time: f32,
    end_time: f32,
    current_time: f32,
    initial_velocity: f32,
) -> f32 {
    if start_time == f32::NEG_INFINITY {
        return 0.0;
    }
    if start_value == end_value {
        0.0
    } else {
        let normalized_time = (current_time - start_time) / (end_time - start_time);
        let normalized_velocity =
            initial_velocity / (end_value - start_value) * (end_time - start_time);
        let normalized_output = if normalized_velocity > 2f32 {
            velocity_of_linear_acceleration_ease_in_out_with_initial_velocity(
                normalized_time,
                normalized_velocity,
            )
        } else {
            velocity_of_constant_acceleration_ease_in_out_with_initial_velocity(
                normalized_time,
                normalized_velocity,
            )
        };
        normalized_output * (end_value - start_value) / (end_time - start_time)
    }
}

/// just the above two woven together
pub fn ease_val_vel(
    start_value: f32,
    end_value: f32,
    start_time: f32,
    end_time: f32,
    current_time: f32,
    initial_velocity: f32,
) -> (f32, f32) {
    if start_time == f32::NEG_INFINITY {
        return (end_value, 0.0);
    }
    let normalized_time = (current_time - start_time) / (end_time - start_time);
    let normalized_velocity =
        initial_velocity / (end_value - start_value) * (end_time - start_time);

    let (normalized_p_out, normalized_vel_out) = if normalized_velocity > 2f32 {
        (
            linear_acceleration_ease_in_out_with_initial_velocity(
                normalized_time,
                normalized_velocity,
            ),
            velocity_of_linear_acceleration_ease_in_out_with_initial_velocity(
                normalized_time,
                normalized_velocity,
            ),
        )
    } else {
        (
            constant_acceleration_ease_in_out_with_initial_velocity(
                normalized_time,
                normalized_velocity,
            ),
            velocity_of_constant_acceleration_ease_in_out_with_initial_velocity(
                normalized_time,
                normalized_velocity,
            ),
        )
    };
    (
        start_value + normalized_p_out * (end_value - start_value),
        normalized_vel_out * (end_value - start_value) / (end_time - start_time),
    )
}

pub struct InterruptableEaser {
    pub start_value: f32,
    pub end_value: f32,
    pub start_time: f32,
    pub start_velocity: f32,
}
impl InterruptableEaser {
    pub fn new(v: f32) -> Self {
        Self {
            start_value: v,
            end_value: v,
            start_time: f32::NEG_INFINITY,
            start_velocity: 0.0,
        }
    }

    /// begins the approach towards `v`
    pub fn approach(&mut self, v: f32, current_time: f32, transition_duration: f32) {
        (self.start_value, self.start_velocity) = ease_val_vel(
            self.start_value,
            self.end_value,
            self.start_time,
            self.start_time + transition_duration,
            current_time,
            self.start_velocity,
        );
        self.start_time = current_time;
        self.end_value = v;
    }

    /// gets the value as it would be at the current time
    pub fn v(&self, current_time: f32, transition_duration: f32) -> f32 {
        ease(
            self.start_value,
            self.end_value,
            self.start_time,
            self.start_time + transition_duration,
            current_time,
            self.start_velocity,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut ie = InterruptableEaser::new(-1.0);
        assert_eq!(ie.v(20.0, 0.2), -1.0);
        ie.approach(1.0, 20.0, 0.2);
        assert!((ie.v(20.1, 0.2)).abs() < 0.0001);
        assert!((ie.v(20.2, 0.2) - 1.0).abs() < 0.0001);
        assert_eq!(ie.v(20.3, 0.2), 1.0);
    }
}
