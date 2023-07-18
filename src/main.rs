/*
This line imports the BinaryHeap data structure from the std::collections module.
BinaryHeap is a priority queue implementation in Rust that will be used to efficiently
keep track of the number of people in the room at any given time.
*/
use std::collections::BinaryHeap;

/*
This line defines a function named times that takes a reference to a slice of tuples (i32, i32)
as input and returns an i32 as the result.
The tuple represents the entry and exit times of the people in the room.
*/
fn times(persons: &[(i32, i32)]) -> i32 {

/*
This line creates a mutable empty vector events
that will store the entry and exit events of the people.
*/
    let mut events: Vec<(i32, i32)> = Vec::new();

/*
This is a loop that iterates over the input persons slice,
which contains tuples of entry and exit times.
For each tuple, it appends two new tuples to the events vector:
one for the entry event with a second element 1,
and another for the exit event with a second element -1.
This approach helps us represent both entry and exit events in a single vector,
so we can sort them later.
*/
    for person in persons {
        events.push((person.0, 1)); // Entry event
        events.push((person.1, -1)); // Exit event
    }

/*
This line sorts the events vector in non-decreasing order based on the first element of each tuple,
which represents the time.
Sorting the events is essential to ensure that we process them in chronological order.
*/
    events.sort();

/*
These lines initialize the variables count, current_people, and a binary heap named heap.
count will store the final result, which represents the number of times the lights get switched on.
current_people will track the number of people in the room at any given time.
The heap will be used to efficiently track the active people in the room.
*/
    let mut count = 0;
    let mut current_people = 0;
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

/*
This is another loop that iterates over the sorted events vector. For each event, it updates the
current_people count based on the second element of the tuple
(which is either 1 for entry or -1 for exit).
If current_people becomes 1, it means there is someone in the room,
and we increment the count to track the number of times the lights get switched on.
*/
    for event in events {
        current_people += event.1; // +1 for entry, -1 for exit

        if current_people == 1 {
            count += 1;
        }
    }

//This line returns the count as the result of the times function.
    count
}
