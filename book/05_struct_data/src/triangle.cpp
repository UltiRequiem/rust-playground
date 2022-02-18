#include <iostream>

using std::cout;
using std::endl;

class Rectangle {
public:
  Rectangle(int width, int height) {
    this->width = width;
    this->height = height;
  }

  int width;
  int height;

  int area() { return width * height; }
};

int main() {
  auto r = Rectangle(3, 5);

  cout << "The area of the rectangle is " << r.area() << "." << endl;

  return 0;
}
