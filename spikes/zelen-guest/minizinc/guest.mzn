int: G; % number of guests
int: T; % number of tables

array[1..G, 1..G] of int: affinity;  % symmetric, maybe non-neg
array[1..T] of int: capacity;

