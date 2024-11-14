enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
    fn prev(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Yellow,
            TrafficLight::Green => TrafficLight::Red,
            TrafficLight::Yellow => TrafficLight::Green,
        }
    }
}

fn main() {
    // Create an instance of TrafficLight
    let light = TrafficLight::Red;

    // Call the next method and print the result
    let next_light = light.prev().next().next();
    
    // Using match to print out the current state
    match next_light {
        TrafficLight::Red => println!("The next light is Red."),
        TrafficLight::Yellow => println!("The next light is Yellow."),
        TrafficLight::Green => println!("The next light is Green."),
    }
}
