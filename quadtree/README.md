# Quadtree

This is a continuation of the historical documents:

* [quadtree](https://github.com/sandialabs/sibl/blob/master/geo/doc/quadtree.md)
* [primal/dual quadrilateral transitions](https://github.com/sandialabs/sibl/blob/master/geo/doc/dual_quad_transitions.md)

The current code:

* [src/lib.rs](src/lib.rs) create and write a quadtree
* [visualize_quadtree.py](visualize_quadtree.py) create *MATPLOTLIB* visualualization

## Example

* `L0` square domain $(x, y) \in ([1, 3] \otimes  [-1, 1])$
* Single point at `(2.6, 0.6)` to trigger refinement.

&nbsp; | &nbsp; | &nbsp;
:---: | :---: | :---:
![](img/visualize_quadtree_L0.png) | ![](img/visualize_quadtree_L1.png) | ![](img/visualize_quadtree_L2.png)
![](img/visualize_quadtree_L3.png) | ![](img/visualize_quadtree_L4.png) | ![](img/visualize_quadtree_L5.png)

[quadtree_data_L0.yaml](data/quadtree_data_L0.yaml)

```yml
boundary:
  origin:
    x: 1.0
    y: -1.0
  width: 2.0
  height: 2.0
level: 0
level_max: 0
node: !Leaf
  points:
  - x: 2.6
    y: 0.6
```

[quadtree_data_L1.yaml](data/quadtree_data_L2.yaml)

```yml
boundary:
  origin:
    x: 1.0
    y: -1.0
  width: 2.0
  height: 2.0
level: 0
level_max: 1
node: !Children
  nw:
    boundary:
      origin:
        x: 1.0
        y: 0.0
      width: 1.0
      height: 1.0
    level: 1
    level_max: 1
    node: !Leaf
      points: []
  ne:
    boundary:
      origin:
        x: 2.0
        y: 0.0
      width: 1.0
      height: 1.0
    level: 1
    level_max: 1
    node: !Leaf
      points:
      - x: 2.6
        y: 0.6
  sw:
    boundary:
      origin:
        x: 1.0
        y: -1.0
      width: 1.0
      height: 1.0
    level: 1
    level_max: 1
    node: !Leaf
      points: []
  se:
    boundary:
      origin:
        x: 2.0
        y: -1.0
      width: 1.0
      height: 1.0
    level: 1
    level_max: 1
    node: !Leaf
      points: []
```

## Balancing

The **balance constraint** is stated as,

* **Weakly balanced quadtree (2:1 balance)**: For a leaf cell `L` at level `l`, any edge-adjacent neighbor cell `N` at level `n` must satisfy `|l - n| ≤ 1`.
  * Equivalently, if `L` has level `l`, all edge-adjacent neighbors in the four cardinal directions (north, south, east, west) must have levels in the range `[l-1, l+1]`.
* **Strongly balanced quadtree**: For a leaf cell `L` at level `l`, any edge-adjacent OR corner-adjacent neighbor cell `N` at level `n` must satisfy `|l - n| ≤ 1`. This includes all eight directions: north, south, east, west, and the four diagonal directions (northeast, northwest, southeast, southwest).

**Boundary considerations**: At domain boundaries, a leaf may have fewer than 4 (weakly balanced) or 8 (strongly balanced) neighbors. The constraints apply only to existing neighbors. This is typically understood but worth noting for implementation.

**"Leaf cell" specification**: The balance constraint applies only to a leaf node in the quadtree, not to internal nodes that have children.
