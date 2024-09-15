using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using static Problem.PROBLEM_CLASS;

namespace Problem
{
    public static class PROBLEM_CLASS
    {
        public class Landmark
        {
            public int Id;
            public int X, Y;
            public bool IsInside;

            public Landmark(int id, int x, int y, bool isInside)
            {
                Id = id;
                X = x;
                Y = y;
                IsInside = isInside;
            }
        }


        #region YOUR CODE IS HERE
        //Your Code is Here:
        //==================
        /// <summary>
        /// Find the shortest path from "goerge" to any of the landmarks that is outside the Honor Stone 
        /// </summary>
        /// <param name="landmarks">list of Landmarks, each with Id, x, y, IsInside </param>
        /// <param name="trails">list of all trails, each consists of landmark1, landmark2, length</param>
        /// <param name="N">number of landmarks</param>
        /// <returns>value of the shortest path from goerge to outside </returns>
        public static int RequiredFunction(List<Landmark> landmarks, List<Tuple<int, int, int>> trails, int N)
        {
            //REMOVE THIS LINE BEFORE START CODING
            //throw new NotImplementedException();

            Dictionary<int, int> storage = new Dictionary<int, int>();
            storage.Add(0, 0);
            int[] d = new int[N];
            d[0] = 0;
            for (int i = 1; i < N; i++)
            {
                d[i] = int.MaxValue;
                storage.Add(i, int.MaxValue);
            }

            Dictionary<int, List<int>> adj = new Dictionary<int, List<int>>();
            Dictionary<(int, int), int> edge_weights = new Dictionary<(int, int), int>();


            foreach (Tuple<int, int, int> tuple in trails)
            {
                if (isFurtherEuclidian(tuple.Item2, tuple.Item1, landmarks))
                {
                    if (!adj.ContainsKey(tuple.Item1))
                    {
                        adj.Add(tuple.Item1, new List<int>());
                    }
                    adj[tuple.Item1].Add(tuple.Item2);
                    edge_weights.Add((tuple.Item1, tuple.Item2), tuple.Item3);
                }
            }

            int min = int.MaxValue;
            int result = int.MaxValue;
            while (storage.Count > 0)
            {
                int u = -1;

                foreach (int key in storage.Keys)
                {
                    if (storage[key] < min)
                    {
                        u = key;
                        min = storage[key];
                    }
                }

                if (u == -1)
                {
                    break;
                }

                min = int.MaxValue;
                storage.Remove(u);


                if (adj.ContainsKey(u))
                {
                    foreach (int v in adj[u])
                    {
                        if (d[u] + edge_weights[(u, v)] < d[v])
                        {
                            storage[v] = d[u] + edge_weights[(u, v)];
                            d[v] = d[u] + edge_weights[(u, v)];
                            if (!landmarks[v].IsInside)
                            {
                                if (d[v] < result)
                                {
                                    result = d[v];
                                }
                            }
                        }
                    }
                }
            }

            return result;

        }

        public static bool isFurtherEuclidian(int vert1, int vert2, List<Landmark> landmarks)
        {
            int x1 = landmarks[vert1].X;
            int y1 = landmarks[vert1].Y;
            int x2 = landmarks[vert2].X;
            int y2 = landmarks[vert2].Y;
            int x = landmarks[0].X;
            int y = landmarks[0].Y;

            double dist1 = Math.Sqrt((Math.Pow(x1 - x, 2) + Math.Pow(y1 - y, 2)));
            double dist2 = Math.Sqrt((Math.Pow(x2 - x, 2) + Math.Pow(y2 - y, 2)));

            return dist2 < dist1;
        }

        #endregion
    }

}
