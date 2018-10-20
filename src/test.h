#pragma once

#include <cstdint>

struct nsStaticAtom {
  uint32_t mSecret;
};

struct nsGkAtoms {
  const nsStaticAtom mAtoms[100];
};

extern const nsGkAtoms gAtoms;
