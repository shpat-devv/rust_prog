mod training;
mod rust_basics;
fn main() {
    let my_list1: Vec<i32> = [8,3,2,5,4,9].to_vec();
    let my_list2: Vec<i32> = [1,1,1,2,3,3,4,4,5].to_vec();
    let my_list3: Vec<i32> = [1,2,3,4,5].to_vec();

    println!("==============================");
    println!("      Array Algorithms");
    println!("==============================\n");

    println!("Reversing array:      {:?}", training::easy::array_reverse(&mut [1, 2, 3, 4, 5]));
    println!("Finding max value:    {:?}", training::easy::find_max(&mut [6, 2, 3, 4, 5]));
    println!("Finding min value:    {:?}", training::easy::find_lowest(&mut [6, 2, 3, 4, 5, 1]));
    println!("Binary search (5):    {}", training::easy::binary_search(&mut [1, 2, 3, 4, 5], 5));
    println!("Is sorted:            {}", training::easy::check_sort(&[2, 2, 3, 4, 5]));
    println!("Bubble sort result:   {:?}", training::easy::bubble_sort(my_list1));
    println!("Remove Duplicates     {:?}",training::easy::remove_duplicates(my_list2));

    println!("\n==============================");
    println!("      Hash Maps");
    println!("==============================\n");

    println!("Reversing array:      {:?}", training::easy::array_reverse(&mut [1, 2, 3, 4, 5]));

    println!("\n==============================");
    println!("      Structs");
    println!("==============================\n");


    let mut bonnie = rust_basics::structures::pet::Pet::new("Bonnie", "Peter", false);
    println!("bonnie                {:?}", bonnie);
    bonnie.do_trick();
    bonnie.give_treat();

    let rect1 = rust_basics::structures::rect::Rectangle::new(10,5);
    println!("{}", rect1.calculate_area());

    println!("\n==============================");
    println!("      Queues");
    println!("==============================\n");

    let mut my_queue = rust_basics::queues::queues::Queues::create(my_list3);
    println!("Original queue: {:?}", my_queue);
    my_queue.dequeue();
    println!("After dequeue: {:?}", my_queue);
    my_queue.queue(10);
    println!("After queueing 10: {:?}", my_queue);


    println!("\n==============================");
    println!("      ENUMS");
    println!("==============================\n");

    let my_ip = rust_basics::enums::IppAddrKind::V4;
    let your_ip = rust_basics::enums::IppAddrKind::V6;

    let my_opinion = rust_basics::enums::Opinion::GOOD(String::from("Cats are cool"));
    let your_opinion = rust_basics::enums::Opinion::BAD(String::from("Cats are bad"));

    println!("my_ip: {:?}", my_ip);
    println!("your_ip: {:?}", your_ip);
    println!("my_opinion: {:?}", my_opinion);
    println!("your_opinion: {:?}", your_opinion);
}
