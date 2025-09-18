"""
This module, visualize_quadtree.py, draw the quadtree visualization with matplotlib.

Example:
    source .venv/bin/activate
    python visualize_quadtree.py ~/scratch/quadtree/quadtree_data.yaml

"""

import json
import sys

import matplotlib.pyplot as plt
import matplotlib.patches as patches
import yaml


# Register constructors for custom tags used by serde for enums.
# This tells PyYAML to treat !Leaf and !Children as regular mappings (diectionaries)
def construct_quadtree_leaf(loader, node):
    """
    Register constructors for custom tags used by serde for enums.
    This tells PyYAML to treat !Leaf and !Children as regular mappings (diectionaries)
    """
    result = loader.construct_mapping(node)
    return {"Leaf": result}


def construct_quadtree_children(loader, node):
    """
    Register constructors for custom tags used by serde for enums.
    This tells PyYAML to treat !Leaf and !Children as regular mappings (diectionaries)
    """
    result = loader.construct_mapping(node)
    return {"Children": result}


yaml.FullLoader.add_constructor("!Leaf", construct_quadtree_leaf)
yaml.FullLoader.add_constructor("!Children", construct_quadtree_children)


def draw_quadtree(ax, node_data, level=0):
    """Draws the quadtree with matplotlib."""
    # Define colors for each level
    colors = ["tab:blue", "tab:orange", "tab:green", "tab:red"]

    # Define line style for the first level
    linestyle = "dashed" if level == 0 else "solid"
    linewidth = 5 if level == 0 else 2

    print(f"Drawing level: {level}")

    # Draw the boundary of the current node
    boundary = node_data["boundary"]
    rect = patches.Rectangle(
        (boundary["origin"]["x"], boundary["origin"]["y"]),
        boundary["width"],
        boundary["height"],
        linewidth=linewidth,
        edgecolor=colors[level],
        linestyle=linestyle,
        facecolor="none",
    )
    ax.add_patch(rect)

    breakpoint()

    # Draw points if it's a Leaf node
    if "Leaf" in node_data["node"]:
        points = node_data["node"]["Leaf"]["points"]
        if points:
            x_coords = [p["x"] for p in points]
            y_coords = [p["y"] for p in points]
            ax.scatter(x_coords, y_coords, color=colors[level], s=10)
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
        print("Usage: python3 visualize_quadtree.py <path_to_yaml_file>")
        sys.exit(1)

    yaml_file_path = sys.argv[1]
    print(f"Processing file: {yaml_file_path}")

    try:
        with open(yaml_file_path, "r", encoding="utf8") as f:
            # quadtree_data = yaml.safe_load(f)
            quadtree_data = yaml.load(
                f, Loader=yaml.FullLoader
            )  # needed to resolve custom tags
        print("Python successfully loaded YAML data:")
        print(json.dumps(quadtree_data, indent=2))
    except FileNotFoundError:
        print(f"Error: YAML file not found at {yaml_file_path}")
        sys.exit(1)
    except yaml.YAMLError as e:
        print(
            f"Error: Could not decode YAML from {yaml_file_path}.  Error details: {e}"
        )
        sys.exit(1)

    fig, ax = plt.subplots(figsize=(6.0, 6.0), dpi=100)
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
    # plt.grid(True)
    plt.grid(False)
    plt.show()


if __name__ == "__main__":
    main()
