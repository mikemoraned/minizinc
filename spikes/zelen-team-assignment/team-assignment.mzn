% problem definition
%% person/project
enum PERSON = { Bob, Sue, Robert, Alice };
enum PROJECT = { P1, P2, P3 };
%% preferences of each person
set of int: PRIORITY = 0..card(PROJECT);
array[PERSON,PROJECT] of PRIORITY: preferences = array2d(PERSON, PROJECT,
    % P1, P2, P3
    [| 3,  1,  2    % Bob
     | 0,  0,  0    % Sue (doesn't care)
     | 1,  1,  3    % Robert
     | 3,  2,  1 |] % Alice
);
%% limits on min/max people per project
int: min_people = 1;
array[PROJECT] of min_people..card(PERSON): max_people = [2, 3, 2];

% what we are deciding
%% assigning people to projects
array[PERSON] of var PROJECT: person_to_project;

% constraints
%% min people per project
constraint forall(project in PROJECT)
  (min_people <= sum(person in PERSON)(person_to_project[person] = project));
%% max people per project
constraint forall(project in PROJECT)
  (sum(person in PERSON)(person_to_project[person] = project) <= max_people[project]);
%% note that we are also implicitly enforcing that every person is assigned
%% to a project because person_to_project always has a project value for each person

% objective
%% try to maximise the overall score, based on summing the satisfied preferences
%% of all people
var int: score = sum(p in PERSON)(preferences[p, person_to_project[p]]);
% var float: score = sum(p in PERSON)(preferences[p, assignment[p]]) / card(PERSON);

solve maximize score;

% output
%% helpful for display, but not involved in any constraint:
array[PERSON] of var PRIORITY: priorities = 
  [ preferences[p, person_to_project[p]]| p in PERSON ];
  
output ["\(person_to_project), \(score) = \(fix(priorities))"];