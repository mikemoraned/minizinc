enum PERSON = { Bob, Sue, Alice };
enum PROJECT = { P1, P2, P3 };

array[PERSON] of var PROJECT: projects;

include "globals.mzn";
constraint alldifferent(projects);