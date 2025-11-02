# Ant Mania Simulation

## Assumptions

- When ant moves it chooses direction with equal probabilities
- If it can move it moves
- They move in random order, one ant can move multiple times when other ants don't move at all. **This affects simulation time**
- Number of ants can vary from 1 to number of nodes in the graph
- Ant can move more than 10,000 times, simulation ends when all moved at least 10,000, are trapped ore dead.

## Design Decisions

I decided NOT to use parallelism for three reasons:

- It wouldn't be easy to keep the simulation fully random
- 3 hours is could be not enough to design parallel algorithm correctly
- Individual ant moves are small tasks, parallelism overhead could make it not worth

## Algorithm

1. Choose alive, non-trapped ant randomly
2. Move it to random neighbor
3. Handle fights if collision occurs
4. Update counters and remove finished ants

**Time complexity**: O(M) where M is total number of moves across all ants.


## Benchmarks

📊 BENCHMARK RESULTS 📊
=======================
Map                       Ants         Avg (ms)     Min (ms)     Max (ms)    
───────────────────────────────────────────────────────────────────────────
hiveum_map_large.txt      1            0.39         0.26         0.59        
hiveum_map_large.txt      10           3.76         2.74         5.07        
hiveum_map_large.txt      100          17.80        13.02        23.01       
hiveum_map_large.txt      1000         39.52        32.20        53.13       
hiveum_map_large.txt      10000        62.69        49.67        74.71       
hiveum_map_large.txt      100000       1382.04      1131.01      1789.27     
hiveum_map_very_large.txt 1            0.29         0.21         0.38        
hiveum_map_very_large.txt 10           3.68         2.64         5.78        
hiveum_map_very_large.txt 100          44.88        34.81        70.36       
hiveum_map_very_large.txt 1000         192.93       175.09       227.36      
hiveum_map_very_large.txt 10000        466.91       347.06       677.31      
hiveum_map_very_large.txt 100000       801.04       614.75       989.78      
hiveum_map_very_large.txt 1000000      25983.08     22529.52     38546.01 


each set was run 10 times to find Abg, Min, and Max.

processor: AMD Ryzen 7 8845HS

