// C++ Is kinda cool
#include <cstdio>

struct Rectangle {
  int width;
  int height;

  // I'm using cpp instead of c bc methods inside structs don't work on c
  int area() { return width * height; }
};

int main() {
  Rectangle rect = {50, 60};

  printf("The area of the rectangle is %d square pixels.\n", rect.area());

  return 0;
}
