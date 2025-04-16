// #include "common.h"
// #include "common_threads.h"
#include <assert.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>

// volitile int
static volatile int counter = 0;

// take pointer to function and execute
// no point to do this, just for practice
//      with function passing
void *mythread(void *str) {
  printf("%s\n", (char *)str);
  int i;
  for (i = 0; i < 1e7; i++) {
    counter = counter + 1;
  }
  printf("%s done\n", (char *)str);
  return NULL;
}

// take pointer to string and print it
// a race condition is ullustrated by different
//      results based on timing of code execution
// a critical section is a piece of code that
//      accesses a shared variable (a shared resource)
// mutual exclusion guarantees that if one thread is
//      executing within the critical section
//      the others will be prevented from doing so
int main(int argc, char *argv[]) {
  pthread_t p1, p2;
  int rc;
  printf("main: begin (counter = %d)\n", counter);
  // second arg of pthread_create is attr
  //      some examples of attributes include
  //      stack size or scheduling priority
  // fourth arg is the params to the function
  //      that is the third param
  //      it can be passed as a ref to an array
  //      &args
  pthread_create(&p1, NULL, mythread, "A");
  pthread_create(&p2, NULL, mythread, "B");
  // join waits for the threads to finish
  pthread_join(p1, NULL);
  pthread_join(p2, NULL);
  printf("main: done with both (counter = %d)\n", counter);
  return 0;
}
