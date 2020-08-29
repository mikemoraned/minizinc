% Employees cannot be on-call for:
% More than 100 hours in each calendar month;
% More than 12 hours in each 24 hour period during the business week (Monday to Friday);
% More than 48 hours over the weekend (in other words, they cannot be on call the full weekend);
% 2 consecutive business days; or
% More than 2 weekend days in one calendar month

int: nEmployees = 6;
set of int: EMPLOYEE = 1..nEmployees;

int: nWeeks = 2;
int: nDays = nWeeks * 7; 
set of int: DAY = 1..nDays;

enum DAY_OF_WEEK = { MON, TUE, WED, THU, FRI, SAT, SUN };
array[DAY] of DAY_OF_WEEK: week_day = [ to_enum(DAY_OF_WEEK, 1 + (d mod 7)) | d in DAY];
enum DAY_TYPE = { BUSINESS, WEEKEND };
array[DAY_OF_WEEK] of DAY_TYPE: day_type = [ BUSINESS, BUSINESS, BUSINESS, BUSINESS, BUSINESS, WEEKEND, WEEKEND ];
% 00:00 -> 09:00: MORNING
% 09:00 -> 17:00: DAY_TIME
% 17:00 -> 00:00: EVENING
% MORNING_EVENING: MORNING, EVENING
% ALL: MORNING, DAY_TIME, EVENING
enum SHIFT = { OFF, MORNING, DAY_TIME, EVENING, MORNING_EVENING, ALL };
array[SHIFT] of int: hour_commitment 
           = [   0,       9,        8,       7,              16,  24 ];
int: full_day_coverage = 24;
int: morning_evening_coverage = 16;
array[EMPLOYEE,DAY] of var SHIFT: roster;

% all days must be covered
constraint forall(d in DAY where day_type[week_day[d]] = BUSINESS)
  (sum(e in EMPLOYEE)(hour_commitment[roster[e,d]]) = morning_evening_coverage);
constraint forall(d in DAY, e in EMPLOYEE where day_type[week_day[d]] = BUSINESS)
  (roster[e,d] in { OFF, MORNING, EVENING, MORNING_EVENING });
constraint forall(d in DAY where day_type[week_day[d]] = WEEKEND)
  (sum(e in EMPLOYEE)(hour_commitment[roster[e,d]]) = full_day_coverage);
constraint forall(d in DAY, e in EMPLOYEE where day_type[week_day[d]] = WEEKEND)
  (roster[e,d] in { OFF, ALL });


% avoid 2 consecutive business days
constraint forall(d in 1..(nDays-1), e in EMPLOYEE where (day_type[week_day[d]] = BUSINESS /\ day_type[week_day[d+1]] = BUSINESS))
  ((roster[e,d] != OFF) -> (roster[e,d+1] = OFF));

% avoid more than 12 hours in each 24 hour period during the business week
constraint forall(e in EMPLOYEE, d in DAY where day_type[week_day[d]] = BUSINESS)
  (hour_commitment[roster[e,d]] <= 12);

% fairness
array[EMPLOYEE] of var int: commitment;
constraint forall(e in EMPLOYEE)(commitment[e] = sum(d in DAY)(hour_commitment[roster[e,d]]));

% var int: max_disparity;
% constraint max_disparity = max([ abs(commitment[e1] - commitment[e2]) | e1 in EMPLOYEE, e2 in EMPLOYEE where e1 != e2]);

% solve minimize max_disparity;
solve satisfy;