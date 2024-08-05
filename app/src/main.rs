
// Need to simulate a client on creation everytime JUDY is called
mod protocol;
mod judy;
mod user;

fn main()
{
    let judy_client = judy::new();
}