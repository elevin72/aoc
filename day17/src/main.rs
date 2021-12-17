fn main() {
    println!("{}", star34());
}

//input
const X_MIN: i32 = 150;
const X_MAX: i32 = 171;
const Y_MIN: i32 = -129;
const Y_MAX: i32 = -70;

fn star34() -> u32 {
    let mut count = 0;
    for y in -150..=150 { // overshoot bounds on both directions
        for x in 0..=171 {
            if reaches_target((x,y)) {
                count += 1;
            }
        }
    }
    count
}

fn reaches_target((mut x_vel, mut y_vel): (i32, i32)) -> bool {
    let (mut x_pos, mut y_pos) = (0, 0);
    while y_vel >= -130 { // if faster than -130 then will never stop in target

        if x_pos >= X_MIN && x_pos <= X_MAX && y_pos >= Y_MIN && y_pos <= Y_MAX {
            return true;
        }

        x_pos += x_vel;
        y_pos += y_vel;

        x_vel += match x_vel.cmp(&0) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Greater => -1,
            std::cmp::Ordering::Equal => 0,
        };

        y_vel -= 1;

    }

    return false;
}
