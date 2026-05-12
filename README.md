## How much data your publisher program will send to the message broker in one run?

In one run, the publisher program sends exactly 5 message events (or 5 `UserCreatedEventMessage` structs) to the message broker. These messages contain the user ID (1 to 5) and associated user names ("Amir", "Budi", "Cica", "Dira", "Emir").

## The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?

It means that both the publisher and the subscriber programs are connecting to the exact same message broker instance (in this case, a RabbitMQ server running locally on your machine on port 5672). Because they are connected to the same broker, the publisher can send messages to it, and the subscriber can receive those same messages from it.


RabbitMQ:
![alt text](image.png)

Run:
![alt text](image-1.png)
Running cargo run in the publisher triggers the app to publish five user-created messages to the RabbitMQ broker, and the subscriber then reads and displays those messages.

Monitoring:
![alt text](image-2.png)
The spikes in the second chart are caused by repeatedly running the publisher. Each time the publisher runs, it sends a short burst of messages to RabbitMQ, so the broker briefly records higher message activity in the Publish and delivery-related rates. After those messages are consumed by the subscriber, the rate drops back to zero, which is why the chart shows sharp spikes.