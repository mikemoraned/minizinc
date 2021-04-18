from minizinc import Instance, Model, Solver
import pprint

# Load n-Queens model from file
nqueens = Model("./nqueens.mzn")

pp = pprint.PrettyPrinter(indent=4)
pp.pprint(nqueens["n"])

# Find the MiniZinc solver configuration for Gecode
gecode = Solver.lookup("gecode")
# Create an Instance of the n-Queens model for Gecode
instance = Instance(gecode, nqueens)

# Assign 4 to n
instance["n"] = 4

pp.pprint(instance["n"])

result = instance.solve()
# Output the array q
print(result["q"])
