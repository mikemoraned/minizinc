int: G; % number of guests
int: T; % number of tables

array[1..(G*G)] of int: affinity;  % symmetric, maybe non-neg

array[1..T] of int: capacity;

array[1..G] of var 1..T: table_of;

