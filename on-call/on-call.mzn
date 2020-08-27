% Employees cannot be on-call for:
% More than 100 hours in each calendar month;
% More than 12 hours in each 24 hour period during the business week (Monday to Friday);
% More than 48 hours over the weekend (in other words, they cannot be on call the full weekend);
% 2 consecutive business days; or
% More than 2 weekend days in one calendar month

int: nEmployees = 6;
set of int: EMPLOYEE = 1..nEmployees;

int: nDays = 10; 
set of int: DAY = 1..nDays;
array[DAY] of var bool: business_day;
array[DAY] of var bool: weekend_day;
constraint forall(d in DAY)(weekend_day[d] = ((d mod 7 = 1) \/ (d mod 7 = 2)));
constraint forall(d in DAY)(business_day[d] = not weekend_day[d]);

% 00:00 -> 09:00: MORNING
% 09:00 -> 17:00: DAY_TIME
% 17:00 -> 00:00: EVENING
enum SHIFT = { OFF, MORNING, DAY_TIME, EVENING, MORNING_EVENING, ALL };
array[SHIFT] of int: hour_commitment 
           = [   0,       9,        8,       7,              16,  24 ];
int: full_day_coverage = 24;
int: morning_evening_coverage = 16;
array[EMPLOYEE,DAY] of var SHIFT: roster;

% all days must be covered
constraint forall(d in DAY where business_day[d])
  (sum(e in EMPLOYEE)(hour_commitment[roster[e,d]]) = morning_evening_coverage);
constraint forall(d in DAY, e in EMPLOYEE where business_day[d])
  (roster[e,d] in { OFF, MORNING, EVENING, MORNING_EVENING });
constraint forall(d in DAY where weekend_day[d])
  (sum(e in EMPLOYEE)(hour_commitment[roster[e,d]]) = full_day_coverage);
constraint forall(d in DAY, e in EMPLOYEE where weekend_day[d])
  (roster[e,d] in { OFF, ALL });

array[EMPLOYEE] of var int: commitment;
constraint forall(e in EMPLOYEE)(commitment[e] = sum(d in DAY)(hour_commitment[roster[e,d]]));

% avoid 2 consecutive business days
constraint forall(d in 1..(nDays-1), e in EMPLOYEE where (business_day[d] /\ business_day[d+1]))
  ((roster[e,d] != OFF) -> (roster[e,d+1] = OFF));

% avoid more than 12 hours in each 24 hour period during the business week
constraint forall(e in EMPLOYEE, d in DAY where business_day[d])
  (hour_commitment[roster[e,d]] <= 12);

% fairness
% constraint forall(e in EMPLOYEE)(commitment[e] < (nDays / nEmployees) + 1);

solve satisfy;