fn init() {
    door.set_path(door_path);
    door2.set_path(door_path);
    door3.set_path(door_path);
    hatchleft.set_path(hatchleft_path);
    hatchright.set_path(hatchright_path);
    lift.set_path(lift_path);
}

fn door_switch_interact() {
    door.go_to(1, 1.0);
    context.learn(flight);
}

fn door2_interact() {
    door2.go_to(1, 1.0);
}

fn door3_switch_interact() {
    door3.go_to(1, 1.0);
    context.forget(flight);
}

fn hatch_trigger_enter() {
    hatchleft.go_to(2, 4.0);
    hatchright.go_to(2, 4.0);
    door2.go_to(1, 4.0);
}

fn lift_trigger_enter() {
    lift.set_motion(forward_cycle, 0.5);
}

fn final_switch_interact() {
    context.win();
}
