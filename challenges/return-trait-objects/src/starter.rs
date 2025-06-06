pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => {
            // Return a Dog instance here
        }
        "robot" => {
            // Return a Robot instance here
        }
        _ => panic!("Unknown speaker type"),
    }
}

// Example usage
pub fn main() {
    let dog_speaker = get_speaker("dog");
    println!("{}", dog_speaker.speak()); // Expected output: Woof

    let robot_speaker = get_speaker("robot");
    println!("{}", robot_speaker.speak()); // Expected output: Beep boop
}
