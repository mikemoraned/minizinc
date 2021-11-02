% re-used constraints
include "alldifferent.mzn";

% problem definition
enum PERSON;
int: num_slots;
set of int: SLOT = 1..num_slots;

int: num_rotas;
set of int: ROTA = 1..num_rotas;

%% availability of each person in that time slot for that rota
enum AVAILABILITY = { Y, n };
array[PERSON,ROTA,SLOT] of AVAILABILITY: availability;

% what we are deciding
array[ROTA,SLOT] of var PERSON: assignment;

% constraints
%% can only assign a person to a rota if they are available
constraint forall(slot in SLOT, rota in ROTA, person in PERSON)
  (assignment[rota,slot] = person -> availability[person,rota,slot] = Y);
%% every person must be assigned to at least one slot in one rota
constraint forall(person in PERSON)(count(slot in SLOT, rota in ROTA)(assignment[rota,slot] = person) >= 1);
%% a person cannot be on multiple rotas at the same time
constraint forall(slot in SLOT)(alldifferent([ assignment[rota,slot] | rota in ROTA ]));
  
output [ show(slot) ++ ":" ++ show(assignment[rota,slot]) ++ if slot == num_slots then "\n" else ", " endif 
        | rota in ROTA, slot in SLOT ];
