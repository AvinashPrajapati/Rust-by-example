// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }

// impl TrafficLight {
//     fn next(&self) -> TrafficLight {
//         match self {
//             TrafficLight::Red => TrafficLight::Green,
//             TrafficLight::Yellow => TrafficLight::Red,
//             TrafficLight::Green => TrafficLight::Yellow,
//         }
//     }
//     fn prev(&self) -> TrafficLight {
//         match self {
//             TrafficLight::Red => TrafficLight::Yellow,
//             TrafficLight::Green => TrafficLight::Red,
//             TrafficLight::Yellow => TrafficLight::Green,
//         }
//     }
// }

// fn main() {
//     // Create an instance of TrafficLight
//     let light = TrafficLight::Red;

//     // Call the next method and print the result
//     let next_light = light.prev().next().next();
    
//     // Using match to print out the current state
//     match next_light {
//         TrafficLight::Red => println!("The next light is Red."),
//         TrafficLight::Yellow => println!("The next light is Yellow."),
//         TrafficLight::Green => println!("The next light is Green."),
//     }
// }
// -----------



fn main(){
    
    // enum Result {
    //     Success(f64),
    //     Failure(u16, char),
    //     Uncertainty,
    //     }
    // let outcome = Result::Failure(1200, 'X');
    // match outcome {
    //     Result::Success(value) =>  print!("Result: {}", value),
    //     Result::Failure(error_code, module) =>
    //     print!("Error n. {} in module {}",
    //     error_code, module),
    //     Result::Uncertainty => {},

    //     };


        // ---------
        // enum CardinalPoint { North, South, West, East };
        // let direction = CardinalPoint::South;
        // print!("{}", match direction {
        // CardinalPoint::North => 'N',
        // CardinalPoint::South => 'S',
        // _ => '*',
        // });


        // yes, you can use for loop also
        for n in -2..5 {
            println!("{} is {}.", n, match n {
            0 => "zero",
            1 => "one",
            _ if n < 0 => "negative",
            _ => "plural",
            });
            }
}