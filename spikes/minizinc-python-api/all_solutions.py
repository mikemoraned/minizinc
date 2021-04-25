from minizinc import Instance, Model, Solver

gecode = Solver.lookup("gecode")

nqueens = Model("./nqueens.mzn")
instance = Instance(gecode, nqueens)
instance["n"] = 4

# Find and print all possible solutions
result = instance.solve(all_solutions=True)
for i in range(len(result)):
    print(result[i, "q"])
