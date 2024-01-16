/*
 * From suggestions of Herb Sutter:
 * https://herbsutter.com/gotw/_100/
 * https://herbsutter.com/gotw/_101/
 */
#include "camera.hpp"

int main() {
  Camera::Config config = {.name{"camera"}};
  Camera cam(config);
}
