#include <iostream>

struct Rectangle {
  int width;
  int height;
  int area() { return width * height; }
};

int main() {
  Rectangle rect = {50, 60};

  std::cout << "The area of the rectangle is " << rect.area()
            << " square pixels." << std::endl;

  return 0;
}
