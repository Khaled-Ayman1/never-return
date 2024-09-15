# Never Return
Bimsa, a young lioness, has been banished from her home, the land of Honor Stone, by her evil 
uncle. He warned her to "Run away, Bimsa. Run away, and never return." Now, Bimsa must 
navigate her way out of Honor Stone, using her knowledge of the landmarks and trails in the area.

# Your Task
You are to help Bimsa find the quickest way out of Honor Stone. She's starting from a place known as the gorge and wants to leave the area without going back on her path. Bimsa 
knows the location of each landmark (N in total) (given as x and y coordinates) and can tell if it's 
inside or outside Honor Stone. She also knows the lengths of the trails between landmarks and 
that each landmark can have up to five connecting trails. Trails can be traveled in both directions, 
but Bimsa will only move from one landmark to another if it takes her farther from the gorge, 
following the "never return" rule.

# Technical Details
"Never Return" Rule: Bimsa moves from landmark A to landmark B only if landmark B is farther from the gorge than landmark A (according to the Euclidean Distance).

The Euclidean distance between two landmarks (x1, y1) and (x2, y2) 

is calculated as 
![image](https://github.com/y0sif/never-return/assets/61329766/91b173f5-674b-4527-96c8-ccd0b560624b) .

# Objective
Design an efficient algorithm to find the distance of the shortest path for Bimsa to leave Honor Stone by traversing from the gorge to any landmark outside, without ever returning. 
The path should strictly follow the "never return" rule, moving from closer landmarks to farther ones until Bimsa is outside Honor Stone.

This task combines knowledge of geography (landmarks and distances) with algorithmic strategy to ensure Bimsa's successful escape following the constraints and rules provided.

# Notes
gorge is always vertex 0

Edges is bidirectional

# Complexity
The complexity of your algorithm should be O(N). where N is the number of landmarks

# Where to Start?
all code to be written inside solution.rs

![image](https://github.com/y0sif/never-return/assets/61329766/044f79ac-df8d-4226-bbb5-4c85934272f0)

solution.rs includes:

![image](https://github.com/y0sif/never-return/assets/61329766/d01b13c7-23aa-4fd5-9027-5a7897d9973e)

landmarks: is list of Landmark 

trails: List of edges in the graph, each edge is a connects two landmarks (Tuple where 
Item1: index of landmark1, Item2: index of landmark2, Item3: weight of this edge)

n: the count of vertices.

# Example

![image](https://github.com/y0sif/never-return/assets/61329766/92203c82-860d-45a4-b481-d0aa5e4aece9)


Result = 5037

Because the path is from 0 to 1 and from 1 to 3 = 3245 + 1792 = 5037.

# How to run tests?
use the following command

```
env RUST_TEST_NOCAPTURE=1; cargo test
```

for release test use the following command

```
env RUST_TEST_NOCAPTURE=1; cargo test --release
```

