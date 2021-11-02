% problem definition
enum PERSON;
int: num_slots;
set of int: SLOT = 1..num_slots;

int: num_rotas;
set of int: ROTA = 1..num_rotas;

%% availability of each person in that time slot for that rota
enum AVAILABILITY = { Y, N };
array[PERSON,ROTA,SLOT] of AVAILABILITY: availability;

% what we are deciding
array[ROTA,SLOT] of var PERSON: assignment;

% constraints
constraint forall(slot in SLOT, rota in ROTA, person in PERSON)
  (assignment[rota,slot] = person -> availability[person,rota,slot] = Y);
  
output [ show(slot) ++ ":" ++ show(assignment[rota,slot]) ++ if slot == num_slots then "\n" else ", " endif 
        | rota in ROTA, slot in SLOT ];
