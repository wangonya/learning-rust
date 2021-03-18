enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => println!("It's about {} o'clock", hours),
        Clock::Analog(hours, minutes, seconds) => {
            println!("Time -> {}:{}:{}", hours, minutes, seconds)
        }
        Clock::Digital(hours, minutes) => println!("Time -> {}:{}", hours, minutes),
    }
}

fn main() {
    tell_time(Clock::Analog(12, 09, 12));
    tell_time(Clock::Sundial(11));
    tell_time(Clock::Digital(10, 39));
}
