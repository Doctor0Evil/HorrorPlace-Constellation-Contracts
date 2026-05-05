# File: tools/train_decision_tree.py
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

from sklearn.tree import DecisionTreeClassifier, export_text
import pandas as pd
import json

def train_pattern_decision_tree(training_data_path: str) -> dict:
    """
    Train decision tree to predict pattern from BCI state.
    
    training_data format (CSV):
    stressScore,visualOverloadIndex,attentionBand,stressBand,pattern_id
    0.87,0.81,locked,high,0
    0.45,0.92,focused,mid,1
    ...
    """
    df = pd.read_csv(training_data_path)
    
    # Encode categorical variables
    df['attentionBand_encoded'] = df['attentionBand'].astype('category').cat.codes
    df['stressBand_encoded'] = df['stressBand'].astype('category').cat.codes
    
    X = df[['stressScore', 'visualOverloadIndex', 'attentionBand_encoded', 'stressBand_encoded']]
    y = df['pattern_id']
    
    # Train decision tree (limit depth for interpretability)
    clf = DecisionTreeClassifier(max_depth=4, min_samples_leaf=10)
    clf.fit(X, y)
    
    # Export as text (for human review)
    tree_rules = export_text(clf, feature_names=list(X.columns))
    print(tree_rules)
    
    # Convert to JSON structure
    def tree_to_json(tree, node=0):
        if tree.feature[node] == -2:  # Leaf node
            pattern_id = int(tree.value[node].argmax())
            confidence = float(tree.value[node].max() / tree.value[node].sum())
            return {"pattern": pattern_id, "confidence": confidence}
        else:
            feature_name = X.columns[tree.feature[node]]
            threshold = float(tree.threshold[node])
            
            return {
                "condition": {
                    "feature": feature_name,
                    "threshold": threshold
                },
                "true_branch": tree_to_json(tree, tree.children_left[node]),
                "false_branch": tree_to_json(tree, tree.children_right[node])
            }
    
    tree_json = tree_to_json(clf.tree_)
    
    with open('pattern_decision_tree.json', 'w') as f:
        json.dump(tree_json, f, indent=2)
    
    return tree_json
