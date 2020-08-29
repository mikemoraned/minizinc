% Employees cannot be on-call for:
% More than 100 hours in each calendar month;
% More than 12 hours in each 24 hour period during the business week (Monday to Friday);
% More than 48 hours over the weekend (in other words, they cannot be on call the full weekend);
% 2 consecutive business days; or
% More than 2 weekend days in one calendar month

int: nEmployees = 6;
set of int: EMPLOYEE = 1..nEmployees;

int: nWeeks = 2;
set of int: WEEK = 1..nWeeks;

enum DAY_OF_WEEK = { MON, TUE, WED, THU, FRI, SAT, SUN };
enum SHIFT = { MORNING, DAY_TIME, EVENING };
array[SHIFT] of int: shift_hour_commitment = 
             [       9,        8,       7 ];
int: full_day_coverage = 24;
int: morning_evening_coverage = 16;

array[DAY_OF_WEEK, SHIFT] of bool: shift_expected = 
 [| true, false, true,    % MON
  | true, false, true,    % TUE
  | true, false, true,    % WED
  | true, false, true,    % THU
  | true, false, true,    % FRI
  | true,  true, true,    % SAT
  | true,  true, true |]; % SUN
array[DAY_OF_WEEK, SHIFT] of DAY_TYPE: is_weekend_shift = 
 [|false, false, false,    % MON
  |false, false, false,    % TUE
  |false, false, false,    % WED
  |false, false, false,    % THU
  |false, false,  true,    % FRI
  | true,  true,  true,    % SAT
  | true,  true,  true |]; % SUN
array[DAY_OF_WEEK] of bool: is_weekend_day = 
 [false, false, false, false, false, true, true];
array[DAY_OF_WEEK] of bool: is_business_day = [ not is_weekend_day[d] | d in DAY_OF_WEEK ];
 
set of int: ASSIGNED = 0..nEmployees;
int: no_employee = 0;
array[WEEK,DAY_OF_WEEK,SHIFT] of var EMPLOYEE: assignment;

% all expected shifts must be covered
constraint forall(w in WEEK, d in DAY_OF_WEEK, s in SHIFT where shift_expected[d,s])
  (assignment[w,d,s] != no_employee);
constraint forall(w in WEEK, d in DAY_OF_WEEK, s in SHIFT where not shift_expected[d,s])
  (assignment[w,d,s] = no_employee);

% avoid 2 consecutive business days
constraint forall(w in WEEK, d in DAY_OF_WEEK, s in SHIFT where is_business_day[d])
  (assignment[w,d,s] -> (roster[e,d+1] = OFF));

% avoid more than 12 hours in each 24 hour period during the business week
constraint forall(e in EMPLOYEE, d in DAY where day_type[week_day[d]] = BUSINESS)
  (hour_commitment[roster[e,d]] <= 12);

% fairness
% array[EMPLOYEE] of var int: commitment;
% constraint forall(e in EMPLOYEE)(commitment[e] = sum(d in DAY)(hour_commitment[roster[e,d]]));

% var int: max_disparity;
% constraint max_disparity = max([ abs(commitment[e1] - commitment[e2]) | e1 in EMPLOYEE, e2 in EMPLOYEE where e1 != e2]);

% solve minimize max_disparity;
solve satisfy;