# Message util
Helper package for sending and reading messages of type `Message` through TCPStream

* `send_message(mut stream: TcpStream, message: &Message) -> Result<(), Box<dyn Error>>` - is function used for sending `Message` struct through provided `TCPStream`
* `read_message(mut stream: &TcpStream) -> Result<Message, Box<dyn Error>>` - is function used for reading and returning `Message` struct from provided `TCPStream`

