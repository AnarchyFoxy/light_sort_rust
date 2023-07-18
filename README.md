This is recruitment task I've received in C++, but in my free time I decided, to rewrite it in Rust, so here is task content:

There are n people entering and exiting a room. For each i ∈ {1, . . . , n}, person i enters at time ai and exits at time bi (assume bi > ai for all i), and all the ai , bi are distinct. At the beginning of the day, the lights in the room are switched off, and the first person who enters the room switches them on. In order to conserve electricity, if person i leaves the room at time bi and there is no one else present in the room at time bi , then person i will switch the lights off. The next person to enter will then switch them on again. Given the values (a1, b1),(a2, b2), . . .(an, bn), we want to find the number of times the lights get switched on.

Function signature:
int times(const std::vector<std::pair<int, int>>& persons);

    Write the solution.
    Check that your algorithm is correct.
    In comments, explain run time and space complexity for your algorithm.

Examples:
Input: [(1, 5), (2, 6), (3, 7)]

Output: 1


Input: [(11, 15), (1, 10), (2, 8), (5, 12)]

Output: 1


Input: [(5, 7), (6, 8), (9, 10), (1, 3), (2, 4)]

Output: 3


Input: [(1, 2), (2, 3), (3, 4)]

Output: 3
