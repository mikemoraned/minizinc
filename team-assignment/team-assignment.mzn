enum PERSON = { Bob, Sue, Alice };
enum PROJECT = { P1, P2, P3 };

array[PERSON] of var PROJECT: assignments;
set of int: PRIORITIES = 0..card(PROJECT);
array[PERSON,PROJECT] of PRIORITIES: preferences = array2d(PERSON, PROJECT,
    % P1, P2, P3
    [| 3,  1,  2    % Bob
     | 0,  0,  0    % Sue
     | 3,  2,  1 |] % Alice
);


include "globals.mzn";
constraint alldifferent(assignments);

var int: score = sum(p in PERSON)(preferences[p, assignments[p]]);

solve maximize score;

output ["\(assignments), \(score)"];