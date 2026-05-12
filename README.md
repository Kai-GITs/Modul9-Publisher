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

Monitoring Slow Subscriber:
![alt text](image-3.png)
The image shows that the queued message count briefly rose to around 15 before falling back to 0. That happened because each publisher run sends 5 messages, and since i ran the publisher several times quickly while the subscriber was slowed down by thread::sleep, messages arrived faster than the subscriber could process them.

So the total became 15 because RabbitMQ temporarily stored the accumulated messages in the queue:

1 publisher run = 5 messages
3 quick runs = 15 messages
The chart then drops back down because the subscriber kept consuming those queued messages until the queue was empty again.

Monitor 3 Slow Subscriber:
![alt text](image-5.png)
This image shows three subscriber processes running at the same time. Each terminal prints different UserCreatedEventMessage items, which means RabbitMQ is distributing the published messages across the three subscribers instead of one subscriber handling everything alone.
![alt text](image-4.png)
RabbitMQ with Connections: 3, Channels: 3, and Consumers: 3. The queued-messages graph stays at 0, while the message-rate graph shows short spikes in publish and consumer activity. That means messages were consumed quickly enough that they did not pile up in the queue.

Before, with one slow subscriber and repeated publisher runs, messages accumulated in the queue, so you saw the queued total rise, such as 15. That happened because one consumer could not keep up.

Now, with three subscribers, the workload is shared. I still publish the same messages, but three consumers process them in parallel, so the queue drains much faster.

Suggested Code improvements: The subsriber program still uses std::thread::sleep inside an async tokio program. That blocks the runtime thread. tokio::time::sleep(...).await would be cleaner and more correct. It also does not set QoS/prefetch. If you want fairer distribution among slow subscribers, set basic_qos(1, ...) so one consumer does not receive too many unacked messages at once.

Cloud Test:
![alt text](image-6.png)
![alt text](image-7.png)
![alt text](image-8.png)
