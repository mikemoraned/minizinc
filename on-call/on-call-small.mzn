% problem definition
enum PERSON;
int: num_slots;
set of int: SLOT = 1..num_slots;

int: num_rotas;
set of int: ROTA = 1..num_rotas;
int: unavailable = 0;
set of int: ROTA0 = 0..num_rotas;

%% availability of each person in that time slot
array[PERSON,SLOT] of ROTA0: availability;

% what we are deciding
array[ROTA,SLOT] of var PERSON: assignment;

% constraints
constraint forall(slot in SLOT, rota in ROTA, person in PERSON)
  (assignment[rota,slot] = person -> availability[person,slot] = rota);
  
output [ show(slot) ++ ":" ++ show(assignment[rota,slot]) ++ if slot == num_slots then "\n" else ", " endif 
        | rota in ROTA, slot in SLOT ];
