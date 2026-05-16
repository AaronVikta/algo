//Answer without any notebooks, write rust code where asked
//explain your reasoning in plain English, 
//show your thinking and not just the answer.


//Question 1
/*
What is the time and space complexity for each
of these functions? Explain your reasoning.
*/
// Function A
fn f_a(n: usize) {
    let mut i = 1;
    while i < n { i *= 3; }
}
 
// Function B
fn f_b(arr: &[i32]) -> Vec<i32> {
    arr.iter().flat_map(|&x| vec![x, x * 2]).collect()
}
 
// Function C
fn f_c(s: &[String]) -> String {
    let mut result = String::new();
    for word in s { result.push_str(word); }
    result
}

/* ANSWER
* for function A the time complexity is 
O(log n) this is because the value of i is mulitiplied
by 3 for rach iteration


Function B is a chain of functions
the iter() is O(1) because it just creates an iterator
the flat_map is O(n) because it processes each element of the array once
the collect is O(n) because it collects all the 
elements into a new vector
so the overall time complexity is O(n) + O(n) = O(n)

function C is a single loop that iterates over a string and pushes 
into the result string.
hence the time complexity is O(n) where n is the number of 
strings in the input slice.
the space complexity is also O(n) because in the worst case,
 all the strings in the input slice are unique and we need to store them all in the result string.
*/

//Question 2
/*
Your teammate says: 'I optimised our search from O(2n) to O(n). 
Big improvement!' What do you tell them? 
Is it actually an improvement?

ANSWER:
This is noy sctully an improvement because O(2n) is the same as O(n) as the 
constant factor is not considered in big O notation.
*/


//Question 3
/*
Question 3 — Design
You are building a player session store for Carnage of Gods that 
will hold up to 500,000 active sessions at peak. 
You need to: look up a session by player_id, insert a new session, 
and remove a session when a player logs out. 
What Rust data structure do you use and what is the complexity
 of each operation? What would be the complexity if you used a 
Vec instead, and why is that unacceptable at 500,000 players?

ANSWER:
I would use a HashMap in Rust to store the player sessions.
The key would be the player_id and the value would 
be the session data.
The complexity of each operation would be:
- Look up a session by player_id: O(1) average case,
 O(n) worst case (if there are hash collisions)
- Insert a new session: O(1) average case, 
O(n) worst case (if there are hash collisions)
- Remove a session: O(1) average case, 
O(n) worst case (if there are hash collisions)
*/

//Question 4
/*
Question 4 — Tradeoff
You have an unsorted list of 1,000,000 player scores.
 You need to check if a given score appears in the list.
 You will perform this check 10,000 times.
  Describe two approaches with different time-space tradeoffs,
   give the complexity of each approach for all 10,000 
   checks combined,
 and state which you would choose and why.

 ANSWER:
 There are two approaches to solving the problem
 1. Linear Search 
 2. HashSet
    Approach 1: Linear Search
- Time Complexity: O(n) per check, O(n * m) for 10,000 checks (where n is the number of scores and m is the number of checks)
- Space Complexity: O(1) (no additional space used)
    Approach 2: HashSet
- Time Complexity: O(n) to build the HashSet, O(1) per check, O(n + m) for 10,000 checks
- Space Complexity: O(n) (to store the HashSet)

I would choose the HashSet approach because it significantly
 reduces the time complexity for checking if a score appears 
 in the list. Although it uses more space, the performance 
 improvement for 10,000 checks is substantial, especially 
 as the number of scores increases. The linear search 
 approach would be inefficient and could lead to long wait
  times for each check, while the HashSet allows for constant 
  time lookups after an initial setup cost.    

*/

//Question 5
/*
*Question 5 — Rust Trap
The following Rust function is used in a hot path that processes 
10,000 player event logs per second. 
Identify the hidden performance problem and rewrite it correctly.

fn build_report(events: &[String]) -> String {
    let mut report = String::new();
    for event in events {
        report = report + "[EVENT] " + event;
    }
    report
}

ANSWER:
The hidden performance problem in the original function 
is that it creates a new String object every time it 
concatenates the report with a new event. This leads to
a lot of unnecessary allocations and copying, which can
significantly degrade performance, especially when processing a large number of events.
A more efficient way to build the report is to use the
push_str method, which appends to the existing String without creating a new one each time. Here’s the rewritten function:  

fn build_report(events: &[String]) -> String {
    let mut report = String::new();
    for event in events {
        report.push_str("[EVENT] ");
        report.push_str(event);
    }
    report
}
*/