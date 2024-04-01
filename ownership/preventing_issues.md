## Preventing Issues

- Ownership prevents memory safety issues:
  - Dangling pointers
  - Double-free
    - Trying to free memory that has already been freed
  - Memory leaks
    - Not freeing memory that should have been freed
