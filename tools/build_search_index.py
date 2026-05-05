#!/usr/bin/env python3
"""Build search index for markdown docs."""

import json, re, math
from pathlib import Path
from datetime import datetime
from collections import defaultdict

def chunk_by_section(lines: list) -> list:
    """Split doc into paragraph chunks."""
    chunks = []
    current_section = ""
    current_chunk = []
    start_line = 0
    
    for i, line in enumerate(lines, 1):
        if line.startswith('##'):
            if current_chunk:
                chunks.append({
                    'section': current_section,
                    'start_line': start_line,
                    'end_line': i - 1,
                    'content': ' '.join(current_chunk)
                })
            current_section = line.strip()
            current_chunk = []
            start_line = i
        elif line.strip():
            current_chunk.append(line.strip())
    
    return chunks

def build_tfidf(documents: list) -> dict:
    """Build TF-IDF index."""
    tf = defaultdict(lambda: defaultdict(int))
    df = defaultdict(int)
    
    for doc_id, doc in enumerate(documents):
        terms = set()
        for chunk in doc['chunks']:
            for term in extract_terms(chunk['content']):
                tf[term][f"doc:{doc_id}"] += 1
                terms.add(term)
        for term in terms:
            df[term] += 1
    
    tfidf = defaultdict(dict)
    num_docs = len(documents)
    
    for term, doc_freqs in tf.items():
        idf = math.log(num_docs / df[term])
        for doc_id, freq in doc_freqs.items():
            tfidf[term][doc_id] = round((1 + math.log(freq)) * idf, 2)
    
    return dict(tfidf)

def extract_terms(text: str) -> list:
    """Extract indexable terms."""
    text = re.sub(r'[^a-z0-9\s-]', ' ', text.lower())
    stopwords = {'the', 'a', 'an', 'and', 'or', 'in', 'on', 'at', 'to', 'for'}
    return [t for t in text.split() if t not in stopwords and len(t) > 2]

def main():
    docs_path = Path("docs")
    md_files = sorted(docs_path.rglob("*.md"))
    
    documents = []
    for i, md_path in enumerate(md_files):
        content = md_path.read_text()
        lines = content.split('\n')
        
        documents.append({
            'id': f"doc:{i}",
            'path': str(md_path),
            'title': lines[0].lstrip('# ') if lines else md_path.stem,
            'chunks': chunk_by_section(lines)
        })
    
    tfidf_index = build_tfidf(documents)
    
    print(json.dumps({
        "version": "1.0.0",
        "generated_at": datetime.now().isoformat(),
        "documents": documents,
        "tfidf_index": tfidf_index
    }, indent=2))

if __name__ == '__main__':
    main()
