int: G; % number of guests
int: T; % number of tables

array[1..(G*G)] of int: affinity;  % symmetric, maybe non-neg

array[1..T] of int: capacity;

array[1..G] of var 1..T: table_of;

% Objective: maximise sum of affinities for pairs at same table
var int: total_affinity =
   sum(g in 1..G, h in 1..G where g < h)(
     affinity[(g * G) + h] * bool2int(table_of[g] = table_of[h])
   );
   
solve maximize total_affinity;