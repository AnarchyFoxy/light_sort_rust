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

/*
The main() function contains the test cases, where different input vectors
are created and passed to the times function.
The result is then printed using println! statements.
*/
fn main() {
    let test1 = vec![(1, 5), (2, 6), (3, 7)];
    println!("{}", times(&test1)); // Output: 1

    let test2 = vec![(11, 15), (1, 10), (2, 8), (5, 12)];
    println!("{}", times(&test2)); // Output: 1

    let test3 = vec![(5, 7), (6, 8), (9, 10), (1, 3), (2, 4)];
    println!("{}", times(&test3)); // Output: 3

    let test4 = vec![(1, 2), (2, 3), (3, 4)];
    println!("{}", times(&test4)); // Output: 3
}
