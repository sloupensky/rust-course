use tokio::io::{AsyncReadExt, AsyncWriteExt,split,duplex};
use message_utils::{Message, send_message, read_message};

#[tokio::test]
async fn test_send_read_message() {
    let (client, mut server) = duplex(64);
    let (mut read, mut write) = split(client);

    let send_task = tokio::spawn(async move {
        let message = Message::Text("test".to_string());
        send_message(&mut write, &message).await.unwrap();
    });

    let read_task = tokio::spawn(async move {
        read_message(&mut read).await.unwrap()
    });

    send_task.await.unwrap();

    let mut buffer = vec![0; 64];
    let n = server.read(&mut buffer).await.unwrap();
    server.write_all(&buffer[..n]).await.unwrap();

    let received_message = read_task.await.unwrap();
    let expected_message = Message::Text("test".to_string());

    assert_eq!(serde_json::to_string(&received_message).unwrap(), serde_json::to_string(&expected_message).unwrap());
}