fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(gift_message.clone());

    println!("{}", gift_message);
}

fn attach_message_to_present(message: String) {
    println!("The present now has this message: {}", message);
}
