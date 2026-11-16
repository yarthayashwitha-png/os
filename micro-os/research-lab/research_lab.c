/* ==============================================================================
 * AETHER-OS RESEARCH LAB: PAGE REPLACEMENT EVALUATION ENGINE
 * Paper: "A Page Replacement Algorithm Based on Frequency Derived From Reference History"
 * Authors: Hong-Bin Tsai & Chin-Laung Lei (National Taiwan University, ACM SAC'17)
 * ============================================================================== */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <time.h>
#include <stdint.h>
#include <stdbool.h>

#define MAX_FRAMES 1024
#define MAX_PAGES  4096
#define MAX_TRACE  1000000
#define DEFAULT_TRACE_LEN 163840
#define DEFAULT_FRAMES 10
#define DEFAULT_VIRTUAL_PAGES 20

/* ANSI Color Formatting */
#define COLOR_RESET   "\033[0m"
#define COLOR_BOLD    "\033[1m"
#define COLOR_CYAN    "\033[36m"
#define COLOR_GREEN   "\033[32m"
#define COLOR_YELLOW  "\033[33m"
#define COLOR_RED     "\033[31m"
#define COLOR_MAGENTA "\033[35m"
#define COLOR_BLUE    "\033[34m"

typedef enum {
    ALG_OPTIMAL = 0,
    ALG_LOG_BASED,
    ALG_LRU,
    ALG_NFU,
    ALG_AGING,
    ALG_RANDOM,
    ALG_COUNT
} AlgorithmType;

const char *ALG_NAMES[ALG_COUNT] = {
    "OPTIMAL (Bélády MIN)",
    "Log-Based (Tsai & Lei)",
    "Least Recently Used (LRU)",
    "Not Frequently Used (NFU)",
    "AGING (Shift Registers)",
    "RANDOM Replacement"
};

typedef struct {
    int page_id;
    int last_used;
    uint32_t total_refs;
    uint8_t age_register;
} Frame;

typedef struct {
    int *data;
    int head;
    int tail;
    int size;
    int capacity;
} LogQueue;

static LogQueue* create_log_queue(int capacity) {
    LogQueue *q = (LogQueue*)malloc(sizeof(LogQueue));
    q->capacity = capacity;
    q->data = (int*)malloc(sizeof(int) * capacity);
    q->head = 0;
    q->tail = 0;
    q->size = 0;
    return q;
}

static void free_log_queue(LogQueue *q) {
    if (q) {
        if (q->data) free(q->data);
        free(q);
    }
}

static void enqueue_log(LogQueue *q, int page) {
    q->data[q->head] = page;
    q->head = (q->head + 1) % q->capacity;
    if (q->size < q->capacity) {
        q->size++;
    } else {
        q->tail = (q->tail + 1) % q->capacity;
    }
}

static int dequeue_log_tail(LogQueue *q) {
    if (q->size == 0) return -1;
    int evicted = q->data[q->tail];
    q->tail = (q->tail + 1) % q->capacity;
    q->size--;
    return evicted;
}

/* ==============================================================================
 * Hotness Parameters & Theoretical Threshold Calculation
 * Equations from Tsai & Lei (ACM SAC'17):
 * M = (1 + (h*N)/F)^2
 * W_thres = ceil((h*N)/(1-h)) * ceil(h*N * M)
 * ============================================================================== */
typedef struct {
    double hot_page_ratio;   /* e.g., 0.40 for 40/60 */
    double hot_access_ratio; /* e.g., 0.60 for 40/60 */
    const char *name;
} HotnessConfig;

HotnessConfig HOTNESS_PROFILES[] = {
    {0.00, 0.00, "Uniform / 0"},
    {0.10, 0.90, "10/90 Hotness"},
    {0.15, 0.85, "15/85 Hotness"},
    {0.20, 0.80, "20/80 Hotness"},
    {0.25, 0.75, "25/75 Hotness"},
    {0.30, 0.70, "30/70 Hotness"},
    {0.35, 0.65, "35/65 Hotness"},
    {0.40, 0.60, "40/60 Hotness (Warm Zone)"},
    {0.45, 0.55, "45/55 Hotness (Warm Zone)"}
};
#define NUM_HOTNESS_PROFILES 9

double calculate_hotness_multiplier(double h, int N, int F) {
    double term = 1.0 + (h * (double)N) / (double)F;
    return term * term;
}

int calculate_threshold_window(double h, int N, int F) {
    if (h <= 0.001 || h >= 0.999) return N * 4;
    double M = calculate_hotness_multiplier(h, N, F);
    double part1 = ceil((h * (double)N) / (1.0 - h));
    double part2 = ceil(h * (double)N * M);
    int w_thres = (int)(part1 * part2);
    return (w_thres > 0) ? w_thres : 10;
}

/* Trace Generation Engine */
void generate_trace(int *trace, int trace_len, int N, HotnessConfig hotness) {
    int hot_pages_count = (int)(hotness.hot_page_ratio * N);
    if (hot_pages_count < 1 && hotness.hot_page_ratio > 0.0) hot_pages_count = 1;

    for (int i = 0; i < trace_len; i++) {
        if (hotness.hot_page_ratio <= 0.001) {
            /* Uniform distribution */
            trace[i] = rand() % N;
        } else {
            double roll = (double)rand() / (double)RAND_MAX;
            if (roll < hotness.hot_access_ratio) {
                /* Reference to a hot page */
                trace[i] = rand() % hot_pages_count;
            } else {
                /* Reference to a cold page */
                int cold_count = N - hot_pages_count;
                trace[i] = hot_pages_count + (rand() % (cold_count > 0 ? cold_count : 1));
            }
        }
    }
}

/* ==============================================================================
 * SIMULATION ENGINE
 * ============================================================================== */
typedef struct {
    int page_faults;
    int hits;
    double hit_ratio;
    double execution_time_ms;
} SimulationResult;

/* 1. Bélády's OPTIMAL Algorithm */
SimulationResult simulate_optimal(const int *trace, int trace_len, int num_frames) {
    clock_t start = clock();
    int frames[MAX_FRAMES];
    int frame_count = 0;
    int faults = 0;
    int hits = 0;

    for (int i = 0; i < trace_len; i++) {
        int page = trace[i];
        bool found = false;
        for (int f = 0; f < frame_count; f++) {
            if (frames[f] == page) {
                found = true;
                hits++;
                break;
            }
        }

        if (!found) {
            faults++;
            if (frame_count < num_frames) {
                frames[frame_count++] = page;
            } else {
                /* Find victim page with furthest future reference */
                int victim_idx = 0;
                int furthest_dist = -1;

                for (int f = 0; f < num_frames; f++) {
                    int p = frames[f];
                    int dist = 99999999;
                    for (int j = i + 1; j < trace_len; j++) {
                        if (trace[j] == p) {
                            dist = j;
                            break;
                        }
                    }
                    if (dist > furthest_dist) {
                        furthest_dist = dist;
                        victim_idx = f;
                    }
                }
                frames[victim_idx] = page;
            }
        }
    }

    clock_t end = clock();
    SimulationResult res;
    res.page_faults = faults;
    res.hits = hits;
    res.hit_ratio = (double)hits / (double)trace_len;
    res.execution_time_ms = ((double)(end - start) / CLOCKS_PER_SEC) * 1000.0;
    return res;
}

/* 2. Tsai & Lei's Log-Based Page Replacement Algorithm */
SimulationResult simulate_log_based(const int *trace, int trace_len, int num_frames, int N, int window_size) {
    clock_t start = clock();
    int frames[MAX_FRAMES];
    int frame_count = 0;
    int faults = 0;
    int hits = 0;

    /* Frequency reference counters inside window */
    int *ref_counter = (int*)calloc(N, sizeof(int));
    LogQueue *log_q = create_log_queue(window_size);

    for (int i = 0; i < trace_len; i++) {
        int page = trace[i];

        /* Update Log Queue and Reference Counters */
        if (log_q->size == log_q->capacity) {
            int old_page = dequeue_log_tail(log_q);
            if (old_page >= 0 && old_page < N) {
                if (ref_counter[old_page] > 0) ref_counter[old_page]--;
            }
        }
        enqueue_log(log_q, page);
        if (page >= 0 && page < N) ref_counter[page]++;

        /* Check memory frame cache */
        bool found = false;
        for (int f = 0; f < frame_count; f++) {
            if (frames[f] == page) {
                found = true;
                hits++;
                break;
            }
        }

        if (!found) {
            faults++;
            if (frame_count < num_frames) {
                frames[frame_count++] = page;
            } else {
                /* Algorithm 1: Select victim with lowest Reference Rate (lowest RC in log) */
                int victim_frame = 0;
                int min_ref = 999999999;

                for (int f = 0; f < num_frames; f++) {
                    int p = frames[f];
                    int count = (p >= 0 && p < N) ? ref_counter[p] : 0;
                    if (count < min_ref) {
                        min_ref = count;
                        victim_frame = f;
                    }
                }
                frames[victim_frame] = page;
            }
        }
    }

    free_log_queue(log_q);
    free(ref_counter);

    clock_t end = clock();
    SimulationResult res;
    res.page_faults = faults;
    res.hits = hits;
    res.hit_ratio = (double)hits / (double)trace_len;
    res.execution_time_ms = ((double)(end - start) / CLOCKS_PER_SEC) * 1000.0;
    return res;
}

/* 3. Least Recently Used (LRU) */
SimulationResult simulate_lru(const int *trace, int trace_len, int num_frames) {
    clock_t start = clock();
    int frames[MAX_FRAMES];
    int last_used[MAX_FRAMES];
    int frame_count = 0;
    int faults = 0;
    int hits = 0;

    for (int i = 0; i < trace_len; i++) {
        int page = trace[i];
        bool found = false;

        for (int f = 0; f < frame_count; f++) {
            if (frames[f] == page) {
                found = true;
                hits++;
                last_used[f] = i;
                break;
            }
        }

        if (!found) {
            faults++;
            if (frame_count < num_frames) {
                frames[frame_count] = page;
                last_used[frame_count] = i;
                frame_count++;
            } else {
                int victim = 0;
                int oldest = last_used[0];
                for (int f = 1; f < num_frames; f++) {
                    if (last_used[f] < oldest) {
                        oldest = last_used[f];
                        victim = f;
                    }
                }
                frames[victim] = page;
                last_used[victim] = i;
            }
        }
    }

    clock_t end = clock();
    SimulationResult res;
    res.page_faults = faults;
    res.hits = hits;
    res.hit_ratio = (double)hits / (double)trace_len;
    res.execution_time_ms = ((double)(end - start) / CLOCKS_PER_SEC) * 1000.0;
    return res;
}

/* 4. Not Frequently Used (NFU) */
SimulationResult simulate_nfu(const int *trace, int trace_len, int num_frames, int N) {
    clock_t start = clock();
    int frames[MAX_FRAMES];
    int frame_count = 0;
    int faults = 0;
    int hits = 0;
    int *nfu_counter = (int*)calloc(N, sizeof(int));

    for (int i = 0; i < trace_len; i++) {
        int page = trace[i];
        if (page < N) nfu_counter[page]++;

        bool found = false;
        for (int f = 0; f < frame_count; f++) {
            if (frames[f] == page) {
                found = true;
                hits++;
                break;
            }
        }

        if (!found) {
            faults++;
            if (frame_count < num_frames) {
                frames[frame_count++] = page;
            } else {
                int victim = 0;
                int min_count = 999999999;
                for (int f = 0; f < num_frames; f++) {
                    int p = frames[f];
                    if (nfu_counter[p] < min_count) {
                        min_count = nfu_counter[p];
                        victim = f;
                    }
                }
                frames[victim] = page;
            }
        }
    }

    free(nfu_counter);
    clock_t end = clock();
    SimulationResult res;
    res.page_faults = faults;
    res.hits = hits;
    res.hit_ratio = (double)hits / (double)trace_len;
    res.execution_time_ms = ((double)(end - start) / CLOCKS_PER_SEC) * 1000.0;
    return res;
}

/* 5. AGING Algorithm (Bit-shift registers) */
SimulationResult simulate_aging(const int *trace, int trace_len, int num_frames) {
    clock_t start = clock();
    int frames[MAX_FRAMES];
    uint8_t age[MAX_FRAMES] = {0};
    bool referenced[MAX_FRAMES] = {false};
    int frame_count = 0;
    int faults = 0;
    int hits = 0;
    const int clock_tick_interval = 16;

    for (int i = 0; i < trace_len; i++) {
        int page = trace[i];

        /* Periodic clock tick: shift right, set MSB if referenced */
        if (i % clock_tick_interval == 0 && i > 0) {
            for (int f = 0; f < frame_count; f++) {
                age[f] >>= 1;
                if (referenced[f]) {
                    age[f] |= 0x80;
                    referenced[f] = false;
                }
            }
        }

        bool found = false;
        for (int f = 0; f < frame_count; f++) {
            if (frames[f] == page) {
                found = true;
                hits++;
                referenced[f] = true;
                break;
            }
        }

        if (!found) {
            faults++;
            if (frame_count < num_frames) {
                frames[frame_count] = page;
                age[frame_count] = 0x80;
                referenced[frame_count] = true;
                frame_count++;
            } else {
                int victim = 0;
                uint8_t min_age = 0xFF;
                for (int f = 0; f < num_frames; f++) {
                    if (age[f] < min_age) {
                        min_age = age[f];
                        victim = f;
                    }
                }
                frames[victim] = page;
                age[victim] = 0x80;
                referenced[victim] = true;
            }
        }
    }

    clock_t end = clock();
    SimulationResult res;
    res.page_faults = faults;
    res.hits = hits;
    res.hit_ratio = (double)hits / (double)trace_len;
    res.execution_time_ms = ((double)(end - start) / CLOCKS_PER_SEC) * 1000.0;
    return res;
}

/* 6. RANDOM Algorithm */
SimulationResult simulate_random(const int *trace, int trace_len, int num_frames) {
    clock_t start = clock();
    int frames[MAX_FRAMES];
    int frame_count = 0;
    int faults = 0;
    int hits = 0;

    for (int i = 0; i < trace_len; i++) {
        int page = trace[i];
        bool found = false;
        for (int f = 0; f < frame_count; f++) {
            if (frames[f] == page) {
                found = true;
                hits++;
                break;
            }
        }

        if (!found) {
            faults++;
            if (frame_count < num_frames) {
                frames[frame_count++] = page;
            } else {
                int victim = rand() % num_frames;
                frames[victim] = page;
            }
        }
    }

    clock_t end = clock();
    SimulationResult res;
    res.page_faults = faults;
    res.hits = hits;
    res.hit_ratio = (double)hits / (double)trace_len;
    res.execution_time_ms = ((double)(end - start) / CLOCKS_PER_SEC) * 1000.0;
    return res;
}

/* ==============================================================================
 * CLI DISPLAY AND BENCHMARK ROUTINES
 * ============================================================================== */
void print_banner() {
    printf("%s", COLOR_CYAN);
    printf("===============================================================================\n");
    printf("   AETHER-OS RESEARCH LAB: REPRODUCIBLE EXPERIMENTAL SUITE                     \n");
    printf("   Evaluation of Log-Based Page Replacement (Tsai & Lei, NTU, ACM SAC'17)     \n");
    printf("===============================================================================\n%s", COLOR_RESET);
}

void print_bar(double ratio, const char *color) {
    int bar_width = 35;
    int filled = (int)(ratio * bar_width);
    printf("%s[", color);
    for (int i = 0; i < bar_width; i++) {
        if (i < filled) printf("#");
        else printf(" ");
    }
    printf("] %6.2f%%%s\n", ratio * 100.0, COLOR_RESET);
}

void run_comparison_experiment(int trace_len, int num_frames, int N, int hotness_idx) {
    HotnessConfig hot = HOTNESS_PROFILES[hotness_idx];
    int *trace = (int*)malloc(sizeof(int) * trace_len);
    generate_trace(trace, trace_len, N, hot);

    int w_thres = calculate_threshold_window(hot.hot_page_ratio, N, num_frames);
    /* For warm zone (40/60 and 45/55), multiply by 2 or 5 per Section 4.2 */
    int eval_window = w_thres;
    if (hotness_idx == 7) eval_window = 2 * w_thres;
    if (hotness_idx == 8) eval_window = 5 * w_thres;
    if (eval_window < 20) eval_window = 20;

    printf("\n%s[+] RUNNING EXPERIMENT: %s (Trace Length: %d, Frames: %d, Pages: %d)%s\n",
           COLOR_BOLD, hot.name, trace_len, num_frames, N, COLOR_RESET);
    printf("    Theoretical Multiplier (M): %.2f | Predicted W_thres: %d | Applied Window: %d\n\n",
           calculate_hotness_multiplier(hot.hot_page_ratio, N, num_frames), w_thres, eval_window);

    SimulationResult res_opt = simulate_optimal(trace, trace_len, num_frames);
    SimulationResult res_log = simulate_log_based(trace, trace_len, num_frames, N, eval_window);
    SimulationResult res_lru = simulate_lru(trace, trace_len, num_frames);
    SimulationResult res_nfu = simulate_nfu(trace, trace_len, num_frames, N);
    SimulationResult res_age = simulate_aging(trace, trace_len, num_frames);
    SimulationResult res_rnd = simulate_random(trace, trace_len, num_frames);

    printf("  %-26s : ", ALG_NAMES[ALG_OPTIMAL]);
    print_bar(res_opt.hit_ratio, COLOR_GREEN);
    printf("  %-26s : ", ALG_NAMES[ALG_LOG_BASED]);
    print_bar(res_log.hit_ratio, COLOR_CYAN);
    printf("  %-26s : ", ALG_NAMES[ALG_LRU]);
    print_bar(res_lru.hit_ratio, COLOR_YELLOW);
    printf("  %-26s : ", ALG_NAMES[ALG_NFU]);
    print_bar(res_nfu.hit_ratio, COLOR_MAGENTA);
    printf("  %-26s : ", ALG_NAMES[ALG_AGING]);
    print_bar(res_age.hit_ratio, COLOR_BLUE);
    printf("  %-26s : ", ALG_NAMES[ALG_RANDOM]);
    print_bar(res_rnd.hit_ratio, COLOR_RED);

    double delta_lru = (res_log.hit_ratio - res_lru.hit_ratio) * 100.0;
    double delta_nfu = (res_log.hit_ratio - res_nfu.hit_ratio) * 100.0;
    double delta_age = (res_log.hit_ratio - res_age.hit_ratio) * 100.0;

    printf("\n%s  => SUMMARY VS CONVENTIONAL ALGORITHMS:%s\n", COLOR_BOLD, COLOR_RESET);
    printf("     Improvement over LRU   : %s%+.2f%%%s\n", (delta_lru >= 0 ? COLOR_GREEN : COLOR_RED), delta_lru, COLOR_RESET);
    printf("     Improvement over NFU   : %s%+.2f%%%s\n", (delta_nfu >= 0 ? COLOR_GREEN : COLOR_RED), delta_nfu, COLOR_RESET);
    printf("     Improvement over AGING : %s%+.2f%%%s\n", (delta_age >= 0 ? COLOR_GREEN : COLOR_RED), delta_age, COLOR_RESET);

    free(trace);
}

void run_paper_sweep() {
    printf("\n%s>>> EXECUTING FULL HOTNESS SWEEP (Replicating Paper Figure 4) <<<%s\n\n", COLOR_CYAN, COLOR_RESET);
    printf("%-24s | %-10s | %-10s | %-10s | %-10s | %-10s\n",
           "Hotness Profile", "OPTIMAL", "Log-Based", "LRU", "NFU", "Delta(Log-LRU)");
    printf("-------------------------+------------+------------+------------+------------+------------\n");

    const int trace_len = 81920;
    const int num_frames = 10;
    const int N = 20;

    for (int h = 0; h < NUM_HOTNESS_PROFILES; h++) {
        HotnessConfig hot = HOTNESS_PROFILES[h];
        int *trace = (int*)malloc(sizeof(int) * trace_len);
        generate_trace(trace, trace_len, N, hot);

        int w_thres = calculate_threshold_window(hot.hot_page_ratio, N, num_frames);
        int eval_win = w_thres;
        if (h == 7) eval_win = 2 * w_thres;
        if (h == 8) eval_win = 5 * w_thres;
        if (eval_win < 20) eval_win = 20;

        SimulationResult r_opt = simulate_optimal(trace, trace_len, num_frames);
        SimulationResult r_log = simulate_log_based(trace, trace_len, num_frames, N, eval_win);
        SimulationResult r_lru = simulate_lru(trace, trace_len, num_frames);
        SimulationResult r_nfu = simulate_nfu(trace, trace_len, num_frames, N);

        double delta = (r_log.hit_ratio - r_lru.hit_ratio) * 100.0;
        printf("%-24s | %9.2f%% | %9.2f%% | %9.2f%% | %9.2f%% | %s%+9.2f%%%s\n",
               hot.name, r_opt.hit_ratio * 100.0, r_log.hit_ratio * 100.0,
               r_lru.hit_ratio * 100.0, r_nfu.hit_ratio * 100.0,
               (delta >= 0 ? COLOR_GREEN : COLOR_RED), delta, COLOR_RESET);

        free(trace);
    }
}

void export_csv_benchmark(const char *filename) {
    FILE *fp = fopen(filename, "w");
    if (!fp) {
        printf("%s[-] Error opening file %s for writing.%s\n", COLOR_RED, filename, COLOR_RESET);
        return;
    }

    fprintf(fp, "Hotness,Optimal,LogBased,LRU,NFU,Aging,Random\n");
    const int trace_len = 40960;
    const int num_frames = 10;
    const int N = 20;

    for (int h = 0; h < NUM_HOTNESS_PROFILES; h++) {
        HotnessConfig hot = HOTNESS_PROFILES[h];
        int *trace = (int*)malloc(sizeof(int) * trace_len);
        generate_trace(trace, trace_len, N, hot);

        int w_thres = calculate_threshold_window(hot.hot_page_ratio, N, num_frames);
        int eval_win = (h >= 7) ? 2 * w_thres : w_thres;
        if (eval_win < 20) eval_win = 20;

        SimulationResult r_opt = simulate_optimal(trace, trace_len, num_frames);
        SimulationResult r_log = simulate_log_based(trace, trace_len, num_frames, N, eval_win);
        SimulationResult r_lru = simulate_lru(trace, trace_len, num_frames);
        SimulationResult r_nfu = simulate_nfu(trace, trace_len, num_frames, N);
        SimulationResult r_age = simulate_aging(trace, trace_len, num_frames);
        SimulationResult r_rnd = simulate_random(trace, trace_len, num_frames);

        fprintf(fp, "\"%s\",%.4f,%.4f,%.4f,%.4f,%.4f,%.4f\n",
                hot.name, r_opt.hit_ratio, r_log.hit_ratio, r_lru.hit_ratio,
                r_nfu.hit_ratio, r_age.hit_ratio, r_rnd.hit_ratio);

        free(trace);
    }

    fclose(fp);
    printf("%s[+] Successfully exported benchmark dataset to: %s%s\n", COLOR_GREEN, filename, COLOR_RESET);
}

int main(int argc, char **argv) {
    srand((unsigned int)time(NULL));
    print_banner();

    if (argc > 1) {
        if (strcmp(argv[1], "--benchmark") == 0 || strcmp(argv[1], "-b") == 0) {
            run_paper_sweep();
            return 0;
        } else if (strcmp(argv[1], "--csv") == 0) {
            const char *out = (argc > 2) ? argv[2] : "research_results.csv";
            export_csv_benchmark(out);
            return 0;
        } else if (strcmp(argv[1], "--help") == 0 || strcmp(argv[1], "-h") == 0) {
            printf("Usage: %s [OPTIONS]\n", argv[0]);
            printf("  --benchmark, -b     Run complete evaluation sweep across all 9 hotness profiles\n");
            printf("  --csv [FILE]        Export experiment data directly to CSV format\n");
            printf("  --interactive, -i   Launch interactive live test console\n");
            return 0;
        }
    }

    /* Interactive Menu */
    while (1) {
        printf("\n%s--- RESEARCH EXPERIMENT LAB MENU ---%s\n", COLOR_BOLD, COLOR_RESET);
        printf("1. Run Baseline Test (40/60 Warm Zone - Paper Highlight with 11%% Peak Improvement)\n");
        printf("2. Run 20/80 Pareto/Zipfian Workload\n");
        printf("3. Run Full Hotness Sweep (All 9 Profiles)\n");
        printf("4. Export Results to CSV (`research_results.csv`)\n");
        printf("5. Custom Parameters (Set Trace Length, Frames, Pages)\n");
        printf("6. Exit Research Lab\n");
        printf("Select choice [1-6]: ");

        int choice = 1;
        if (scanf("%d", &choice) != 1) {
            break;
        }

        if (choice == 1) {
            run_comparison_experiment(DEFAULT_TRACE_LEN, DEFAULT_FRAMES, DEFAULT_VIRTUAL_PAGES, 7);
        } else if (choice == 2) {
            run_comparison_experiment(DEFAULT_TRACE_LEN, DEFAULT_FRAMES, DEFAULT_VIRTUAL_PAGES, 3);
        } else if (choice == 3) {
            run_paper_sweep();
        } else if (choice == 4) {
            export_csv_benchmark("research_results.csv");
        } else if (choice == 5) {
            int t_len, f_cnt, p_cnt, h_sel;
            printf("Enter trace length (e.g., 50000): ");
            scanf("%d", &t_len);
            printf("Enter number of physical frames (e.g., 10): ");
            scanf("%d", &f_cnt);
            printf("Enter number of virtual pages (e.g., 20): ");
            scanf("%d", &p_cnt);
            printf("Select hotness [0=Uniform, 1=10/90, 2=15/85, 3=20/80, 4=25/75, 5=30/70, 6=35/65, 7=40/60, 8=45/55]: ");
            scanf("%d", &h_sel);
            if (h_sel >= 0 && h_sel < NUM_HOTNESS_PROFILES) {
                run_comparison_experiment(t_len, f_cnt, p_cnt, h_sel);
            }
        } else {
            printf("Exiting Research Lab. Goodbye!\n");
            break;
        }
    }

    return 0;
}
