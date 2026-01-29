import sys
import os
import json
import matplotlib.pyplot as plt
import matplotlib.cm as cm
import matplotlib.colors as mcolors
from mpl_toolkits.mplot3d import Axes3D
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
import numpy as np
from scipy.spatial import ConvexHull, QhullError

class Visualizer:
    def __init__(self, data):
        self.data = data
        self.nodes = {n['id']: np.array([n['x'], n['y'], n['z']]) for n in data['nodes']}
        self.subgroups = {s['id']: s for s in data['subgroups']}
        
        profits = [s['profit'] for s in self.subgroups.values()]
        
        if not profits: 
            v_min, v_max = 0, 1
        else:
            v_min, v_max = min(profits), max(profits)
            if v_min == v_max: v_max += 0.1

        self.norm = mcolors.Normalize(vmin=v_min, vmax=v_max)
        self.cmap_heat = plt.get_cmap("coolwarm") 

        self.route_colors = plt.cm.tab20.colors

    def _get_subgroup_color(self, profit):
        return self.cmap_heat(self.norm(profit))

    def _get_route_color(self, index):
        return self.route_colors[index % len(self.route_colors)]

    def _draw_subgroup_2d(self, ax, pts, color):
        if len(pts) == 1:
            ax.plot(pts[0, 0], pts[0, 1], 'o', color=color, markersize=6, alpha=0.9)
            return

        if len(pts) == 2:
            ax.plot(pts[:, 0], pts[:, 1], 'o-', color=color, linewidth=3, markersize=4, alpha=0.8)
            return

        try:
            hull = ConvexHull(pts[:, :2])
            ax.fill(pts[hull.vertices, 0], pts[hull.vertices, 1], color=color, alpha=0.6, zorder=2)
            # Borda leve
            loop = np.append(hull.vertices, hull.vertices[0])
            ax.plot(pts[loop, 0], pts[loop, 1], '-', color=color, linewidth=1)
        except QhullError:
            ax.plot(pts[:, 0], pts[:, 1], 'o-', color=color, linewidth=2, alpha=0.7)

    def _draw_subgroup_3d(self, ax, pts, color):
        if len(pts) == 1:
            ax.scatter(pts[0, 0], pts[0, 1], pts[0, 2], c=[color], s=40, depthshade=False)
            return

        if len(pts) == 2:
            ax.plot(pts[:, 0], pts[:, 1], pts[:, 2], color=color, linewidth=3, alpha=0.9)
            return

        if len(pts) == 3:
            tri = Poly3DCollection([pts], alpha=0.6)
            tri.set_facecolor(color)
            tri.set_edgecolor(color)
            ax.add_collection3d(tri)
            return

        try:
            hull = ConvexHull(pts)
            triangles = [pts[s] for s in hull.simplices]
            mesh = Poly3DCollection(triangles, alpha=0.5)
            mesh.set_facecolor(color)
            mesh.set_edgecolor(color) 
            ax.add_collection3d(mesh)
        except QhullError:
            ax.plot(pts[:, 0], pts[:, 1], pts[:, 2], 'o--', color=color, alpha=0.5)

    def plot_2d(self):
        fig, ax = plt.subplots(figsize=(12, 10))
        
        for c in self.data['clusters']:
            points = self._get_cluster_points(c)
            if len(points) >= 3:
                try:
                    hull = ConvexHull(points[:, :2])
                    v = np.append(hull.vertices, hull.vertices[0])
                    ax.plot(points[v, 0], points[v, 1], color='#AAAAAA', linestyle='--', linewidth=1, zorder=1)
                except QhullError: pass
            
            if len(points) > 0:
                cx, cy = np.mean(points[:,0]), np.mean(points[:,1])
                ax.text(cx, cy + 2, f"C{c['id']}", fontsize=9, color='gray', fontweight='bold', ha='center')

        count_sub = 0
        for s in self.subgroups.values():
            pts = np.array([self.nodes[nid] for nid in s['node_ids']])
            if len(pts) == 0: continue
            
            color = self._get_subgroup_color(s['profit'])
            self._draw_subgroup_2d(ax, pts, color)
            count_sub += 1
            
        for nid, n in self.nodes.items():
            ax.scatter(n[0], n[1], c='black', marker='o', s=20, zorder=3)

        for i, route in enumerate(self.data['routes']):
            valid_pts = [self.nodes[nid] for nid in route if nid in self.nodes]
            if not valid_pts: continue
            pts = np.array(valid_pts)
            c = self._get_route_color(i)
            ax.plot(pts[:,0], pts[:,1], '-', color=c, linewidth=2, label=f'Vehicles {i}', zorder=4)

        sm = cm.ScalarMappable(cmap=self.cmap_heat, norm=self.norm)
        sm.set_array([])
        cbar = plt.colorbar(sm, ax=ax, fraction=0.03, pad=0.04)
        cbar.set_label('Reward', fontsize=10)

        ax.set_title(f"{len(self.data['routes'])} Vehicles", fontsize=14)
        ax.set_xlabel("X"); ax.set_ylabel("Y")
        if len(self.data['routes']) <= 20:
            ax.legend(loc='upper right', framealpha=1.0, fontsize=9, ncol=2)
        ax.grid(True, linestyle=':', alpha=0.4)
        plt.tight_layout()
        plt.show()

    def plot_3d(self):
        fig = plt.figure(figsize=(12, 10))
        ax = fig.add_subplot(111, projection='3d')

        for c in self.data['clusters']:
            points = self._get_cluster_points(c)
            if len(points) >= 4:
                try:
                    hull = ConvexHull(points)
                    for simplex in hull.simplices:
                        cycle = np.append(simplex, simplex[0])
                        ax.plot(points[cycle, 0], points[cycle, 1], points[cycle, 2], 
                                color='#CCCCCC', linestyle='--', linewidth=0.8, alpha=0.5)
                except QhullError: pass

        count_sub = 0
        for s in self.subgroups.values():
            pts = np.array([self.nodes[nid] for nid in s['node_ids']])
            if len(pts) == 0: continue
            color = self._get_subgroup_color(s['profit'])
            self._draw_subgroup_3d(ax, pts, color)
            count_sub += 1


        for nid, n in self.nodes.items():
            ax.scatter(n[0], n[1], n[2], c='black', marker='o', s=20, depthshade=False)

        for i, route in enumerate(self.data['routes']):
            valid_pts = [self.nodes[nid] for nid in route if nid in self.nodes]
            if not valid_pts: continue
            pts = np.array(valid_pts)
            c = self._get_route_color(i)
            ax.plot(pts[:,0], pts[:,1], pts[:,2], '-', color=c, linewidth=2, label=f'Vehicles {i}')

        sm = cm.ScalarMappable(cmap=self.cmap_heat, norm=self.norm)
        sm.set_array([])
        cbar = plt.colorbar(sm, ax=ax, fraction=0.03, pad=0.1)
        cbar.set_label('Reward', fontsize=10)

        ax.set_title(f"{len(self.data['routes'])} Vehicles", fontsize=14)
        ax.set_xlabel("X"); ax.set_ylabel("Y"); ax.set_zlabel("Z")
        if len(self.data['routes']) <= 20:
            ax.legend(loc='upper right', framealpha=0.9, fontsize=8)
        plt.tight_layout()
        plt.show()

    def _get_cluster_points(self, cluster):
        pts = []
        for sid in cluster['subgroup_ids']:
            if sid in self.subgroups:
                for nid in self.subgroups[sid]['node_ids']:
                    if nid in self.nodes:
                        pts.append(self.nodes[nid])
        return np.array(pts)

if __name__ == "__main__":
    if len(sys.argv) > 1:
        file_path = sys.argv[1]
    else:
        print("Error: Please provide the path to the JSON file as a command-line argument")
        sys.exit(1)

    if not os.path.exists(file_path):
        print(f"Error: File '{file_path}' not found")
        sys.exit(1)

    print(f"Read data from: {file_path}")

    try:
        with open(file_path, 'r') as f:
            data = json.load(f)
    except Exception as e:
        print(f"Error reading JSON: {e}")
        sys.exit(1)
        
    plot = Visualizer(data)
    
    if data["mode"] == "2d":
        plot.plot_2d()
    elif data["mode"] == "3d":
        plot.plot_3d()
    else:
        print(f"Error: Unknown mode '{data['mode']}'. Use '2d' or '3d'")
