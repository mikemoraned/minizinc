% Employees cannot be on-call for:
% More than 100 hours in each calendar month;
% More than 12 hours in each 24 hour period during the business week (Monday to Friday);
% More than 48 hours over the weekend (in other words, they cannot be on call the full weekend);
% 2 consecutive business days; or
% More than 2 weekend days in one calendar month

int: nEmployees = 6;
set of int: EMPLOYEE = 1..nEmployees;

int: nDays = 4 * 7; % 4 * 7 is taken "as one calendar month"
set of int: DAY = 1..nDays;

enum DAY_OF_WEEK = { MON, TUE, WED, THU, FRI, SAT, SUN };
array[DAY] of DAY_OF_WEEK: day_of_week = [ to_enum(DAY_OF_WEEK, 1 + ((d - 1 ) mod card(DAY_OF_WEEK))) | d in DAY];
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
array[DAY_OF_WEEK, SHIFT] of bool: is_weekend_shift = 
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
array[DAY,SHIFT] of var ASSIGNED: assignment;

% all expected shifts must be covered
constraint forall(d in DAY, s in SHIFT where shift_expected[day_of_week[d],s])
  (assignment[d,s] != no_employee);
constraint forall(d in DAY, s in SHIFT where not shift_expected[day_of_week[d],s])
  (assignment[d,s] = no_employee);

% avoid 2 consecutive business days
constraint forall(d in 1..nDays-1, s in SHIFT where (is_business_day[day_of_week[d]] /\ is_business_day[day_of_week[d+1]]))
  (forall(e in EMPLOYEE)
    (assignment[d,s] = e -> assignment[d+1,s] != e));

% avoid more than 12 hours in each 24 hour period during the business week
constraint forall(e in EMPLOYEE, d in DAY where is_business_day[day_of_week[d]])
  (sum(s in SHIFT where assignment[d,s] = e)(shift_hour_commitment[s]) <= 12);

% avoid more than 48 hours over the weekend (in other words, they cannot be on call the full weekend)
constraint forall(e in EMPLOYEE, d in 1..nDays-2 where day_of_week[d] = FRI)
  (sum(w_d in d..d+2, s in SHIFT where is_weekend_shift[day_of_week[w_d],s] /\ assignment[w_d,s] = e)
    (shift_hour_commitment[s]) <= 48);
  
% avoid more than 2 weekend days in one calendar month for any employee
% constraint forall(e in EMPLOYEE)
%   (count([ is_weekend_day[day_of_week[d]] /\ (exists(s in SHIFT)(assignment[d,s] = e)) | d in DAY]) <= 2);

    
array[EMPLOYEE] of var int: commitment;
constraint forall(e in EMPLOYEE)
  (commitment[e] = sum(d in DAY, s in SHIFT where assignment[d,s] = e)(shift_hour_commitment[s]));

% avoid more than 100 hours in each calendar month
constraint forall(e in EMPLOYEE)
  (commitment[e] <= 100);

% fairness
var int: max_disparity;
constraint max_disparity = max([ abs(commitment[e1] - commitment[e2]) | e1 in EMPLOYEE, e2 in EMPLOYEE where e1 != e2]);
var float: avg_disparity;
constraint avg_disparity = 
  sum([ abs(commitment[e1] - commitment[e2]) | e1 in EMPLOYEE, e2 in EMPLOYEE where e1 != e2])
    / (card(EMPLOYEE) * card(EMPLOYEE));

% churn
int: flattened_size = nDays * card(SHIFT);
array[1..flattened_size] of var ASSIGNED: flattened = [ assignment[d, s] | d in DAY, s in SHIFT];
var int: consistency;
constraint consistency = sum(d in 1..flattened_size-1)(flattened[d] = flattened[d+1]);

solve maximize consistency;

% solve minimize avg_disparity;
% solve satisfy;

output [ "\(d), \(day_of_week[d]): \(assignment[d, MORNING]), \(assignment[d, DAY_TIME]), \(assignment[d, EVENING])\n" | d in DAY];
output [ "commitment = \(commitment)" ];

% output ["\(flattened)"];