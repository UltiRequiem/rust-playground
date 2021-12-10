#include <functional>
#include <iostream>

// Incredible easy considering this is C++
auto get_multiplier(int factor) {
  return [factor](int x) { return x * factor; };
}

int main() {
  auto multiplier = get_multiplier(2);

  std::cout << multiplier(3) << std::endl;

  return 0;
}
