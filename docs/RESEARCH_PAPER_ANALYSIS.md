# Academic Research Paper Analysis

## Title
**A Page Replacement Algorithm Based on Frequency Derived From Reference History**

## Authors & Citation
* **Hong-Bin Tsai & Chin-Laung Lei**  
* Department of Electrical Engineering, National Taiwan University, Taipei, Taiwan  
* *ACM Symposium on Applied Computing (SAC '17)*, April 3–7, 2017, Marrakesh, Morocco  
* DOI: [10.1145/3019612.3019737](http://dx.doi.org/10.1145/3019612.3019737)

---

## 1. Executive Abstract
Page replacement algorithms are a foundational pillar of modern virtual memory management, deciding which victim page in main memory (DRAM) to evict when allocating space for newly referenced pages. While modern hardware advances (such as byte-addressable Non-Volatile Persistent Memory / PCM) have drastically decreased swap I/O latency, software eviction policies remain a bottleneck.

Tsai and Lei present a **log-based page replacement algorithm** that derives page reference frequencies from a sliding history window. By tracking frequency within an analytically bounded threshold window $W_{thres}$, the algorithm evicts the page with the lowest reference rate. The paper demonstrates that under realistic memory access patterns with varying "hotness", the proposed algorithm achieves up to an **11% hit ratio improvement over LRU**, **9.21% over NFU**, and **8.18% over AGING**.

---

## 2. Motivation: The Shift from Hardware to Software Bottlenecks
Traditional virtual memory architectures assumed massive latency disparities between main memory (DRAM: ~100 ns) and mechanical disks or solid-state drives (SSD: ~10 µs to 10 ms). Consequently:
1. Minimizing page replacement overhead in CPU cycles was historically favored over complex history tracking.
2. Evicting pages quickly was paramount to avoid CPU stalls.

However, the advent of **Persistent Memory (PMEM)** and **Phase Change Memory (PCM)** bridges the physical latency gap, providing near-DRAM read/write speeds with byte addressability. In tests on Linux kernels using PMEM swap disks with `stress-ng` parallel sorting (`qsort`), Tsai and Lei observed that as memory pressure and worker counts increased, the elapsed execution time gap between RAM-only and PMEM-swap widened noticeably. This confirmed that **the virtual memory management policy itself (software overhead and hit ratio quality)** is now the primary constraint, justifying more sophisticated reference history analysis.

---

## 3. Theoretical Model and Algorithmic Formulation

### 3.1 The Sliding Window History Log
The algorithm maintains:
1. **Queue-Structured Page Log ($\text{Log}$)**: A circular FIFO queue of maximum length $W$ tracking the most recent page requests.
2. **Reference Counter Table ($RC$)**: An array/hash map recording the occurrence count $RC(P_k)$ of each virtual page $P_k$ inside the window.

$$\sum_{k=1}^{N} RC(P_k) = W$$

Where $N$ is the number of distinct virtual pages, and $W$ is the window length.

When page $P_k$ is referenced:
- $P_k$ is enqueued at the head of the log.
- $RC(P_k)$ is incremented.
- If the queue exceeds capacity $W$, the tail element $P_{\text{tail}}$ is dequeued, and $RC(P_{\text{tail}})$ is decremented.

When a page fault occurs and all physical frames $F$ are occupied:
- The Reference Rate $RR(P_i)$ is calculated for every page currently resident in memory:
  $$RR(P_i) = \frac{RC(P_i)}{\text{TotalRef}}$$
- The algorithm evicts the resident frame with the **lowest reference rate** ($\min RR$).

```
Algorithm: Log-based Page Replacement
Input : Log of recent references, FrameList of memory resident pages
Output: Victim page to evict

foreach Page_i in FrameList:
    if victim is NULL or RR(Page_i) < minRR:
        minRR  <- RR(Page_i)
        victim <- Page_i
return victim
```

---

## 4. Analytical Estimation of Window Threshold ($W_{thres}$)
A central contribution of the paper is proving that an infinite history log is unnecessary. The authors derive a closed-form formula for the **threshold window size ($W_{thres}$)**—the minimum history length where performance matches an unbounded log.

### 4.1 Hotness Multiplier ($M$)
Pages do not have uniform access probabilities. The workload exhibits a hot/cold split:
- Let $h$ be the hotness fraction ($h \in [0.10, 0.45]$).
- In an $h / (1 - h)$ distribution (e.g., 20/80), $h \cdot N$ pages are "hot" and receive $(1 - h)$ of all memory references.
- The **Hotness Multiplier ($M$)** defines how many times a hot page must be referenced while each non-hot page is referenced at least once:

$$M = \left( 1 + \frac{hN}{F} \right)^2$$

Where:
- $N$ = Number of distinct virtual pages referenced.
- $F$ = Number of physical page frames allocated.

### 4.2 Threshold Window Equation ($W_{thres}$)
$$W_{thres} = \left\lceil \frac{hN}{1 - h} \right\rceil \times \lceil hN \times M \rceil$$

### 4.3 The "Warm Zone" Discovery
In workloads where $h$ approaches 50/50 (specifically **40/60** and **45/55** hotness), the boundary between hot and cold pages becomes blurred (the difference in reference rates drops to only 2%–4%). The authors define this region as the **Warm Zone**.
- To prevent eviction errors in the warm zone, the window must be scaled:
  * For **40/60**: $W_{\text{applied}} = 2 \times W_{thres}$
  * For **45/55**: $W_{\text{applied}} = 5 \times W_{thres}$
- With this compensation, the hit ratio difference between the finite threshold log and an infinite log is under **0.007%**.

---

## 5. Comparative Evaluation Summary

| Algorithm | Mechanism | Peak Advantage of Log-Based | Key Weakness |
| :--- | :--- | :--- | :--- |
| **Bélády OPTIMAL (MIN)** | Unattainable theoretical limit (evicts furthest future reference) | OPTIMAL is +10–15% higher (Upper Bound) | Requires full future knowledge; cannot be implemented in online OS |
| **Log-Based (Tsai & Lei)** | Bounded sliding window frequency with analytical $W_{thres}$ | **Baseline** | Requires maintaining a sliding queue and counters |
| **Least Recently Used (LRU)** | Recency-based; evicts page with oldest timestamp | **+11.20% improvement** | Blind to overall frequency; thrashing occurs under cyclic patterns |
| **Not Frequently Used (NFU)** | Cumulative global reference counters | **+9.21% improvement** | History pollution; early-accessed pages stay forever without decay |
| **AGING** | Shift registers updated at periodic timer ticks | **+8.18% improvement** | Coarse time granularity; truncation loss in limited register bits |
| **RANDOM** | Pseudo-random frame selection | **+20–35% improvement** | Zero locality awareness |

---

## 6. Implementation in the Dual-OS Project

Our project integrates this research paper across both software tiers:

1. **Debian MicroOS (`/usr/local/bin/research-lab`)**:
   - Production-grade C evaluation engine executing the exact algorithms, hotness generators, and window equations.
   - Provides an interactive console, ASCII bar graph comparisons, paper figure replication sweeps, and automated CSV export (`research_results.csv`).
   - Accessible via Desktop Shortcut `Research Lab (ACM SAC'17)` or shell command `research-lab`.

2. **RustOS Bare-Metal Kernel (`pagerep` shell command)**:
   - Direct bare-metal demonstration running in protected mode inside the kernel.
   - Visually renders frame allocations, sliding window updates, hit/miss events, and victim selection directly to the VGA color display.
