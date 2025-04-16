// #include "common.h"
// #include "common_threads.h"
#include <assert.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>

// take pointer to function and execute
void *mythread(void *fn) {
  fn;
  return NULL;
}

// take pointer to string and print it
void *print(void *str) {
  printf("%s\n", (char *)str);
  return NULL;
}

int main(int argc, char *argv[]) {
  pthread_t p1, p2;
  int rc;
  printf("main: begin\n");
  pthread_create(&p1, NULL, mythread, print("A"));
  pthread_create(&p2, NULL, mythread, print("B"));
  // join waits for the threads to finish
  pthread_join(p1, NULL);
  pthread_join(p2, NULL);
  printf("main: end\n");
  return 0;
}
