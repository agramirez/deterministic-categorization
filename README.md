# Deterministic Categorization

This project contains a Proof of Concept process for deterministic categorization of text messages
using a combination of regular expressions, cosine similarity of text embeddings, and LLMs.

Note that we propose a specific process for ensuring a statistically deterministic result via LLM
categorization and provide some emperical evidence for the same (**This is still not ready**).

Specifically, if we ask the LLM if the text fits into a specific category--and not a set of categories
or an open question about what category it could be--then we significantly reduce the chances of a 
non-deterministic answer.  Additionally, by using well known category keywords via regular expressions
and avoiding LLM, cosine similarity, or partial regular expression matches, then we can further reduce
the false positives to the point where we can consider the final result "functionally deterministic".

**Table of Contents**

- [Quck Start](#quick-tart)
- [General Architecture](#general-architecture)
    - [Benefits](#benefits)
        - [Flexible sources and destinations](#flexible-sources-and-destinations)
        - [Parallelized processing for optimal performance](#parallelized-processing-for-optimal-performance)
        - [Progressive categorization to ensure determinism](#progressive-categorization-to-ensure-determinism)
        - [Confidence level associated with each category match](#confidence-level-associated-with-each-category-match)
    - [Process Flow](#process-flow)
- [Implementation](#implementation)
    - [Integration and Unit Tests](#integration-and-unit-tests)
        - [Erlang](#erlang)
        - [Rust](#rust)
        - [Python](#python)
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
3. Progressive categorization to ensure determinism
4. Optimal resource usage
5. Confidence level associated with each category match

### Benefits

#### Flexible sources and destinations

A key idea in the architecture is to allow flexibility in data sources and destinations.  This follows standard practices in Dependency Injection for both testability and flexibility in taking the design from Proof of Concept (PoC) to Minimum Viable Product (MVP).  For example, in the PoC, and even in the MVP phase, we can use CSV files along with a spreadsheet editor as our UI to generate the regular expression categories.  Once we have our basic design and process in place we can then switch to a more standard database approach with some Admin user interface for editing.

#### Parallelized processing for optimal performance

The architecture follows an event driven microservices framework such that we can ensure optimal performance for categorization.  This might not be necessary in an overnight batch process, but it becomes more critical in realtime event driven systems where we might want to categorize messages as they arrive in the system.

#### Progressive categorization to ensure determinism

A key aspect of this framework is the idea of ensuring determinism by matching first to strict deterministic categories and falling back to less deterministic methods should the strict categories fail.

For example, we begin by searching for specific keywords associated with a category.  These keyword categories must be manually created, but it is easy to do so and the framework will readily accept the categories whether they are created in a CSV file, a database, etc.

If these initial keyword categories are not matched, then we can fall back to less deterministic versions.  For example, we can try to match a keyword category to mispelled versions of the same.  If someone sends a message requesting information about **corn**, but mispells the word as **cron**, we can create a regular expression to match the mispelling.  In that case it is not 100% certain that they are talking about corn though, as they could be asking a question about a Linux **cron** job.  But, that is when we can leverage LLMs to validate the message and determine if the message was asking about **corn** or **cron** jobs.  A key feature of this process is that we have already filtered out messages completely unrelated to corn, or cron jobs for that matter.  Thus we efficiently only request an LLM check when absolutely necessary.

Further, because we are sure about the categories that we are searching for, the LLM is limitted to a Yes/No answer about a specific category, and is not given an open ended request to find "all potential categories" or "some possible categories", thus reducing the chances of receiving a non-deterministic response.

> NOTE: It might be more cost effective to call an LLM API once to determine all possible categories, but, it will also be more prone to false positives and non-deterministic answers.  This tradoff must be considered.  However, given that many, or most, messages will be deterministically categorized by simple keyword matching, I would argue that in practice this process can be MORE cost effective if the overall number of non-keyword matched message plus the total number of categories is lower than the total number of messages categorized via an LLM single call.

#### Confidence level associated with each category match

Since determinism is not necessarily guaranteed, although it is expected to be functionality present, we include a confidence level for each category match, such that we can account for potential deviations from full determinism.

In the case of a full keyword match we can say that our confidence is 100% (1.0).  In the case that we match with a mispelled word, we can provide some statistically generated confidence about how likely the match is a true positive vs a false positive.  For example, we could run the mispelled word regular expression for a category against a list of curated messages from our database and determine how many false positives we get.  We then associated that value to the regular expression category match such that when a new message arrives and is categorized by said regular expression we know the likelihood that it is correct.  This value can then be used in reports and aggregated statistics as necessary.

### Drawbacks

**TBD**

### Process Flow

Our process starts by selecting the categories and messages to be processed.

We can select from multiple sources such as local files, databases, or cloud storage.

The second step is to process each message using the selected categories.  We provision the ability to create category hirarchies such that a match on a leaf hirarchy implies a match on parent hirarchies.

Example:
```
             => Pest => English => The Wriggler (wrigler)
             => Pest => English => The Wriggler Mispelled (wr?i?gle?r?)
             => Pest => English => Fall Army worm (FAW|Fall Army Work)
Crop => Corn => English => (Corn/Maize)
             => English => Mispelled (cron|con|ron|crn)
             => Spanish => (Maíz)
Language => English => USA
            Spanish => Mexico (neta|wey)
                    => Spain (vale)
                    => Colombia (chimba|vacano)
         => Esperanto
         => Kannada
```

In the above list we can see that a regular expression--defined in parenthesis ()--can be used to match a specific item.  However, a hirarchy exists such that we can infer that if a user sends a message asking about a corn pest such as The Wriggler (this is made up by the way...) then we can infer that the message is written in English, is about a Crop, is about the Corn Crop (assuming that The Wriggler only infects Corn crops).

> NOTE: The above process will require some additional logic and processing that is not currently implement as of 2026-04-27...but in theory it should be implementable and thus allow a reduction in checks and validations if our hirarchy and filters work correctly.

The third step is to match one or more categories to a message.  This step, of matching one or more categories to a message is done in parallel to optimize performance.  We use category groups to process multiple categories in parallel.  Categories that belong to the same group are processed sequentially and optionally can be stopped on the fist successful match such that more deterministic categorization methods are tested first before less deterministic methods.

The final step is to push the categorized messages to one or more destination storage locations. 

## Implementation

A Proof of Concept (PoC) implementation is written in Python using standard regular expressions, llama.cpp server, tiny LLM models, and iPython notebook. **WORK IN PROGRESS, but enough for demo purposes**

A more standadized Minimal Viable Product (MVP) is implemented in Erlang using regular expresion, llama.cpp server, tiny LLM models, the Gherkin/Cucumber testing framework, and PostgreSQL for data storage. **WORK IN PROGRESS**

An aggregation and visualization framework for aggregated metrics is provided via PosgreSQL and Grafana. **WORK IN PROGRESS**

### Integration and Unit Tests

#### Erlang

**TBD**

#### Rust

**TBD**

#### Python

**TBD**


### Llama CPP and Tiny LLM models

**TBD**

### Aggregation and Visualization (PosgreSQL and Grafana)

In this case I have chosen Grafana as the visualization technology for the following reasons:

1. It is easily integrated into a development environment
2. It provies support for visualizing every part of the system (from infrastructure to reports and aggregated metrics)
3. It looks nice

Example:
![Grafana dashboard example](./doc/images/dashboard-example.png)

PosgreSQL will be used as the database backend because it can be easily integrated into this demo and because it supports many awesome features out of the box (ie it has AI/LLM integrations which would be used to generate embeddings directly from data in the database).

However, both of these technologies can be substituted for others such as Google BigQuery, Google Analytics UI, custom dashboards, AWS Quicksight, etc.