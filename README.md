# Deterministic Categorization

This project contains a Proof of Concept process for deterministic categorization of text messages
using a combination of regular expressions, cosine similarity of text embeddings, and LLMs.

Note that we propose a specific process for ensuring a statistically deterministic result via LLM
categorization and provide some emperical evidence for the same.

Specifically, if we ask the LLM if the text fits into a specific category--and not a set of categories
or an open question about what category it could be--then we significantly reduce the chances of a 
non-deterministic answer.  Additionally, by using well known category keywords via regular expressions
and avoiding LLM, cosine similarity, or partial regular expression matches, then we can further reduce
the false positives to the point where we can consider the final result "functionally deterministic".

**Table of Contents**

- [Quck Start](#quick-tart)
- [General Architecture](#general-architecture)
    - [Process Flow](#process-flow)
- [Implementation](#implementation)
    - [Integration and Unit Tests (Rust)](#integration-and-unit-tests-rust)
    - [Llama CPP and Tiny LLM models](#llama-cpp-and-tiny-llm-models)
    - [Aggregation and Visualization (PosgreSQL and Grafana)](#aggregation-and-visualization-posgresql-and-grafana)

## Quick Start

**Prerequsites**

- [Git](https://git-scm.com/)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Podman](https://podman.io/) or [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Dev Containers Extension](https://code.visualstudio.com/docs/devcontainers/containers)

**Step 1:** Download this repository

```bash
git clone https://gitlab.com/agramirez/deterministic-categorization.git
```

**Step 2:** Build and open the container (from VSCode execute)

```
Ctrl+Shift+P -> Dev Containers: Open folder in container
```

**Step 3:** View the [Proof of Concept iPython Notebook](./categorizer/poc/Process.ipynb)

## General Architecture

![General Architecture Diagram](./doc/diagrams/architecture/General%20Architecture.svg)

Our general architecture provides 4 key features:

1. Flexible sources and destinations
2. Parallelized processing for optimal performance
3. Progressive categorization from most deterministic to least deterministic
4. Low LLM costs from progressive categorization

### Process Flow

Our process starts by selecting the categories and messages to be processed.

We can select from multiple sources such as local files, databases, or cloud storage.

The second step is to process each message using the selected categories.  We provision the ability to create category hirarchies such that a match on a leaf hirarchy implies a match on parent hirarchies.

The third step is to match one or more categories to a message.  This step, of matching one or more categories to a message is done in parallel to optimize performance.  We use category groups to process multiple categories in parallel.  Categories that belong to the same group are processed sequentially and optionally can be stopped on the fist successful match such that more deterministic categorization methods are tested first before less deterministic methods.

The final step is to push the categorized messages to one or more destination storage locations. 

## Implementation

A Proof of Concept (PoC) implementation is written in Python using standard regular expressions, llama.cpp server, tiny LLM models, and iPython notebook.

A more standadized Minimal Viable Product (MVP) is implemented in Erlang using regular expresion, llama.cpp server, tiny LLM models, the Gherkin/Cucumber testing framework, and PostgreSQL for data storage.

An aggregation and visualization framework for aggregated metrics is provided via PosgreSQL and Grafana.

### Integration and Unit Tests (Rust)

**TBD**

### Llama CPP and Tiny LLM models

**TBD**

### Aggregation and Visualization (PosgreSQL and Grafana)

**TBD**