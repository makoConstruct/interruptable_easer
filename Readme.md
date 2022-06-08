State for running an ease animation along a linear quantity. The twist: You can interrupt the animation and point it at a different target. With simpler animation code, this would result in either a sudden jump, or the velocity would suddenly go to zero. InterruptableEaser instead reorients intelligently without any sudden jerks or jumps.

It works by just remembering the initial velocity and hte initial location, instead of sort of simulating, frame by frame, a little thing moving along. This might not be practical (it certainly wasn't the easiest way, and the implementation could probably be improved) but it's a better API for many kinds of animation.

A fairly realistic example of using an InterruptableEaser to make an animated health bar:

```rust
extern crate blibium; //note, blibium is my simple C++ game engine. There isn't a rust version. The following API might not be possible, but if it is, maybe it would be worth creating blibium rust, because this is better than the C++ version (because C++ doesn't have macros, so it ends up having to use a special langauge for resource stuff).
use blibium::{def, grab, rgb_hex, draw_rectangle, Rect, v2};

def!(health_bar_anim_dur, 0.2);
//ppmr is a unit representing the size of the screen, pixels per milli-radian. We can use it to do things in resolution-independent ways/allow the user to scale the UI.
def!(health_thickness, |ppmr| 0.8*ppmr);
def!(whether_rtl, false);
//resources can be grabbed by other resource definitions
def!(health_anchor, |whether_rtl, ppmr| v2(2.0*ppmr).mirror_x_if(whether_rtl));
def!(health_bar, InterruptableEaser::new(100.0));
def!(health_color, rgb_hex!(F21D40))

async fn take_damage(damage: f32){
    grab!(mut health_bar, t, health_bar_anim_dur);
    let new_health = (health_bar.end_value - damage).max(0.0);
    health_bar.approach(new_health, t, health_bar_anim_dur);
}

async fn render_health(amount: f32){
    grab!(health_bar, health_anchor, health_bar_direction, health_thickness, health_bar_anim_dur, health_color);
    draw_rectangle(
        &Rect::bl_ur(
            health_anchor,
            health_anchor + health_bar_direction*health_bar.v(t, health_bar_anim_dur) + v2(0.0, health_thickness), 
        ),
        health_color,
    );
}
```