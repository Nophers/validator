mod validator;
use validator::valid;

fn main() {
    let file = "test.png3";
    let is_valid = valid(file);

    let message = match is_valid {
        true => "Valid file",
        false => "Invalid file",
    };

    println!("{}", message);
}
