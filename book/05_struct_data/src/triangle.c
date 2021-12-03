#include <stdio.h>

struct Rectangle {
  int width;
  int height;
};

int rect_area(struct Rectangle r) { return r.width * r.height; }

int main() {
  struct Rectangle rect = {50, 60};

  printf("The area of the rectangle is %d square pixels.", rect_area(rect));

  return 0;
}
