"""This module, visualize_quadtree.py, draw the quadtree visualization with matplotlib."""

import sys
import json
import matplotlib.pyplot as plt
import matplotlib.patches as patches


def draw_quadtree(ax, node_data, level=0):
    """Draws the quadtree with matplotlib."""
    # Draw the boundary of the current node
    boundary = node_data["boundary"]
    rect = patches.Rectangle(
        (boundary["origin"]["x"], boundary["origin"]["y"]),
        boundary["width"],
        boundary["height"],
        linewidth=1,
        edgecolor="r" if level == 0 else "b",
        facecolor="none",
    )
    ax.add_patch(rect)

    # Draw points if it's a Leaf node
    if "Leaf" in node_data["node"]:
        points = node_data["node"]["Leaf"]["points"]
        if points:
            x_coords = [p["x"] for p in points]
            y_coords = [p["y"] for p in points]
            ax.scatter(x_coords, y_coords, color="g", s=10)
    # Recursively draw children if it's a Children node
    elif "Children" in node_data["node"]:
        children = node_data["node"]["Children"]
        draw_quadtree(ax, children["nw"], level + 1)
        draw_quadtree(ax, children["ne"], level + 1)
        draw_quadtree(ax, children["sw"], level + 1)
        draw_quadtree(ax, children["se"], level + 1)


def main():
    """The main entry point for the scrirpt."""
    if len(sys.argv) < 2:
        print("Usage: python3 visualize_quadtree.py <path_to_json_file>")
        sys.exit(1)

    json_file_path = sys.argv[1]
    print(f"Processing file: {json_file_path}")

    try:
        with open(json_file_path, "r", encoding="utf8") as f:
            quadtree_data = json.load(f)
    except FileNotFoundError:
        print(f"Error: JSON file not found at {json_file_path}")
        sys.exit(1)
    except json.JSONDecodeError:
        print(f"Error: Could not decode JSON from {json_file_path}")
        sys.exit(1)

    fig, ax = plt.subplots(1)
    ax.set_aspect("equal", adjustable="box")

    # Set plot limits based on the root boundary
    root_boundary = quadtree_data["boundary"]
    ax.set_xlim(
        root_boundary["origin"]["x"],
        root_boundary["origin"]["x"] + root_boundary["width"],
    )
    ax.set_ylim(
        root_boundary["origin"]["y"],
        root_boundary["origin"]["y"] + root_boundary["height"],
    )

    draw_quadtree(ax, quadtree_data)

    plt.title("Quadtree Visualization")
    plt.xlabel("x")
    plt.ylabel("y")
    plt.grid(True)
    plt.show()


if __name__ == "__main__":
    main()
