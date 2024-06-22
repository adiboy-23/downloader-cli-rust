pub fn stop(message: &str) {
    println!("Something is wrong...i think its {message}");
    std::process::exit(0); //to stop the process
}
