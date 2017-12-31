## 算法练习本

#### [算法（第4版）](http://www.ituring.com.cn/book/875)

* [排序](https://github.com/nanlong/arithmetic_rs/tree/master/src/sort)
    * [快排](https://github.com/nanlong/arithmetic_rs/blob/master/src/sort/quick_sort.rs)
    * [冒泡](https://github.com/nanlong/arithmetic_rs/blob/master/src/sort/bubble_sort.rs)
    * [堆排](https://github.com/nanlong/arithmetic_rs/blob/master/src/sort/heap_sort.rs)
    * [归并](https://github.com/nanlong/arithmetic_rs/blob/master/src/sort/merge_sort.rs)
* 队列
    * [索引优先队列](https://github.com/nanlong/arithmetic_rs/blob/master/src/queue/index_binary_heap.rs)
    * [最小索引优先队列](https://github.com/nanlong/arithmetic_rs/blob/master/src/queue/index_min_pq.rs)

* [树](https://github.com/nanlong/arithmetic_rs/tree/master/src/tree)
    * [二分搜索树](https://github.com/nanlong/arithmetic_rs/blob/master/src/tree/binary_search_tree.rs)
    * [AVL树](https://github.com/nanlong/arithmetic_rs/blob/master/src/tree/avl_tree.rs)
    * [红黑树](https://github.com/nanlong/arithmetic_rs/blob/master/src/tree/red_black_tree.rs)
    
* [图](https://github.com/nanlong/arithmetic_rs/tree/master/src/graph)
    * [并查集](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/union_find.rs)
    * [无向图](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/graph.rs)
        * [深度优先搜索](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/depth_first_search.rs)
        * [深度优先路径](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/depth_first_paths.rs)
        * [广度优先路径](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/breadth_first_paths.rs)
        * [连通分量](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/cc.rs)
        * [环检测](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/cycle.rs)
        * [二分图检测](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/two_color.rs)
    * [符号图](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/symbol_graph.rs)
    * 有向图
    * 加权图
        * [边](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/edge.rs)
        * [加权无向图](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/edge_weighted_graph.rs)
        * 最小生成树
            * [Prim 算法（延迟版本）](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/lazy_prim_mst.rs)
            * [Prim 算法（即时版本）](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/prim_mst.rs)
            * [Kruskal 算法](https://github.com/nanlong/arithmetic_rs/blob/master/src/graph/kruskal_mst.rs)