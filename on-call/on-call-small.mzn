% problem definition
enum PERSON;
int: num_slots;
set of int: SLOT = 1..num_slots;
%% availability of each person in that time slot
array[PERSON,SLOT] of bool: availability;

% what we are deciding
array[SLOT] of var PERSON: assignment;

