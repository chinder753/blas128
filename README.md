🔴
🟡
🔵
🟢

---

# Complex

| Operation | SIMD |
| :--: | :--: |
| Add | 🟢 |
| Sub | 🟢 |
| Div | 🟢 |
| Mul | 🟢 |

---

# RealVector

| Operation | SIMD |
| :--: | :--: |
| Add | 🟢 |
| Sub | 🟢 |
| Div | 🟢 |
| Mul | 🟢 |

---

# ComplexVector

| Operation | SIMD |
| :--: | :--: |
| Add | 🟢 |
| Sub | 🟢 |
| Div | 🔵 |
| Mul | 🔵 |

---

# Matrix

| Operation | Storage Schemes | SIMD |
| :--: | :--: | :--: |
| Add | full | 🔵 |
| Sub | full | 🔵 |
| Div | full | 🔵 |
| Mul | full | 🔵 |
| Add | packed | 🔴 |
| Sub | packed | 🔴 |
| Div | packed | 🔴 |
| Mul | packed | 🔴 |

---

# leve1

| Routine or Function Group | Data Types | SIMD |
| :--: | :--: | :--: |
| cblas_?asum | s, d, sc, dz | 🔵 |
| cblas_?axpy | s, d, c, z | 🔵 |
| cblas_?copy | s, d, c, z | 🔵 |
| cblas_?dot | s, d | 🔵 |
| cblas_?sdot | | 🔴 |
| cblas_?dotc | c, z | 🔵 |
| cblas_?dotu | c, z | 🔵 |
| cblas_?nrm2 | s, d, sc, dz | 🔵 |
| cblas_?rot | s, d, c, z | 🟡 |
| cblas_?rotg | s, d, c, z | 🔵 |
| cblas_?rotm | s, d | 🔵 |
| cblas_?rotmg | | 🔴 |
| cblas_?scal | s, d, c, z | 🟡 |
| cblas_?swap | s, d, c, z | 🔵 |
| cblas_i?amax | s, d, c, z | 🔵 |
| cblas_i?amin | s, d, c, z | 🔵 |
| cblas_?cabsl | | 🔴 |

---

# level2

| Routine or Function Group | Data Types | SIMD |
| :--: | :--: | :--: |
| cblas_?gbmv | | 🔴 |
| cblas_?gemv | | 🔴 |
| cblas_?ger | | 🔴 |
| cblas_?gerc | | 🔴 |
| cblas_?geru | | 🔴 |
| cblas_?hbmv | | 🔴 |
| cblas_?hemv | | 🔴 |
| cblas_?her | | 🔴 |
| cblas_?her2 | | 🔴 |
| cblas_?hpmv | | 🔴 |
| cblas_?hpr | | 🔴 |
| cblas_?hpr2 | | 🔴 |
| cblas_?sbmv | | 🔴 |
| cblas_?spmv | | 🔴 |
| cblas_?spr | | 🔴 |
| cblas_?spr2 | | 🔴 |
| cblas_?symv | | 🔴 |
| cblas_?syr | | 🔴 |
| cblas_?sry2 | | 🔴 |
| cblas_?tbmv | | 🔴 |
| cblas_?tbsv | | 🔴 |
| cblas_?tpmv | | 🔴 |
| cblas_?tbsv | | 🔴 |
| cblas_?tpmv | | 🔴 |
| cblas_?tpsv | | 🔴 |
| cblas_?trmv | | 🔴 |
| cblas_?trsv | | 🔴 |

---

# level3

| Routine or Function Group | Data Types | SIMD |
| :--: | :--: | :--: |
| cblas_?gemm | | 🔴 |
| cblas_?hemm | | 🔴 |
| cblas_?herk | | 🔴 |
| cblas_?her2k | | 🔴 |
| cblas_?symm | | 🔴 |
| cblas_?syrk | | 🔴 |
| cblas_?syr2k | | 🔴 |
| cblas_?trmm | | 🔴 |
| cblas_?trsm | | 🔴 |
