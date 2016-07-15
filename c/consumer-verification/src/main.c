#include <config.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dlfcn.h>
#include <curl/curl.h>

char *append_filename(char *executable, char *filename) {
  int executable_len = strlen(executable);
  int filename_len = strlen(filename);
  int package_len = strlen(PACKAGE_NAME);
  int diff = executable_len - package_len;
  char *string = malloc(diff + filename_len + 1);
  memcpy(string, executable, diff);
  memcpy(string + diff, filename, filename_len);
  string[diff + filename_len] = 0;
  return string;
}

char *slurp_file(char *filename) {
  FILE *fp = fopen(filename, "rb");
  if (fp) {
    fseek(fp, 0, SEEK_END);
    long fsize = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    char *string = malloc(fsize + 1);
    int read = fread(string, fsize, 1, fp);
    string[fsize] = 0;
    fclose(fp);
    return string;
  } else {
    printf("Failed to read %s\n", filename);
    return 0;
  }
}

/*
Definitions of the exported functions from the pact mock server library
*/
typedef int32_t (*lib_create_mock_server)(char *, int32_t);
typedef int32_t (*lib_mock_server_matched)(int32_t);
typedef int32_t (*lib_cleanup_mock_server)(int32_t);
typedef char* (*lib_mock_server_mismatches)(int32_t);

lib_create_mock_server create_mock_server;
lib_mock_server_matched mock_server_matched;
lib_cleanup_mock_server cleanup_mock_server;
lib_mock_server_mismatches mock_server_mismatches;

/* Loads the mock server shared library and sets up the functions we need to call */
int setup_mock_server_functions(char *mock_server_lib) {
  /* Get a handle to the pact mock server library*/
  void *handle = dlopen(mock_server_lib, RTLD_NOW | RTLD_GLOBAL);
  if (handle) {
    /* We have a handle, so lookup the functions we need */
    create_mock_server = dlsym(handle, "create_mock_server");
    mock_server_matched = dlsym(handle, "mock_server_matched");
    cleanup_mock_server = dlsym(handle, "cleanup_mock_server");
    mock_server_mismatches = dlsym(handle, "mock_server_mismatches");
    return create_mock_server != 0 && mock_server_matched != 0 && cleanup_mock_server != 0 &&
      mock_server_mismatches != 0;
  } else {
    printf("Failed to open shared library %s\n", dlerror());
    return 0;
  }
}

/* Execute the basic test against the provider server */
void execute_basic_test(int port) {
  CURL *curl = curl_easy_init();
  if (curl) {
    char url[64];
    sprintf(url, "http://localhost:%d/mallory?name=ron&status=good", port);
    printf("Executing request against %s\n", url);
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L);
    CURLcode res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
      puts("\nRequest failed");
    }
    puts("\n");
    curl_easy_cleanup(curl);
  } else {
    puts("Could not initialise the curl library.");
  }
}

/*
  Run a basic test sing the simple_pact.json file
*/
void basic_test(char *executable) {
  /* Load the pact file into memory */
  char *pactfile = append_filename(executable, "simple_pact.json");
  char *pact = slurp_file(pactfile);
  if (pact) {
    /* Create the mock server from the pact file. The mock server port will be returned */
    int port = create_mock_server(pact, 0);
    printf("Mock server started on port %d\n", port);

    /* Now we execute out test against the mock server */
    execute_basic_test(port);

    /* Check the result */
    if (mock_server_matched(port)) {
      puts("OK: Mock server verified all requests, as expected");
    } else {
      puts("FAILED: Mock server did not match all requests!!");
    }

    /* Lastly, we need to shutdown and cleanup the mock server */
    cleanup_mock_server(port);

    free(pact);
  } else {
    printf("Failed to read %s\n", pactfile);
  }
  free(pactfile);
}

size_t error_test_read_callback(char *buffer, size_t size, size_t nitems, void *instream) {
  printf("error_test_read_callback: %zd, %zd\n", size, nitems);
  sprintf(buffer, "{}\n");
  return 3;
}

/* Execute the error test against the provider server, where we expect validations to fail */
void execute_error_test(int port) {
  CURL *curl = curl_easy_init();
  if (curl) {
    char url[32];
    sprintf(url, "http://localhost:%d/?test=hi", port);
    printf("Executing request against %s\n", url);
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L);
    curl_easy_setopt(curl, CURLOPT_UPLOAD, 1L);
    curl_easy_setopt(curl, CURLOPT_INFILESIZE, 3L);

    struct curl_slist *list = NULL;
    list = curl_slist_append(list, "Content-Type: application/json");
    list = curl_slist_append(list, "Expect:");
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, list);

    curl_easy_setopt(curl, CURLOPT_READFUNCTION, error_test_read_callback);

    CURLcode res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
      printf("\nRequest failed: %d - %s\n", res,  curl_easy_strerror(res));
    }
    puts("\n");
    curl_easy_cleanup(curl);
  } else {
    puts("Could not initialise the curl library.");
  }
}

/*
  Run a error test sing the test_pact_with_bodies.json file. This test is expected to have some verification errors.
*/
void error_test(char *executable) {
  /* Load the pact file into memory */
  char *pactfile = append_filename(executable, "test_pact_with_bodies.json");
  char *pact = slurp_file(pactfile);
  if (pact) {
    /* Create the mock server from the pact file. The mock server port will be returned */
    int port = create_mock_server(pact, 0);
    printf("Mock server started on port %d\n", port);

    /* Now we execute out test against the mock server */
    execute_error_test(port);

    /* Check the result */
    if (mock_server_matched(port)) {
      puts("FAILED: Mock server verified all requests!!");
    } else {
      puts("OK: Mock server did not match all requests.");
      char *mismatch_json = mock_server_mismatches(port);
      puts(mismatch_json);
    }

    /* Lastly, we need to shutdown and cleanup the mock server */
    cleanup_mock_server(port);
    free(pact);
  } else {
    printf("Failed to read %s\n", pactfile);
  }
  free(pactfile);
}

int main (int argc, char **argv) {
  puts("This is " PACKAGE_STRING ".");

  if (argc < 3 || (strcmp(argv[1], "basic") != 0 && strcmp(argv[1], "error") != 0 && strcmp(argv[1], "v2") != 0)) {
    puts("You need to specify the test to run: basic, error, v2 and the path to the rust DLL");
    return 1;
  }

  if (!setup_mock_server_functions(argv[2])) {
    puts("Failed to setup the mock server library functions, exiting.\n");
    return 1;
  }

  curl_global_init(CURL_GLOBAL_ALL);

  if (strcmp(argv[1], "basic") == 0) {
    puts("Running basic pact test");
    basic_test(argv[0]);
  } else if (strcmp(argv[1], "error") == 0) {
    puts("Running error pact test");
    error_test(argv[0]);
  } else {
    puts("Hmm, I'm sure I validated all the inputs, so how did you get here?");
  }

  return 0;
}
