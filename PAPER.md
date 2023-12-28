# Sophon: Data oriented composition of AI agents and oracles

## Introduction
In this paper I'll discuss the implementation of this system in its currrent state, propose better design patterns for facilitating communication between bound agents,
and detail how this library can be extended to include those changes.  Agents in this library were designed to be agnostic over backends,
meaning the interface doesn't care if it's communicating with a large language model, a human user, or some other medium for exchanging structured data,
such as a search engine.  This provides dependents of this library with a wider degree of flexibility when designing specialized agent based systems,
since they can treat any data source as a valid agent within their system.  We'll discuss how this can impact message passing within networked and local
agent environments in a later section.

## Implementation


## Better design propositions


## Options for extending this library


## Additional Notes

### Issues
- cyclic dependencies
  - prior to execution of the agent runtime, we can compile the user's configuration into an intermediate syntax tree
    and walk this tree in search of cyclic dependencies, backtracking when none are found for a given agent, 
    until we've ensured all agents are clean
  - I have exeperience writing compilers, so this is a feasible solution
- latency between single connections
  - we can provide optional configuration for multiple backend nodes driving a single agent.
    With this functionallity we can acquire real time benchmarking and select less latent nodes for that agent on a per-relationship basis,
    improving performance the longer the program runs and potentially allowing this system to modify the configuration 
    to reflect these improved node choices, making subsequent executions faster.
  - we can extend the agent-backend interface to allow a single agent to run parallel queries accros multiple backends.  This will be challenging
    in that we'll need functionallity to run multiple backends multiple times for each agent relationship.  We can use a linear interpolation over 
    an averaged event-per-backend delta, pruning slower backends as we go, until all but one are pruned at a delta 1
  - This might slow the system down more than it helps in short lived agents, but will increase the average performance
    of transactions in large systems with long lifetimes.
     
### Options
- State machine
- Entity Component System
- Hierarchical entity system (boo)

