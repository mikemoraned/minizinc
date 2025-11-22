int: G; % number of guests
int: T; % number of tables

array[1..(G*G)] of int: affinity;  % symmetric, maybe non-neg

array[1..T] of int: capacity;

array[1..G] of var 1..T: table_of;

% Capacity constraint
constraint
  forall(t in 1..T)(
    sum(g in 1..G)(bool2int(table_of[g] = t)) <= capacity[t]
  );

% Objective: maximise sum of affinities for pairs at same table
var int: total_affinity =
   sum(g in 1..G, h in 1..G where g < h)(
     affinity[(g * G) + h] * bool2int(table_of[g] = table_of[h])
   );
   
solve maximize total_affinity;

output ["Guest " ++ show(g) ++ " → Table " ++ show(table_of[g]) ++ "\n" | g in 1..G ];