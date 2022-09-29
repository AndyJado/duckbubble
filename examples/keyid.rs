use duckbubble::parts::KeyId;

fn main() {
    let mut ids = KeyId::new();
    loop {
        dbg!(ids.next());
    }
}
