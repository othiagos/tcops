import os
import glob
import math
import numpy as np
from sklearn.cluster import KMeans

# Configuration
TSP_DIR = 'data/tsp'
TCOPS_DIR = 'data/tcops'
SOLUTIONS_FILE = os.path.join(TSP_DIR, 'solutions')

# Parameters
TARGET_INSTANCE = ""
OVERWRITE = False
THETA = 0.75
VEHICLES = 3
SUBGROUPS_PER_CUSTOMER = 0.1
CLUSTERS_PER_SUBGROUP = 0.2

def load_solutions(filepath):
    solutions = {}
    if not os.path.exists(filepath):
        print(f"Warning: Solutions file not found at {filepath}")
        return solutions
    with open(filepath, 'r') as f:
        for line in f:
            line = line.strip()
            if not line or ':' not in line:
                continue
            parts = line.split(':')
            name = parts[0].strip().lower()
            try:
                val_str = parts[1].strip().split()[0]
                solutions[name] = float(val_str)
            except (ValueError, IndexError):
                pass
    return solutions

def parse_tsp_file(filepath):
    nodes = []
    original_comment = "No comment"
    reading_coords = False
    with open(filepath, 'r') as f:
        for line in f:
            line = line.strip()
            if line.startswith("COMMENT"):
                parts = line.split(":", 1)
                if len(parts) > 1:
                    original_comment = parts[1].strip()
                continue
            if line.startswith("EOF"):
                break
            if line.startswith("NODE_COORD_SECTION"):
                reading_coords = True
                continue
            if reading_coords:
                parts = line.split()
                if len(parts) >= 3:
                    nodes.append((int(parts[0]), float(parts[1]), float(parts[2])))
    return nodes, original_comment

def generate_instances():
    os.makedirs(TCOPS_DIR, exist_ok=True)
    solutions = load_solutions(SOLUTIONS_FILE)
    tsp_files = glob.glob(os.path.join(TSP_DIR, '*.[tT][sS][pP]'))

    if TARGET_INSTANCE:
        tsp_files = [f for f in tsp_files if os.path.basename(f).rsplit('.', 1)[0].lower() == TARGET_INSTANCE.lower()]

    for tsp_file in tsp_files:
        base_name = os.path.basename(tsp_file).rsplit('.', 1)[0]
        search_name = base_name.lower()
        
        output_path = os.path.join(TCOPS_DIR, f"{base_name}.tcops")
        if os.path.exists(output_path) and not OVERWRITE:
            print(f"Skipping {base_name}: File already exists.")
            continue
        
        if search_name not in solutions:
            continue
            
        tsp_opt = solutions[search_name]
        nodes, original_comment = parse_tsp_file(tsp_file)
        
        if len(nodes) < 5:
            continue
            
        depot = nodes[0]
        customers = nodes[1:]
        N = len(customers)
        customer_coords = np.array([[n[1], n[2]] for n in customers])

        num_subgroups = max(2, math.ceil(N * SUBGROUPS_PER_CUSTOMER))
        num_clusters = max(1, math.ceil(num_subgroups * CLUSTERS_PER_SUBGROUP))

        kmeans_sg = KMeans(n_clusters=num_subgroups, n_init=10, random_state=42)
        sg_labels = kmeans_sg.fit_predict(customer_coords)
        sg_centroids = kmeans_sg.cluster_centers_ 

        kmeans_cl = KMeans(n_clusters=num_clusters, n_init=10, random_state=42)
        cl_labels = kmeans_cl.fit_predict(sg_centroids)

        t_max_total = THETA * tsp_opt
        budget_per_vehicle = round(t_max_total / VEHICLES, 2)
        
        lines = []
        lines.append(f"NAME: {base_name}")
        lines.append("TYPE: TCOPS")
        lines.append(f"COMMENT: Generated from {base_name}.tsp")
        lines.append(f"DIMENSION: {len(nodes)}")
        lines.append(f"SUBGROUPS: {num_subgroups + 1}")
        lines.append(f"CLUSTERS: {num_clusters + 1}")
        lines.append(f"VEHICLES: {VEHICLES}")
        lines.append("EDGE_WEIGHT_TYPE: EUC_2D")
        
        lines.append("NODE_COORD_SECTION: id profit x y")
        lines.append(f"0 0.0 {depot[1]:.2f} {depot[2]:.2f}")
        
        subgroup_nodes = {i: [] for i in range(num_subgroups)}
        for idx, (orig_id, x, y) in enumerate(customers):
            new_id = idx + 1
            # Deterministic pseudo-random profit generation
            p_j = 1.0 + ((7141 * new_id + 73) % 100)
            lines.append(f"{new_id} {p_j:.1f} {x:.2f} {y:.2f}")
            sg_id = sg_labels[idx]
            subgroup_nodes[sg_id].append(new_id)
            
        lines.append("SUBGROUP_SECTION: subgroup_id id-vertex-list")
        lines.append("0 0")
        for sg_id in range(num_subgroups):
            nodes_str = " ".join(map(str, subgroup_nodes[sg_id]))
            lines.append(f"{sg_id + 1} {nodes_str}")
            
        lines.append("CLUSTER_SECTION: cluster_id id-subgroup-list")
        lines.append("0 0")
        cluster_sgs = {i: [] for i in range(num_clusters)}
        for sg_index, c_label in enumerate(cl_labels):
            cluster_sgs[c_label].append(sg_index + 1)
            
        for c_id in range(num_clusters):
            sgs_str = " ".join(map(str, cluster_sgs[c_id]))
            lines.append(f"{c_id + 1} {sgs_str}")
            
        lines.append("VEHICLES_SECTION: id tmax start_node_id end_node_id")
        for v_id in range(VEHICLES):
            lines.append(f"{v_id} {budget_per_vehicle:.1f} 0 0")
            
        output_path = os.path.join(TCOPS_DIR, f"{base_name}.tcops")
        with open(output_path, 'w') as f:
            f.write("\n".join(lines) + "\n")
            
        print(f"[OK] {base_name}.tcops -> {num_subgroups} subgroups, {num_clusters} clusters.")

if __name__ == "__main__":
    generate_instances()