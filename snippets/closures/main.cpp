#include <functional>
#include <iostream>

// Incredible easy considering this is C++
auto get_multiplier(int factor) {
  return [factor](int x) { return x * factor; };
}

int main() {
  auto multiplier = get_multiplier(2);

  for (int i = 1; i < 8; i++) {
    std::cout << multiplier(i) << std::endl;
  }

  return 0;
}
