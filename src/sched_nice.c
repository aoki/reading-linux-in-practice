#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
// unused
// #include <string.h>
#include <err.h>
// need in macos
#include <signal.h>

#define NLOOP_FOR_ESTIMATION 1000000000UL
#define NSECS_PER_MSEC 1000000UL
#define NSECS_PER_SEC 1000000000UL

static inline void load(unsigned long nloop)
{
  unsigned long i;
  for (i = 0; i < nloop; i++)
    ;
}

static inline long diff_nsec(struct timespec before, struct timespec after)
{
  return ((after.tv_sec * NSECS_PER_SEC + after.tv_nsec) - (before.tv_sec * NSECS_PER_SEC + before.tv_nsec));
}

static void child_fn(int id, struct timespec *buf, int nrecord, unsigned long nloop_per_resol, struct timespec start)
{
  int i;
  for (i = 0; i < nrecord; i++)
  {
    struct timespec ts;
    load(nloop_per_resol);
    clock_gettime(CLOCK_MONOTONIC, &ts);
    buf[i] = ts;
  }
  for (i = 0; i < nrecord; i++)
  {
    printf("%d\t%ld\t%d\n", id, diff_nsec(start, buf[i]) / NSECS_PER_MSEC, (i + 1) * 100 / nrecord);
  }
  exit(EXIT_SUCCESS);
}

static unsigned long loops_per_msec()
{
  struct timespec before, after;
  // @see https://qiita.com/ozaki-r/items/fb4a48c2833e4b479ae1
  clock_gettime(CLOCK_MONOTONIC, &before);

  unsigned long i;
  for (i = 0; i < NLOOP_FOR_ESTIMATION; i++)
    ;
  clock_gettime(CLOCK_MONOTONIC, &after);

  int ret;
  return NLOOP_FOR_ESTIMATION * NSECS_PER_MSEC / diff_nsec(before, after);
}

// helper function to validate args
static inline void arg_validation(int arg, char *argname)
{
  if (arg < 1)
  {
    fprintf(stderr, "<%s>(%d) should be >= 1\n", argname, arg);
    exit(EXIT_FAILURE);
  }
}

static pid_t *pids;

int main(int argc, char *argv[])
{
  int ret = EXIT_FAILURE;

  if (argc < 3)
  {
    fprintf(stderr, "usage: %s <total[ms]> <resolution[ms]>\n", argv[0]);
    exit(EXIT_FAILURE);
  }

  int nproc = 2;
  int total = atoi(argv[1]);
  int resol = atoi(argv[2]);

  arg_validation(total, "total");
  arg_validation(resol, "resol");

  if (total % resol)
  {
    fprintf(stderr, "<total>(%d) should be multiple of <resolution>(%d)\n", total, resol);
    exit(EXIT_FAILURE);
  }
  int nrecord = total / resol;

  struct timespec *logbuf = malloc(nrecord * sizeof(struct timespec));
  if (!logbuf)
  {
    err(EXIT_FAILURE, "malloc(logbuf) failed");
  }

  // puts("estimating workload which takes just one millisecond");

  unsigned long nloop_per_resol = loops_per_msec() * resol;

  // puts("end estimation");
  fflush(stdout);

  pids = malloc(nproc * sizeof(pid_t));
  if (pids == NULL)
  {
    warn("malloc(pids) failed");
    goto free_logbuf;
  }

  struct timespec start;
  clock_gettime(CLOCK_MONOTONIC, &start);

  int i, ncreated;
  for (i = 0, ncreated = 0; i < nproc; i++, ncreated++)
  {
    pids[i] = fork();
    if (pids[i] < 0)
    {
      goto wait_children;
    }
    else if (pids[i] == 0)
    {
      // children
      if (i == 1)
      {
        nice(5);
      }
      child_fn(i, logbuf, nrecord, nloop_per_resol, start);
      /* shouldn't reach here */
    }
  }
  ret = EXIT_SUCCESS;

// parent
wait_children:
  if (ret == EXIT_FAILURE)
  {
    for (i = 0; i < ncreated; i++)
    {
      if (kill(pids[i], SIGINT) < 0)
      {
        warn("kill(%d) failed", pids[i]);
      }
    }
  }

  for (i = 0; i < ncreated; i++)
  {
    if (wait(NULL) < 0)
    {
      warn("wait() failed.");
    }
  }

free_logbuf:
  free(logbuf);
  exit(ret);
}
