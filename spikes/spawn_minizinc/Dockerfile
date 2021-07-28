FROM minizinc/minizinc:2.5.5

COPY nqueens.mzn .

RUN minizinc nqueens.mzn -D n=8 --verbose --time-limit 30000