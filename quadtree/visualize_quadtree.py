"""
This module, visualize_quadtree.py, draw the quadtree visualization with matplotlib.

Example:
    source .venv/bin/activate
    python visualize_quadtree.py ~/scratch/quadtree/quadtree_data.yaml

"""

import sys
from pathlib import Path
from typing import Final
import yaml

from matplotlib import rc
import matplotlib.colors as mcolors
import matplotlib.pyplot as plt
import matplotlib.patches as patches


def construct_quadtree_leaf(loader, node):
    """
    Register constructors for custom tags used by serde for enums.
    This tells PyYAML to treat !Leaf as regular mappings (diectionaries)
    """
    result = loader.construct_mapping(node)
    return {"Leaf": result}


def construct_quadtree_children(loader, node):
    """
    Register constructors for custom tags used by serde for enums.
    This tells PyYAML to treat !Children as regular mappings (diectionaries)
    """
    result = loader.construct_mapping(node)
    return {"Children": result}


yaml.FullLoader.add_constructor("!Leaf", construct_quadtree_leaf)
yaml.FullLoader.add_constructor("!Children", construct_quadtree_children)


def draw_quadtree(ax, node_data, colors, level=0):
    """Draws the quadtree with matplotlib."""
    # Use the modulo operator to cycle through the colors list.
    # This prevents an IndexError if the quadtree level is larger than the number of colors.
    current_color = colors[level % len(colors)]

    # Define line style for the first level
    linestyle = "dashed" if level == 0 else "solid"
    linewidth = 4 if level == 0 else 2

    # print(f"Drawing level: {level}")

    # Draw the boundary of the current node
    boundary = node_data["boundary"]
    rect = patches.Rectangle(
        (boundary["origin"]["x"], boundary["origin"]["y"]),
        boundary["width"],
        boundary["height"],
        linewidth=linewidth,
        edgecolor=current_color,
        linestyle=linestyle,
        facecolor="none",
        zorder=level,
    )
    ax.add_patch(rect)

    # Draw points if it's a Leaf node
    if "Leaf" in node_data["node"]:
        points = node_data["node"]["Leaf"]["points"]
        if points:
            x_coords = [p["x"] for p in points]
            y_coords = [p["y"] for p in points]
            ax.scatter(x_coords, y_coords, color=current_color, s=10)
    # Recursively draw children if it's a Children node
    elif "Children" in node_data["node"]:
        children = node_data["node"]["Children"]
        draw_quadtree(ax, children["nw"], colors, level + 1)
        draw_quadtree(ax, children["ne"], colors, level + 1)
        draw_quadtree(ax, children["sw"], colors, level + 1)
        draw_quadtree(ax, children["se"], colors, level + 1)


def draw_dual_vertices(ax, dual_vertices, color="black"):
    """Draws the dual vertices on the quadtree visualization."""
    if not dual_vertices:
        return

    x_coords = [p["x"] for p in dual_vertices]
    y_coords = [p["y"] for p in dual_vertices]
    ax.scatter(x_coords, y_coords, color=color, s=25, zorder=10, marker="o")


def draw_dual_edges(ax, dual_edges, color="darkgray"):
    """Draws the dual edges on the quadtree visualization."""
    if not dual_edges:
        return

    for edge in dual_edges:
        p1, p2 = edge
        ax.plot(
            [p1["x"], p2["x"]],
            [p1["y"], p2["y"]],
            color=color,
            linewidth=1,
            zorder=9,
        )


def main():
    """The main entry point for the script."""
    if len(sys.argv) < 2:
        print("Usage: python3 visualize_quadtree.py <path_to_yaml_file>")
        sys.exit(1)

    dpi: Final[int] = 300
    show: Final[bool] = False
    save: Final[bool] = True
    latex: Final[bool] = True
    if latex:
        rc("font", **{"family": "serif", "serif": ["Computer Modern Roman"]})
        rc("text", usetex=True)

    yaml_file_path = sys.argv[1]
    print(f"Processing file: {yaml_file_path}")

    file_path = Path(yaml_file_path)
    filename_stem = file_path.stem  # removes the .yaml extension
    print(f"Filename stem: {filename_stem}")

    try:
        with open(yaml_file_path, "r", encoding="utf8") as f:
            viz_data = yaml.load(
                f, Loader=yaml.FullLoader
            )  # needed to resolve custom tags
        print("Python successfully loaded YAML data.")
        # print("Python successfully loaded YAML data:")
        # print(json.dumps(quadtree_data, indent=2))
    except FileNotFoundError:
        print(f"Error: YAML file not found at {yaml_file_path}")
        sys.exit(1)
    except yaml.YAMLError as e:
        print(
            f"Error: Could not decode YAML from {yaml_file_path}.  Error details: {e}"
        )
        sys.exit(1)

    quadtree_data = viz_data["quadtree"]
    _level_max = quadtree_data["level_max"]

    fig, ax = plt.subplots(figsize=(6.0, 6.0), dpi=dpi)
    ax.set_aspect("equal", adjustable="box")

    # Define colors for each level
    # colors = ["tab:blue", "tab:orange", "tab:green", "tab:red"]
    colors = list(mcolors.TABLEAU_COLORS.keys())

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

    # Draw the primary quadtree
    draw_quadtree(ax, quadtree_data, colors)

    # Draw dual vertices and edges if they exist
    draw_dual_vertices(ax, viz_data.get("dual_vertices"))
    draw_dual_edges(ax, viz_data.get("dual_edges"))

    # plt.title("Quadtree Visualization")
    # plt.title(f"level max = {level_max}")
    plt.title(f"{filename_stem}")
    plt.xlabel(r"$x$")
    plt.ylabel(r"$y$")
    # plt.grid(True)
    plt.grid(False)
    if show:
        plt.show()

    if save:
        extension = ".png"  # ".png" | ".pdf" | ".svg"
        output_path = file_path.parent  # same directory as input file
        filename = filename_stem + extension
        pathfilename = output_path.joinpath(filename)
        fig.savefig(pathfilename, bbox_inches="tight", pad_inches=0)
        print(f"Serialized to {pathfilename}")


if __name__ == "__main__":
    main()
