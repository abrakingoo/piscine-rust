// Imagine you are working on some software to control smart lights in a house. 
// You have access to an array of all the lights in that house.

// Define the associated function new, and add it to the data structure Light. 
// It should create a new light with the alias passed as an argument, with a brightness of 0.

// Define the function change_brightness, which receives a slice of lights, 
// an alias and a u8value. It should attempt to find the correct light by its alias, 
// and change the value of the brightness if found.

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
    // Associated function to create a new Light with default brightness 0.
    pub fn new(alias: &str) -> Self {
        Self {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}

// Function to change the brightness of a Light identified by its alias.
pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    if let Some(light) = lights.iter_mut().find(|light| light.alias == alias) {
        light.brightness = value;
    }
}