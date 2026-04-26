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

**Step 2:** Build and open the container

```
Ctrl+Shift+P -> Dev Containers: Open folder in container
```

**Step 3:** View the [Proof of Concept iPython Notebook](./categorizer/poc/Process.ipynb)