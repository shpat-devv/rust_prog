pub mod queues {
    pub struct Queues{
        queue: Vec<i32>,
    }

    impl Queues{
        pub fn create(queue: Vec<i32>) -> Queues{
            let queue_struct =  Queues { queue: queue };
            queue_struct
        }

        pub fn dequeue(&mut self){
            self.queue.remove(0);
        }

        pub fn queue(&mut self, val:i32){
            self.queue.push(val);
        }
    }
}